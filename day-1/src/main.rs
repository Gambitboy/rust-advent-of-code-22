use std::{fs::File, io::Read};

fn read_and_sum_calories(file_name: &str) -> Vec<i32> {
    let mut file = File::open(file_name).expect("Input should be there.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("There should be content in the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    println!("Lines in file: {}", lines.len());

    let mut elf_calories: Vec<i32> = Vec::new();
    let mut elf_count: i32 = 0;

    let mut calories: i32 = 0;

    lines.iter().for_each(|line| {
        if *line == "" {
            elf_count += 1;
            elf_calories.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<i32>().expect("All lines are strings");
        }
    });

    println!("Elf count: {}", elf_count);
    println!("Unique calories sets collected: {}", elf_calories.len());

    elf_calories.clone()
}

fn main() {
    let mut elf_calories = read_and_sum_calories("input.txt");

    let most_calories = elf_calories
        .clone()
        .into_iter()
        .max()
        .expect("Vector is filled with usize values");

    println!("The most calories: {}", most_calories);

    elf_calories.sort_unstable();
    elf_calories.reverse();

    let mut index = 0;
    let mut top3 = 0;
    while index < 3 {
        top3 += elf_calories[index];
        index += 1;
    }

    println!("Top 3 calories: {}", top3);
}
