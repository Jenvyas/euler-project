fn divisor_sum(num: u32) -> u32 {
    let mut divisor_sum = 1;
    let sqrt_1 = (num as f32).sqrt();
    for div in 2..=sqrt_1 as u32 {
        if num % div == 0 {
            if sqrt_1.fract() == 0.0 {
                divisor_sum += sqrt_1 as u32;
            } else {
                divisor_sum += div + num / div;
            }
        }
    }
    divisor_sum
}

fn amicable_pair_sum(max: u32) -> u32 {
    let mut sum = 0;
    for i in 1..max {
        let sum_1 = divisor_sum(i);
        let sum_2 = divisor_sum(sum_1);
        if sum_2 == i && sum_1 != i {
            sum += sum_1;
        }
    }
    sum
}

fn main() {
    println!("{}", amicable_pair_sum(10000));
}
