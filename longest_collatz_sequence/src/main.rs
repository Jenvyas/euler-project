fn main() {
    let mut max_len = 1;
    let mut max_num = 1;
    for mut i in 1u64..1000000 {
        let num = i;
        let mut len = 1;
        while i != 1 {
            if i % 2 == 0 {
                i /= 2;
            } else {
                i = 3 * i + 1;
            }
            len += 1;
        }
        if len > max_len {
            max_len = len;
            max_num = num;
        }
    }
    println!("{}: {}", max_num, max_len);
}
