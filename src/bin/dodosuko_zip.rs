use std::{
    fs::File,
    io::{Read, Write},
};

fn main() {
    let src_file = "data_files/dodosuko.txt";
    let dist_file = "data_files/dodosuko.run_length_zip";

    // read contents
    let mut f = File::open(src_file).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // run-length encoding
    let chunk_size = 8;
    let chunks: Vec<String> = contents
        .chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect())
        .collect();

    let reset_count = 1;
    let mut count = 0;
    let mut output = "".to_string();
    let mut previous = chunks[0].clone();
    for (i, chunk) in chunks.iter().enumerate() {
        if &previous == chunk {
            count += 1;
            continue;
        }

        output.push_str(&count.to_string());
        output.push_str(&previous);

        count = reset_count;
        previous = chunk.to_string();

        if i == chunks.len() - 1 {
            output.push_str(&count.to_string());
            output.push_str(&previous);
        }
    }

    // write to file
    let mut buf = File::create(dist_file).expect("cannot create file");
    buf.write(output.as_bytes()).expect("cannot write to file");
}
