use std::{fs::File, io::Read};

#[derive(Debug, Default)]
struct PatNumIdx {
    pat: String,
    num: usize,
    idx: usize,
}

#[derive(Debug, Default)]
struct Compressed {
    inner: Vec<PatNumIdx>,
}

impl Compressed {
    fn last_mut(&mut self) -> &mut PatNumIdx {
        self.inner.last_mut().unwrap()
    }

    fn last_pat(&self) -> &str {
        &self.inner.last().unwrap().pat
    }

    fn last_idx(&self) -> usize {
        self.inner.last().unwrap().idx
    }

    fn add(&mut self, pat: String) {
        if self.inner.is_empty() {
            let p = PatNumIdx {
                pat: pat.clone(),
                num: 1,
                idx: 0,
            };
            self.inner.push(p);
            return;
        }

        if self.last_pat() == &pat {
            self.last_mut().num += 1;
            return;
        }

        let p = PatNumIdx {
            pat,
            idx: self.last_idx() + 1,
            num: 1,
        };

        self.inner.push(p);
    }
}

fn main() {
    let src_file = "data_files/dodosuko.txt";
    let dist_file = "data_files/dodosuko.run_length_zip";

    // read contents
    let mut f = File::open(src_file).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    // println!("With text:\n{}", contents);

    // run-length encoding
    let chunk_size = 8;
    // TODO: what is collect
    let chunks: Vec<String> = contents
        .chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect())
        .collect();

    println!("{chunks:?}");

    let mut compressed = Compressed::default();

    chunks.into_iter().for_each(|c| compressed.add(c));

    println!("{compressed:?}");
}
