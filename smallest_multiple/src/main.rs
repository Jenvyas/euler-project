use std::collections::HashMap;

fn insert_larger_factor(factors: &mut HashMap<i32, i32>, factor_count: &(i32, i32)) {
    let entry = factors.entry(factor_count.0).or_insert(factor_count.1);
    if *entry < factor_count.1 {
        *entry = factor_count.1;
    }
}

fn common_prime_factors(nums: &Vec<i32>) -> HashMap<i32, i32> {
    let mut factors = HashMap::new();
    for num in nums {
        let mut num = *num;
        let mut factor_count = (2, 0);
        while num % 2 == 0 {
            factor_count.1 += 1;
            num /= 2;
        }
        insert_larger_factor(&mut factors, &factor_count);
        for i in 3..=(num as f32).sqrt() as i32 {
            factor_count = (i, 0);
            while num % i == 0 {
                factor_count.1 += 1;
                num /= i;
            }
            insert_larger_factor(&mut factors, &factor_count);
        }
        if num > 2 {
            factor_count = (num, 1);
            insert_larger_factor(&mut factors, &factor_count);
        }
    }
    factors
}

fn main() {
    let nums: Vec<i32> = (1..=20).collect();
    println!("{:?}", common_prime_factors(&nums));
    println!(
        "{:?}",
        common_prime_factors(&nums)
            .iter()
            .fold(1, |product, factor| {
                let mut prod = 1;
                for _ in 0..*factor.1 {
                    prod *= factor.0;
                }
                product * prod
            })
    );
}
