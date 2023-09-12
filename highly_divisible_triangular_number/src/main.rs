fn factors(num: i32) -> i32 {
    let mut factors = 2;

    for i in 2..(num as f32).sqrt() as i32 {
        if num % i == 0 {
            factors+=2;
        }
    }

    if num % (num as f32).sqrt() as i32 == 0 {
        factors+=1;
    }

    factors
}

fn main() {
    let mut divisors = 0;
    let mut n = 1;
    let mut num = 0;

    while divisors<=500 {
        num += n;
        divisors = factors(num);
        n += 1;
    }
    println!("{num}");
}
