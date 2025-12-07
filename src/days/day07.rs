use std::collections::{HashMap, HashSet};

use crate::utils::file::read_lines;

pub fn run_laboratory() {
    println!("--- Day 7: Laboratory ---");
    day07("07-test");
    day07("07");
    println!("--------------------\n\n");
}

fn day07(file_name: &str) {
    let lines = read_lines(file_name);

    let first_line = lines.first().expect("Has to have a start line");
    let start_beem = first_line.find('S').expect("Has to have a start beam");
    let mut beams: HashMap<usize, u128> = HashMap::from([(start_beem, 1)]);
    let mut counter = 0;
    for line in lines {
        let previous_beams = beams.clone();
        beams.clear();
        let chars: Vec<char> = line.chars().collect();
        let mut visited: HashSet<usize> = HashSet::new();
        for (beam, beam_count) in &previous_beams {
            match chars[*beam] {
                '^' => {
                    beams.insert(beam - 1, beams.get(&(beam - 1)).unwrap_or(&0) + beam_count);
                    beams.insert(beam + 1, beams.get(&(beam + 1)).unwrap_or(&0) + beam_count);

                    counter += 1;
                }
                _ => {
                    beams.insert(*beam, beams.get(&beam).unwrap_or(&0) + beam_count);
                }
            }
            visited.insert(*beam);
        }
    }

    let timelines: u128 = beams.values().sum();
    println!("Counter: {}", counter);
    println!("Timelines: {}", timelines);
}
