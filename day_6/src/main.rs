use std::fs;

const MAX_AGE: u128 = 8;
const DAYS: u128 = 256;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let input_ages: Vec<u8> = input
        .split(',')
        .map(|n| {
            n.chars()
                .next()
                .unwrap()
                .to_string()
                .parse::<u8>()
                .expect("Couldn't parse input number")
        })
        .collect();

    let mut ages: Vec<u128> = vec![0; MAX_AGE as usize + 1];

    for age in input_ages {
        ages[age as usize] += 1;
    }

    for _ in 1..=DAYS {
        let mut new_ages = vec![0; MAX_AGE as usize + 1];

        for j in 0..=(MAX_AGE - 1) {
            new_ages[j as usize] = ages[j as usize + 1];
        }

        new_ages[MAX_AGE as usize] = ages[0];
        new_ages[6] += ages[0];

        ages = new_ages;
    }

    let mut res: u128 = 0;

    for age in ages {
        res += age;
    }

    println!("Result: {:?}", res);
}
