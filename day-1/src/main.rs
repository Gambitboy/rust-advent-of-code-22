use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("input.txt").expect("Input should be there.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("There should be content in the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    println!("Lines in file: {}", lines.len());

    let mut elf_calories: Vec<usize> = Vec::new();
    let mut elf_count: usize = 0;

    let mut calories: usize = 0;

    lines.iter().for_each(|line| {
        if *line == "" {
            elf_count += 1;
            elf_calories.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<usize>().expect("All lines are strings");
        }
    });

    println!("Elf count: {}", elf_count);
    println!("Unique calories sets collected: {}", elf_calories.len());

    let most_calories = elf_calories
        .into_iter()
        .max()
        .expect("Vector is filled with usize values");

    println!("The most calories: {}", most_calories);
}
