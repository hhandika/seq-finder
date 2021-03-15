pub fn construct_names(names: &str, len: usize) -> String {
    let words: Vec<&str> = names.split("_").collect();

    let mut seqname = String::new();

    words[0..(len-1)].iter()
        .for_each(|w| {
            let comp = format!("{}_", w);
            seqname.push_str(&comp);
        });
       
    seqname.push_str(words[len-1]);

    seqname
    
}