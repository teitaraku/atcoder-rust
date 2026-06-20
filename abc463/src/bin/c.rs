use proconio::input;

fn main() {
    input! {
        n: usize,
        hl: [(usize, usize);n],
        q: usize,
        t: [usize; q],
    }
    let mut queries = vec![];
    for _ in 0..q {
        input! {
            k: usize,
            b: [usize; k],
        }
        queries.push(b);
    }
    println!("{:?}", queries);
}
