use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
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
