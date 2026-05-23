use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let mut s = "HelloWorld".to_string();
    s.remove(x - 1);
    println!("{}", s);
}
