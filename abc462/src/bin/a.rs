use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let ans = s
        .chars()
        .filter(|&c| c.is_ascii_digit())
        .collect::<String>();
    if ans != "" {
        println!("{}", ans);
    }
}
