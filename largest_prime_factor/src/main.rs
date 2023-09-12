fn largest_prime_factor(mut num: u64) -> u64 {
    let mut largest_factor = 1;
    for i in 2..=(num as f32).sqrt() as u64 {
        while num % i == 0 {
            largest_factor = i;
            num /= i;
        }
    }
    largest_factor
}
fn main() {
    println!("{}",largest_prime_factor(600851475143));
}
