fn day_of_the_week(date: u32, month: u32, year: u32) -> u32 {
    let j = year / 100;
    let k = year % 100;
    let m = [13, 14, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12][month as usize - 1];
    (date + (13 * (m + 1) / 5) + k + k / 4 + j / 4 + 5 * j) % 7
    //zeller's congruence
}

fn main() {
    let mut total_sundays = 0;
    for year in 1901..=2000 {
        for month in 1..=12 {
            if day_of_the_week(1, month, year) == 1 {
                total_sundays += 1;
            }
        }
    }
    println!("{}", total_sundays);
}
