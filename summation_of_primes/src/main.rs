fn is_prime(num: i32) -> bool {
    for i in 2..=(num as f32).sqrt() as i32 {
        if num % i == 0 {
            return false;
        }
    }
    true
}
fn main() {
    let mut sum: u64= 0;
    for i in 2..2000000 {
        if is_prime(i) {
            sum += i as u64;
        }
    }
    println!("{}",sum);
}
