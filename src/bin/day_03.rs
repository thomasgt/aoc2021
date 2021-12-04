use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone, Debug)]
struct BitCounter {
    counts: Vec<usize>,
}

impl BitCounter {
    fn new(n: usize) -> Self {
        BitCounter { counts: vec![0; n] }
    }

    fn accumulate(&self, x: usize) -> Self {
        let mut result = self.clone();
        result.counts.iter_mut().enumerate().for_each(|(i, c)| *c += x >> i & 1);
        result
    }

    fn convert_with_rule<F>(&self, mut f: F) -> usize
        where F: FnMut(usize) -> bool
    {
        self.counts.iter().enumerate().fold(0, |acc, (i, x)| {
            acc + ((f(*x) as usize) << i)
        })
    }
}

fn parse_report(records: &Vec<String>) -> (usize, usize, usize, usize) {
    let n_bits = records[0].len();
    let n_records = records.len();

    println!("parsing {} {}-bit records", n_records, n_bits);

    let record_values: Vec<usize> = records.iter().map(|x| usize::from_str_radix(x, 2).expect("failed to parse line")).collect();

    let counter = record_values.iter().fold(BitCounter::new(n_bits), |acc, x| acc.accumulate(*x));

    let gamma = counter.convert_with_rule(|x| x > n_records / 2);
    let epsilon = counter.convert_with_rule(|x| x < n_records / 2);

    // use data from gamma to get initial partitions
    let (o2_partition, co2_partition): (Vec<usize>, Vec<usize>) = record_values.iter().partition(|x| ((*x & gamma) >> (n_bits - 1)) != 0);
    let o2_rate = *recursive_partition_filter_by_size(&o2_partition, n_bits - 2, true).get(0).expect("failed to find o2 rating");
    let co2_rate = *recursive_partition_filter_by_size(&co2_partition, n_bits - 2, false).get(0).expect("failed to find co2 rating");

    (gamma, epsilon, o2_rate, co2_rate)
}

fn recursive_partition_filter_by_size(v: &Vec<usize>, bit: usize, keep_largest: bool) -> Vec<usize> {
    let (set_partition, clear_partition): (Vec<usize>, Vec<usize>) = v.iter().partition(|x| ((*x >> bit) & 1) != 0);
    let keep_partition = match (keep_largest, set_partition.len().cmp(&clear_partition.len())) {
        (false, Ordering::Less) => set_partition,
        (false, Ordering::Equal) => clear_partition,
        (false, Ordering::Greater) => clear_partition,
        (true, Ordering::Less) => clear_partition,
        (true, Ordering::Equal) => set_partition,
        (true, Ordering::Greater) => set_partition,
    };

    if keep_partition.len() == 1 || bit == 0 {
        keep_partition
    } else {
        recursive_partition_filter_by_size(&keep_partition, bit - 1, keep_largest)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = Path::new(args.get(1).expect("missing INPUT argument"));

    let file = File::open(input_path).expect("failed to open INPUT file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|x| x.expect("failed to read line")).collect();

    let metrics = parse_report(&lines);
    println!("gamma rate = {} | epsilon rate = {} | product = {}", metrics.0, metrics.1, metrics.0 * metrics.1);
    println!("o2 rate = {} | co2 rate = {} | product = {}", metrics.2, metrics.3, metrics.2 * metrics.3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_report() {
        let records = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        let c = parse_report(&records);
        assert_eq!(c.0, 22);
        assert_eq!(c.1, 9);
        assert_eq!(c.2, 23);
        assert_eq!(c.3, 10);
    }
}