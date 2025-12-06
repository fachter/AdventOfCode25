use std::fs;

fn main() {
    let lines = read_lines("03");

    let mut sum = 0;
    for line in lines {
        let mut largest = 0;
        let mut second_largest = 0;
        println!("{}", line);

        let chars: Vec<char> = line.chars().collect();
        for char_value in &chars[..chars.len() - 1] {
            let number = char_value.to_digit(10).unwrap();
            if number > largest {
                largest = number;
                second_largest = 0;
            } else if number > second_largest {
                second_largest = number;
            }
        }
        let last_char = chars[chars.len() - 1].to_digit(10).unwrap();
        if last_char > second_largest {
            second_largest = last_char;
        }

        println!("Combined Number: {}{}", largest, second_largest);
        let value = largest * 10 + second_largest;
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
