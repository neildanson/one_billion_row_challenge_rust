use std::{collections::BTreeMap, fs::File, sync::Arc};

use memmap::{Mmap, MmapOptions};
use rayon::prelude::*;

//write a function to open a text file and return a s a single string
fn read_file(file_name: String) -> Option<Mmap> {
    let mut file = File::open(file_name).expect("Could not open file");
    let options = MmapOptions::new();
    let mmap = unsafe { MmapOptions::new().map(&file).ok()? };
    Some(mmap)
}

#[derive(Debug)]
struct Data<'a> {
    town: &'a str,
    measurement: f64,
}

struct Stats {
    min: f64,
    max: f64,
    total: f64,
    count: f64,
}

impl Stats {
    fn new(initial: f64) -> Stats {
        Stats {
            min: initial,
            max: initial,
            total: 0.0,
            count: 0.0,
        }
    }

    fn update(&mut self, value: f64) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.total += value;
        self.count += 1.0;
    }
}

impl std::fmt::Debug for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let average = self.total / self.count;
        write!(
            f,
            "min: {}, max: {}, average: {}",
            self.min, self.max, average
        )
    }
}

fn read_data<'a>(contents: &'a str) -> Option<Data<'a>> {
    let index_semicolon = contents.find(';')?;
    let town = &contents[..index_semicolon];
    let measurement = &contents[index_semicolon + 1..];
    let measurement = measurement.parse().ok()?;
    let data = Data {
        town: town,
        measurement: measurement,
    };
    return Some(data);
}

fn read_line<'a>(contents: &'a str) -> Option<(&'a str, &'a str)> {
    let index_end = contents.find('\n')?;
    let line = &contents[..index_end];
    return Some((line, &contents[index_end + 1..]));
}

#[derive(Debug)]
enum Token {
    SemiColon(usize),
    Eol(usize),
}
fn read_all<'a>(contents: Mmap) -> Vec<String> {
    let data = &contents[..1_000_00];
    data
            .par_chunks(32768)
            .enumerate()
            .flat_map(|(i, bytes)| {
                bytes.par_iter().enumerate().filter_map(move |(j,c)| match *c as char {
                    '\n' => Some(Token::Eol((i+1) * j)),
                    ';' => Some(Token::SemiColon((i+1) * j)),
                    _ => None,
                })
            })
            .collect::<Box<[Token]>>()
            .par_windows(2)
            .filter_map(|chunk| match chunk {
                [Token::SemiColon(i), Token::Eol(j)] => Some((i+1,j+1)),
                [Token::Eol(i), Token::SemiColon(j)] => Some((i+1,j+1)),
                _ => None,
            })
            .map(|(i,j)| String::from_utf8(data[i..j].to_vec()).unwrap())
            .collect()
}
fn main() {
    let file_name = String::from("C:\\Users\\neild\\source\\repos\\measurements.txt");


    let start_time = std::time::Instant::now();
    let contents = read_file(file_name).unwrap();
    let result = read_all(contents);
    let result = result.iter().take(10).collect::<Vec<_>>();
    //let result = run(&contents);
    let end_time = std::time::Instant::now();
    println!("{:?}", result);

    let duration = end_time.duration_since(start_time);
    println!("{}", duration.as_millis());
}
