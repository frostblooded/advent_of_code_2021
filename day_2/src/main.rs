use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";

fn main() {
    let input: String = fs::read_to_string(INPUT_FILE_NAME).expect("Couldn't read input file");

    let command_pairs: Vec<(char, u32)> = input
        .lines()
        .map(|l| {
            let split: Vec<&str> = l.split(" ").collect();

            (
                split
                    .get(0)
                    .expect("Line has no command")
                    .chars()
                    .next()
                    .unwrap(),
                split
                    .get(1)
                    .expect("Line has no argument number")
                    .parse::<u32>()
                    .expect("Can't parse line argument as number"),
            )
        })
        .collect();

    let mut forward_movement = 0;
    let mut aim = 0;
    let mut depth = 0;

    for (command, arg) in command_pairs {
        match command {
            'f' => {
                forward_movement += arg;
                depth += aim * arg;
            }
            'd' => aim += arg,
            'u' => aim -= arg,
            _ => panic!("Unsupported command"),
        }
    }

    println!("Result: {}", forward_movement * depth);
}
