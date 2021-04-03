use regex::Regex;
use walkdir::WalkDir;

pub fn find_cleaned_fastq(path: &str, len: usize, sep: char) {
    println!("[seq]");
    WalkDir::new(path)
        .into_iter()
        .filter_map(|ok| ok.ok())
        .filter(|e| e.file_type().is_file())
        .for_each(|e| {
            let path = e.path().to_string_lossy();
            let fname = e.path().file_name().unwrap().to_string_lossy();
            if re_matches_lazy(&fname) {
                let id = construct_id(&fname, len, sep);
                println!("{}:{}", id, path);
            }
        });
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
