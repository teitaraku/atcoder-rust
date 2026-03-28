use proconio::input;

fn main() {
    input! {
        s: String,
    }

    if s.len() == 5 || s.len() == 10 {
        println!("Yes");
    } else {
        println!("No");
    }
}
