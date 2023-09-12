fn main() {
    println!("Hello, world!");
    'outer: for a in 1..1000 {
        for b in 1..1000 - a {
            let c = 1000 - a - b;
            let squares: [u64; 3] = [(a as u64).pow(2), (b as u64).pow(2), (c as u64).pow(2)];
            if squares[0] + squares[1] == squares[2] {
                println!("{a}, {b}, {c}");
                println!("{}", a * b * c);
                break 'outer;
            }
        }
    }
}
