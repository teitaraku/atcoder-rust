use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let mut count = -1;

    let mut s2 = s.clone();
    s2.retain(|c| c != 'A');

    let mut t2 = t.clone();
    t2.retain(|c| c != 'A');

    // println!("{}", s2);
    // println!("{}", t2);

    if s2 != t2 {
        println!("{}", count);
    } else {
        println!("{}", 0);
    }
}
