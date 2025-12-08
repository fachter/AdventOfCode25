use std::fs;

pub fn read_lines(file_name: &str) -> Vec<String> {
    let file_name = format!("data/{}.txt", file_name);
    let file_content = fs::read_to_string(file_name).expect("Could not read file!!");
    file_content
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

// pub fn read_file(file_name: &str) -> String {
//     let file_name = format!("data/{}.txt", file_name);
//     fs::read_to_string(file_name).expect("Could not read file!!")
