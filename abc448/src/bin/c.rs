use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize;n],
    }
    let mut query = vec![];
    for _ in 0..q {
        input! {
            k: usize,
            b: [usize;k],
        }
        query.push(b);
    }

    for i in 0..q {
        let mut ac = a.clone();
        let b = &query[i];
        for &j in b {
            ac[j - 1] = 9999999999;
        }
        let min = ac.iter().min().unwrap();
        println!("{}", min);
    }
}
