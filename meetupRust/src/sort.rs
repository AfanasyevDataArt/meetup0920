use std::time::Instant;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let mut lines = lines_from_file("/home/alexey/tmp/data.bin");

    println!("Read: {} lines", lines.len());

    let now = Instant::now();
    lines.sort();
    let elapsed = now.elapsed().as_millis();

    println!("Sorted: {} ms", elapsed);

}