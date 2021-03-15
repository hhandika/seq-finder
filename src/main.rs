mod finder;

use std::path::{Path};

fn main() {
    let files = Path::new("raw_reads/test_files/bunomys_chrysocomus_ABC123_read1.fastq.gz");
    let dir = files.parent().unwrap();
    let fnames = files.file_name().unwrap().to_string_lossy();

    let seq = finder::construct_names(&fnames, 3);
    println!("[samples]");
    println!("{}:{}/", seq, dir.to_string_lossy());

}
