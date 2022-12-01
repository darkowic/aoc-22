use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    let contents =
        fs::read_to_string(INPUT_FILE_PATH).expect("Should have been able to read the file");

    let part1_result = contents
        .split("\n\n")
        .map(|item| {
            item.split("\n")
                .filter_map(|el| el.parse::<isize>().ok())
                .sum::<isize>()
        })
        .max()
        .unwrap();

    println!("Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?.\nAnswer: {part1_result}");

    let mut data = contents
        .split("\n\n")
        .map(|item| {
            item.split("\n")
                .filter_map(|el| el.parse::<isize>().ok())
                .sum::<isize>()
        })
        .collect::<Vec<isize>>();
        // is there syntax like this? Or can lambda be used here?
        // .split_at(<self.len()> - 3)
    
    data.sort();

    
    let part2_result = data.split_at(data.len() - 3).1.iter().sum::<isize>();

    println!("Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?.\nAnswer: {part2_result}");
}
