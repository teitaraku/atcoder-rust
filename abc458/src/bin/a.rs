use proconio::input;

fn main() {
    input! {
        s: String,
        n: usize,
    }
    let end = s.len() - n;
    if let Some(ns) = s.get(n..end) {
        println!("{}", ns);
    }
}
