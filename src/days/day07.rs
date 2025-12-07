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
    let mut beams: HashSet<usize> = HashSet::from([start_beem]);
    let mut counter = 0;
    for line in lines {
        let previous_beams = beams.clone();
        beams.clear();
        let chars: Vec<char> = line.chars().collect();
        for beam in &previous_beams {
            match chars[*beam] {
                '^' => {
                    beams.insert(beam - 1);
                    beams.insert(beam + 1);
                    counter += 1;
                }
                _ => {
                    beams.insert(*beam);
                }
            }
        }
        // println!("{:?}", beams);
        // println!("{:?}", previous_beams);
        // println!("{}", line);
    }

    println!("Counter: {}", counter);
}
