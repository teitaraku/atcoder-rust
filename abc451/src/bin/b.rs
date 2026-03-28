use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        input! {
            ai: usize,
            bi: usize,
        }
        a.push(ai);
        b.push(bi);
    }

    let mut dep_p = vec![];
    let mut dep_n = vec![];
    for _ in 0..m {
        dep_p.push(0);
        dep_n.push(0);
    }
    for i in 0..n {
        dep_p[a[i] - 1] += 1;
        dep_n[b[i] - 1] += 1;
    }
    for i in 0..m {
        println!("{}", dep_n[i] - dep_p[i]);
    }
}
