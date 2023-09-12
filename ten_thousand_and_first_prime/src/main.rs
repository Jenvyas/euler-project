fn is_prime(num: i32) -> bool {
    for i in 2..=(num as f32).sqrt() as i32 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("Hello, world!");
    let mut counter = 0;
    let mut num = 1;
    while counter != 10001 {
        num += 1;
        if is_prime(num) {
            counter += 1;
        }
    }
    println!("{}", num);
}
