use proconio::input;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, usize); q],
    }
    let mut block: Vec<usize> = vec![0; n];
    for i in 0..q {
        let (c, x) = queries[i];
        if c == 1 {
            block[x - 1] += 1;
            if !block.iter().any(|&a| a == 0) {
                block.iter_mut().for_each(|a| *a -= 1);
            }
        } else {
            let count = block.iter().filter(|&a| a >= &x).count();
            println!("{}", count);
        }
    }
}
