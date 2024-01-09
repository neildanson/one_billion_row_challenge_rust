use core::hash;
use std::{fs::File, io::Read, collections::HashMap};

//write a function to open a text file and return a s a single string
fn read_file(file_name : String) -> String {
    let mut file = File::open(file_name).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file");
    contents
}

#[derive(Debug)]
struct Data<'a> { 
    town : &'a str,
    measurement : f64,
}

struct Stats {
    min : f64,
    max : f64,
    total : f64,
    count : f64,
}

impl Stats {
    fn new(initial : f64) -> Stats {
        Stats { min : initial, max : initial, total : initial, count : 1.0 }
    }

    fn update(&mut self, value : f64) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.total += value;
        self.count += 1.0;
    }
}

impl std::fmt::Debug for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let average = self.total / self.count;
        write!(f, "min: {}, max: {}, average: {}", self.min, self.max, average)
    }
}

fn read_data<'a>(contents : &'a str) -> Option<(Data<'a>, &'a str)>{
    let index_semicolon = contents.find(';')?;
    let index_end = contents.find('\n')?;
    let town = &contents[..index_semicolon];
    let measurement = &contents[index_semicolon+1..index_end];
    let data = Data { town, measurement : measurement.parse().unwrap() };
    return Some((data, &contents[index_end+1..]));
}

fn read_line<'a>(contents : &'a str) -> Option<(&'a str, &'a str)> {
    let index_end = contents.find('\n')?;
    let line = &contents[..index_end];
    return Some((line, &contents[index_end+1..]));
}

fn run(contents : &str) -> HashMap<&str, Stats> {


    let decoder_thread = std::thread::spawn(move || {
        
    });


    let mut maybe_data = read_data(contents);
    let mut group = HashMap::new();
    while let Some((data, contents)) = maybe_data {
        let stats = group.entry(data.town).or_insert(Stats::new(data.measurement));
        stats.update(data.measurement);
        maybe_data = read_data(contents);
    }
    group
}

fn main() {
    let file_name = String::from("C:\\Users\\neild\\source\\repos\\measurements.txt");
    let start_time = std::time::Instant::now();
    let contents = read_file(file_name);
    
    let result = run(&contents);
    println!("{:?}", result);
    
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("{}", duration.as_millis());
    //println!("{:?}", data);
}
