use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1]; // команда
    
    if command == "ls" {
        dir_info(".")
    } 
    else if command == "cdls" {
        let dir_name = &args[2];
        dir_info(dir_name);

    }
    
    else if command == "cat" {
        let file_path = &args[2];
        let file_context = std::fs::read_to_string(file_path).expect("File not found");
        println!("{}", file_context)
    }
}


fn dir_info(path: &str) {
    let mut ctr = 0;
    for file in fs::read_dir(path).expect("Error") {
        let dir = file.expect("Error");
        ctr+=1;
        // проверка на то, что это директория
        if dir.file_type().expect("Error").is_dir() {
            println!("{}. {}/", ctr, dir.file_name().to_str().unwrap());
            continue;
        } println!("{}. {}", ctr, dir.file_name().to_str().unwrap());
   } 
}