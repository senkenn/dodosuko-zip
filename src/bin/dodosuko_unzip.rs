use std::{
    fs::File,
    io::{Read, Write},
};

use regex::Regex;

fn main() {
    let src_file = "data_files/dodosuko.txt";
    let dist_file = "data_files/dodosuko.run_length_zip";

    // read contents
    let mut f = File::open(dist_file).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    // run-length decoding
    let mut src = "".to_string();
    let count_re = Regex::new(r"\d").unwrap();
    for (i, char) in contents.chars().enumerate() {
        if let Some(caps) = count_re.captures(&char.to_string()) {
            let chunk: String = contents.chars().skip(i + 1).take(i + 8).collect();
            let chunk_num: u8 = caps[0].parse().unwrap();
            for _ in 0..chunk_num {
                src.push_str(&chunk);
            }
        }
    }

    // write to file
    let mut buf = File::create(src_file).expect("cannot create file");
    buf.write(src.as_bytes()).expect("cannot write to file");
}
