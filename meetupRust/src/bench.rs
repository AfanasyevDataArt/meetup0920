use std::time::Instant;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::cmp::Ordering;

const SIZE : usize = 128;

struct LineBuf{
    buf: [u8; SIZE]
}

fn lines_from_file(filename: impl AsRef<Path>) -> (Vec<String>, Vec<LineBuf>) {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let lines :Vec<String> =buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut str_vec = vec![];
    let mut buf_vec = vec![];

    for line in lines.iter(){
        for idx in 0..(line.len() / SIZE){
            let sub_line = &line[idx*SIZE .. (idx+1)*SIZE].to_string();

            str_vec.push(sub_line.clone());

        }
    }

    (str_vec, buf_vec)
}



fn main() {
    let ( mut lines_str, mut lines_buf) = lines_from_file("/home/alexey/tmp/data.bin");

    println!("Vec<String> Read: {} lines", lines_str.len());
    let now = Instant::now();
    lines_str.sort();
    let elapsed = now.elapsed().as_millis();
    println!("Sorted: {} ms", elapsed);

    println!("Vec<u8[{}]> Read: {} lines", SIZE, lines_buf.len());
    let now_buf = Instant::now();
//    lines_buf.sort();
    let elapsed_buf = now_buf.elapsed().as_millis();
    println!("Sorted: {} ms", elapsed_buf);

}