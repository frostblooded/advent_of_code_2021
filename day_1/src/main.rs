use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";

fn main() {
    let input: String = fs::read_to_string(INPUT_FILE_NAME).expect("Couldn't read input file");

    let input_numbers: Vec<u32> = input
        .lines()
        .map(|l| l.parse::<u32>().expect("Parsing input number failed"))
        .collect();

    let windows = input_numbers.windows(3);
    let mut previous_window: Option<&[u32]> = None;
    let mut increases_count = 0;

    for window in windows {
        if let Some(prev_win) = previous_window {
            if window.iter().sum::<u32>() > prev_win.iter().sum() {
                increases_count += 1;
            }
        }

        previous_window = Some(window);
    }

    println!("Increases count: {}", increases_count);
}
