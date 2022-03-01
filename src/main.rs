use std::io::Write;
use std::{collections::HashSet, fs::File, time::Instant};

mod model;

use model::SteuerId;

fn main() {
    write_to_file(10_000, "ten-thousand.txt").unwrap();
    write_to_file(10_000_000, "ten-million.txt").unwrap();
}

fn write_to_file(n: usize, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let mut hash_set = HashSet::with_capacity(n);

    println!("Creating {} Steuer-IDs...", n);

    let now = Instant::now();
    while hash_set.len() < n {
        let id = SteuerId::create_random();
        if hash_set.insert(id.0) {
            writeln!(file, "{}", id)?;

            // if hash_set.len() % 10_000 == 0 {
            //     println!("{} created...", hash_set.len());
            // }
        }
    }

    println!("Done in {} milliseconds.", now.elapsed().as_millis());

    Ok(())
}
