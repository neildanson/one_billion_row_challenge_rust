use std::{collections::{BTreeMap, HashMap}, fs::File, str, hash::RandomState};

use memmap2::{Mmap, MmapOptions};
use rayon::prelude::*;

//write a function to open a text file and return a s a single string
fn read_file(file_name: String) -> Option<Mmap> {
    let file = File::open(file_name).expect("Could not open file");
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

    fn merge(&mut self, other: &Stats) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.total += other.total;
        self.count += other.count;
    }
}

impl std::fmt::Debug for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let average = self.total / self.count;
        write!(
            f,
            "min: {}, max: {}, average: {:.1}",
            self.min, self.max, average
        )
    }
}

fn read_data(contents: &str) -> Option<Data> {
    let (town, measurement) = contents.split_once(';')?;
    let measurement = measurement.parse().ok()?;
    let data = Data {
        town,
        measurement,
    };
    Some(data)
}

fn read_all(bytes: &Mmap) -> BTreeMap<&str, Stats> {

    let result = bytes
        .par_split(|&b| b == b'\n')
        .filter_map(|line| str::from_utf8(line).ok())
        .filter_map(|str| read_data(str))
        .fold(|| HashMap::with_hasher(RandomState::default()), |mut map, data| {
            let stats = map
                .entry(data.town)
                .or_insert_with(|| Stats::new(data.measurement));
            stats.update(data.measurement);
            map
        })
        .reduce(
            || HashMap::with_hasher(RandomState::default()),
            |mut map1, map2| {
                for (key, value) in map2 {
                    let stats = map1.entry(key).or_insert_with(|| Stats::new(value.min));
                    stats.merge(&value);
                }
                map1
            },
        )
        .into_iter()
        .collect::<BTreeMap<_, _>>();
        result
}
fn main() {
    let file_name = String::from("..\\measurements.txt");

    let start_time = std::time::Instant::now();
    let contents = read_file(file_name).unwrap();
    let result = read_all(&contents);
    let result = result.iter().take(10).collect::<Vec<_>>();
    let end_time = std::time::Instant::now();
    println!("{:?}", result);

    let duration = end_time.duration_since(start_time);
    println!("{}", duration.as_millis());
}
