use std::{cmp::max, fs};

fn max_path_sum(pyramid: &str) -> u128 {
    let last_line = pyramid.lines().rev().nth(0).unwrap().trim();
    let mut bottom_row: Vec<u128> = Vec::new();
    for num in last_line.split(" ") {
        bottom_row.push(num.parse().unwrap());
    }
    for line in pyramid.lines().rev().skip(1) {
        let mut top_row = Vec::new();
        for (index, num) in line.trim().split(" ").enumerate() {
            let num: u128 = num.parse().unwrap();
            top_row.push(max(num + bottom_row[index], num + bottom_row[index + 1]));
        }
        bottom_row = top_row;
    }
    bottom_row[0]
}

fn main() {
    let pyramid =
        fs::read_to_string("triangle.txt").expect("Should have been able to read the file");
    println!("{}", max_path_sum(&pyramid));
}
