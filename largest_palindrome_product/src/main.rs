fn is_palindrome(num: u32) -> bool {
    let num = num.to_string();
    num[..num.len()/2] == num[(num.len() as f32/2.0).ceil() as usize..].chars().rev().collect::<String>()
}
fn main() {
    let mut max_palindrome = 0;
    let bound = (100, 1000);
    for i in bound.0..bound.1 {
        for j in bound.0..bound.1 {
            let product = i*j;
            if is_palindrome(product) && product>max_palindrome {
                max_palindrome = product;
            }
        }
    }
    println!("{max_palindrome}");
}
