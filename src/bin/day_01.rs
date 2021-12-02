use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = Path::new(args.get(1).expect("missing INPUT argument"));
    let window_size = args.get(2).expect("missing WINDOW argument").parse::<usize>().expect("failed to parse window size");

    let file = File::open(input_path).expect("failed to open INPUT file");
    let reader = BufReader::new(file);

    let depths: Vec<i32> = reader.lines().map(|x| x.expect("failed to read line").parse::<i32>().expect("failed to parse depth")).collect();

    let depth_windows: Vec<i32> = depths.windows(window_size).map(|x| x.into_iter().sum()).collect();

    let differences = depth_windows.windows(2).map(|x| x[1] - x[0]);

    let n_increases = differences.filter(|x| *x > 0).count();

    println!("{}", n_increases)
}
