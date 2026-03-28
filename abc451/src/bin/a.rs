use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let result = s.len() == 5 || s.len() == 10;

    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}
