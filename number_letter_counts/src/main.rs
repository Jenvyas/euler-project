fn three_digits_to_word(mut num: u32, power: u32) -> String {
    let single_digits = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let ten_to_nineteen = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let double_digit = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    let fixes = [
        "",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
    ];

    let mut number = String::from("");
    let mut num_vec: Vec<usize> = Vec::new();

    while num > 0 {
        num_vec.push((num % 10) as usize);
        num /= 10;
    }
    for (index, digit) in num_vec.iter().enumerate() {
        if index == 0 && num_vec.len() == 1 {
            number = single_digits[*digit].to_owned();
        } else if index == 1 {
            match *digit {
                0 => {
                    number = single_digits[num_vec[0]].to_string();
                }
                1 => {
                    number = ten_to_nineteen[num_vec[0]].to_string();
                }
                2..=9 => {
                    if num_vec[0] == 0 {
                        number = double_digit[*digit].to_string();
                    } else {
                        number = format!("{}-{}", double_digit[*digit], single_digits[num_vec[0]]);
                    }
                }
                _ => {}
            }
        } else if index > 1 {
            number = match &number[..] {
                "" => format!("{} hundred", single_digits[*digit]),
                num => format!("{} hundred and {}", single_digits[*digit], num),
            };
        }
    }
    if power == 0 {
        return number
    }
    match &number[..] {
        "" => number,
        _ => format!("{} {}", number, fixes[power as usize]),
    }
}

fn num_to_word(mut num: u32) -> String {
    let mut power = 0;
    let mut number = String::from("");
    if num == 0 {
        return "zero".to_string();
    }
    while num > 0 {
        let num_slice = num % 1000;
        let new_three_digits = three_digits_to_word(num_slice, power);
        if power == 0 {
            number = match num_slice {
                1..=99 => match num/1000 {
                    0 => new_three_digits,
                    _ => format!("and {}", new_three_digits)
                },
                100..=999 => new_three_digits,
                _ => number,
            };
        } else {
            number = match num_slice {
                1..=99 => match num/1000 {
                    0 => format!("{} {}", new_three_digits, number),
                    _ => format!("and {} {}", new_three_digits, number)
                },
                100..=999 => format!("{} {}", new_three_digits, number),
                _ => number,
            };
        }
        num /= 1000;
        power += 1;
    }
    number
}

fn main() {
    let mut total_count = 0;
    for i in 1..=1000 {
        total_count+=num_to_word(i).replace([' ', '-'], "").len();
    }
    println!("{}",total_count);
}
