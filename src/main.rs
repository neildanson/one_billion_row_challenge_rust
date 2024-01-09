use core::hash;
use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    io::Read,
    sync::{mpsc::channel, Arc},
};

//write a function to open a text file and return a s a single string
fn read_file(file_name: String) -> String {
    let mut file = File::open(file_name).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");
    contents
}

#[derive(Debug)]
struct Data {
    town: Arc<str>,
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

fn read_data(contents: Arc<str>) -> Option<Data> {
    let index_semicolon = contents.find(';')?;
    let town = &contents[..index_semicolon];
    let measurement = &contents[index_semicolon + 1..];
    let data = Data {
        town: Arc::from(town),
        measurement: measurement.parse().unwrap(),
    };
    return Some(data);
}

fn read_line<'a>(contents: &'a str) -> Option<(&'a str, &'a str)> {
    let index_end = contents.find('\n')?;
    let line = &contents[..index_end];
    return Some((line, &contents[index_end + 1..]));
}

fn run(contents: &str) -> HashMap<Arc<str>, Stats> {
    let (dtx, drx) = channel::<Arc<str>>();
    let (rtx, rrx) = channel::<Data>();
    let (tx, rx) = channel::<HashMap<Arc<str>,Stats>>();
    let results_thread = std::thread::spawn(move || {
        let mut town_stats = HashMap::new();
        while let Ok(data) = rrx.recv() {
            let stats = town_stats
                .entry(data.town)
                .or_insert(Stats::new(data.measurement));
            stats.update(data.measurement);
        }
        tx.send(town_stats).unwrap();
    });

    let decoder_thread = std::thread::spawn(move || {
        while let Ok(line) = drx.recv() {
            let data = read_data(line.clone());
            match data {
                Some(data) => rtx.send(data).unwrap(),
                None => println!("Error"),
            };
        }
    });

    let mut maybe_data = read_line(contents);
    while let Some((line, rest)) = maybe_data {
        dtx.send(Arc::from(line)).unwrap();
        maybe_data = read_line(rest);
    }
    //decoder_thread.join().unwrap();
    //results_thread.join().unwrap();
    rx.recv().unwrap()
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
