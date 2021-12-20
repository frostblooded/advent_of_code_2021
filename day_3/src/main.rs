use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let mut bits_count: [(u32, u32); 12] = [(0, 0); 12];

    for line in input.lines() {
        for (i, b) in line.chars().enumerate() {
            match b {
                '0' => bits_count[i].0 += 1,
                '1' => bits_count[i].1 += 1,
                _ => panic!("Invalid bit in input"),
            };
        }
    }

    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();

    for counts in bits_count {
        if counts.0 > counts.1 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    let gamma_num: isize =
        isize::from_str_radix(&gamma, 2).expect("Failed to parse gamma as decimal");
    let epsilon_num: isize =
        isize::from_str_radix(&epsilon, 2).expect("Failed to parse epsilon as decimal");

    println!("Gamma: {}", gamma_num);
    println!("Epsilon: {}", epsilon_num);
    println!("Multiplied: {}", gamma_num * epsilon_num);
}
