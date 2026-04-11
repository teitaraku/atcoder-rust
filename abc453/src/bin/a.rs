use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut flag = false;
    for c in s.as_str().chars() {
        if c != 'o' || flag {
            flag = true;
            print!("{}", c);
        }
    }
}
