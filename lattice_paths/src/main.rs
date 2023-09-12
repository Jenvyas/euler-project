use std::collections::HashMap;

struct LaticePaths {
    cache: HashMap<((u8, u8), (u8, u8)), u128>
}

impl LaticePaths {
    fn new() -> Self {
        LaticePaths{cache: HashMap::new()}
    }
    fn count_paths(&mut self, start: (u8, u8), end: (u8, u8)) -> u128 {
        if let Some(count) = self.cache.get(&(start, end)) {
            *count
        } else {
            let count = match (start, end) {
                ((x1, y1), (x2, y2)) if x1 == x2 || y1 == y2 => 1,
                ((x, y), end) => {
                    self.count_paths((x + 1, y), end) + self.count_paths((x, y + 1), end)
                }
            };

            self.cache.insert((start, end), count);
            count
        }
    }
}

fn main() {
    let paths = LaticePaths::new().count_paths((0, 0), (20, 20));
    println!("{:?}", paths);
}
