fn main() {
    let start = 1;
    let end = 100;
    let mut sum_of_squares = 0;
    for i in start..=end {
        sum_of_squares += (i as i32).pow(2);
    }
    let square_of_sum: i32 = (((end/2)*(start+end)) as i32).pow(2);
    println!("{}",square_of_sum-sum_of_squares);
}
