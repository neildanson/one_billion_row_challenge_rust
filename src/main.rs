#![feature(iter_collect_into)]
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    str,
};

use fxhash::{FxBuildHasher, FxHasher};
use memmap2::{Mmap, MmapOptions};
use rayon::prelude::*;

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
    count: i32,
}

impl Stats {
    fn new(initial: f64) -> Stats {
        Stats {
            min: initial,
            max: initial,
            total: 0.0,
            count: 0,
        }
    }

    fn update(&mut self, value: f64) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.total += value;
        self.count += 1;
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
        let average = self.total / (self.count as f64);
        write!(
            f,
            "min: {}, max: {}, average: {:.1}",
            self.min, self.max, average
        )
    }
}

fn read_data(contents: &str) -> Option<Data> {
    let (town, measurement) = contents.split_once(';')?;
    let measurement = fast_float::parse(measurement).ok()?;
    let data = Data { town, measurement };
    Some(data)
}

fn read_all(bytes: &Mmap) -> BTreeMap<&str, Stats> {
    let mut result = BTreeMap::new();
    bytes
        .par_split(|&b| b == b'\n')
        .filter_map(|line| str::from_utf8(line).ok())
        .filter_map(read_data)
        .fold(
            || HashMap::with_capacity_and_hasher(500, FxBuildHasher::default()),
            |mut map, data| {
                let stats = map
                    .entry(data.town)
                    .or_insert_with(|| Stats::new(data.measurement));
                stats.update(data.measurement);
                map
            },
        )
        .reduce(
            || HashMap::with_capacity_and_hasher(1000, FxBuildHasher::default()),
            |mut result, map2| {
                for (key, value) in map2 {
                    let stats = result.entry(key).or_insert_with(|| Stats::new(value.min));
                    stats.merge(&value);
                }
                result
            },
        )
        .into_iter()
        .collect_into(&mut result);
        result
}
fn main() {
    let file_name = String::from("..\\measurements.txt");
    let start_time = std::time::Instant::now();
    let contents = read_file(file_name).unwrap();
    let result = read_all(&contents);
    let result :Vec<_> = result.iter().collect();
    let end_time = std::time::Instant::now();
    println!("{:?}", result);

    let duration = end_time.duration_since(start_time);
    println!("{}", duration.as_millis());
}
