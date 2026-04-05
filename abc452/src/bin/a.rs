use proconio::input;

fn main() {
    input! {
        m: usize,
        d: usize,
    }

    let pairs = [(1, 7), (3, 3), (5, 5), (7, 7), (9, 9)];

    if pairs.contains(&(m, d)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
