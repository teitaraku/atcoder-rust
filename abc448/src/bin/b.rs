use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut c: [usize;m],
        ab: [(usize, usize);n]
    }
    let mut used = 0;

    for i in 0..n {
        let &(a, b) = &ab[i];
        let mut amount = 0;
        if 0 < c[a - 1] {
            if b <= c[a - 1] {
                amount = b;
            } else {
                amount = c[a - 1];
            }
        }
        c[a - 1] -= amount;
        used += amount;
    }
    println!("{}", used);
}
