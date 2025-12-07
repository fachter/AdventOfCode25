use std::{cmp::max, collections::HashMap, fs};

fn main() {
    let lines = read_lines("03");

    let mut sum: u128 = 0;
    let n_batteries = 12usize;
    for line in lines {
        let mut max_values: HashMap<u32, u128> = HashMap::new();
        for _ in 0..n_batteries {
            max_values.insert(0, 0 as u128);
        }
        println!("{}", line);

        let chars: Vec<char> = line.chars().collect();
        let char_count = chars.len();
        for (i, char_value) in (&chars).iter().enumerate() {
            let number = char_value.to_digit(10).unwrap() as u128;
            update_max_values(
                &mut max_values,
                number,
                n_batteries,
                max(0, (n_batteries as i32) - ((char_count as i32) - (i as i32))) as u32,
            );
        }

        // println!("Max Values: {:?}", max_values);
        let mut value: u128 = 0;
        for (key, val) in &max_values {
            let power_value = (n_batteries as u32 - 1) - key;
            value += val * 10u128.pow(power_value);
        }
        // println!("Value: {}", value);

        // println!("Combined Number: {}{}", largest, second_largest);
        // let value = largest * 10 + second_largest;
        sum += value;
    }

    println!("Sum: {}", sum);
}

fn read_lines(file_name: &str) -> Vec<String> {
    let file_name = format!("data/{}.txt", file_name);
    let file_content = fs::read_to_string(file_name).expect("Could not read file!!");
    file_content
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn update_max_values(
    max_values: &mut HashMap<u32, u128>,
    number: u128,
    n_batteries: usize,
    index: u32,
) {
    if index >= (n_batteries as u32) {
        return;
    }
    let current = max_values.get(&index).copied().unwrap_or(0);
    if number > current {
        max_values.insert(index, number);
        for i in (index + 1)..(n_batteries as u32) {
            max_values.insert(i, 0);
        }
    } else {
        update_max_values(max_values, number, n_batteries, index + 1);
    }
}
