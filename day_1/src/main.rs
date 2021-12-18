use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";

fn main() {
    let input: String = fs::read_to_string(INPUT_FILE_NAME).expect("Couldn't read input file");

    let input_numbers: Vec<u32> = input
        .lines()
        .map(|l| l.parse::<u32>().expect("Parsing input number failed"))
        .collect();

    let mut previous_number: Option<u32> = None;
    let mut increases_count = 0;

    for number in input_numbers {
        if let Some(prev_num) = previous_number {
            if number > prev_num {
                increases_count += 1;
            }
        }

        previous_number = Some(number);
    }

    println!("Increases count: {}", increases_count);
}
