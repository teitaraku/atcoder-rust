use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    for i in 1..=h {
        let mut line = vec![];
        for j in 1..=w {
            let mut count = 0;
            if 1 < i {
                count += 1;
            }
            if i < h {
                count += 1;
            }
            if 1 < j {
                count += 1;
            }
            if j < w {
                count += 1;
            }
            line.push(count);
        }
        println!("{}", line.iter().join(" "));
    }
}
