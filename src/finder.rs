use std::fs::File;
use std::io::{LineWriter, Write};

use regex::Regex;
use walkdir::WalkDir;

pub fn find_cleaned_fastq(path: &str, len: usize, sep: char, iscsv: bool) {
    let output = File::create(get_fnames(iscsv)).expect("FILE EXISTS.");
    let mut line = LineWriter::new(output);
    if iscsv {
        writeln!(line, "id,path").unwrap();
    } else {
        writeln!(line, "[seq]").unwrap();
    }
    WalkDir::new(path)
        .into_iter()
        .filter_map(|ok| ok.ok())
        .filter(|e| e.file_type().is_file())
        .for_each(|e| {
            let path = e.path().parent().unwrap();
            let fname = e.path().file_name().unwrap().to_string_lossy();
            if re_matches_lazy(&fname) {
                let id = construct_id(&fname, len, sep);
                let full_path = String::from(path.canonicalize().unwrap().to_string_lossy());
                if iscsv {
                    writeln!(line, "{},{}/", id, full_path).unwrap();
                } else {
                    writeln!(line, "{}:{}/", id, full_path).unwrap();
                }
            }
        });
}

fn get_fnames(iscsv: bool) -> String {
    let mut fname = String::from("seq-finder");
    if iscsv {
        fname.push_str(".csv");
    } else {
        fname.push_str(".conf");
    }

    fname
}

fn re_matches_lazy(fname: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(_|-)((?i)(read|r)1)").unwrap();
    }

    RE.is_match(fname)
}

fn construct_id(names: &str, len: usize, sep: char) -> String {
    let words: Vec<&str> = names.split(sep).collect();

    let mut seqname = String::new();

    words[0..(len - 1)].iter().for_each(|w| {
        let comp = format!("{}{}", w, sep);
        seqname.push_str(&comp);
    });

    seqname.push_str(words[len - 1]);
    seqname
}
