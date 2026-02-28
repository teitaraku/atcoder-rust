use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
    }

    if m <= n / 2 + n % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
