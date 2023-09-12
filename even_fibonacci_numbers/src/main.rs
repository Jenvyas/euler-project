fn main() {
    let mut n: i32 = 1;
    let mut n_plus_one = 2;
    let mut sum = 0;
    while n < 4000000 {
        if n % 2 == 0 {
            sum += n;
        }
        n_plus_one += n;
        n = n_plus_one - n;
    }
    println!("{sum}");
}
