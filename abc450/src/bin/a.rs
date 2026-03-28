use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let s: String = (0..n)
        .map(|x| n - x)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("{}", s);
}
