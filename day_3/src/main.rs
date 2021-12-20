use std::fs;

fn calculate(lines: Vec<&str>, position: u8, search_for_most_common: bool) -> &str {
    if lines.len() == 1 {
        return lines.iter().next().unwrap();
    }

    let mut zeroes: u32 = 0;
    let mut ones: u32 = 0;
    let mut zero_lines: Vec<&str> = vec![];
    let mut one_lines: Vec<&str> = vec![];

    for line in lines {
        match line.chars().nth(position.into()).unwrap() {
            '0' => {
                zeroes += 1;
                zero_lines.push(line);
            }
            '1' => {
                ones += 1;
                one_lines.push(line);
            }
            _ => panic!("Invalid input bit"),
        }
    }

    let comparator = if search_for_most_common {
        |a: u32, b: u32| a > b
    } else {
        |a: u32, b: u32| a <= b
    };

    let resulting_lines = if comparator(zeroes, ones) {
        zero_lines
    } else {
        one_lines
    };

    calculate(resulting_lines, position + 1, search_for_most_common)
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let lines: Vec<&str> = input.lines().collect();

    let ogr = calculate(lines.clone(), 0, true);
    let csr = calculate(lines, 0, false);

    let ogr_num: isize = isize::from_str_radix(&ogr, 2).expect("Failed to parse ogr as decimal");
    let csr_num: isize = isize::from_str_radix(&csr, 2).expect("Failed to parse csr as decimal");

    println!("OGR binary: {}", ogr);
    println!("OGR: {}", ogr_num);
    println!("CSR binary: {}", csr);
    println!("CSR: {}", csr_num);
    println!("Multiplied: {}", ogr_num * csr_num);
}
