use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let first = s.chars().nth(0).unwrap();
    let last = s.chars().last().unwrap();
    if first.eq(&last) {
        println!("Yes");
    } else {
        println!("No");
    }
}
