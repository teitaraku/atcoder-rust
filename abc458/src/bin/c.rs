use regex::Regex;

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let length = s.len();
    let re = Regex::new(r"C").unwrap();
    let mut counter = 0;
    for m in re.find_iter(&s) {
        let pos = m.start();
        if length - pos > pos {
            counter += pos + 1;
        } else {
            counter += length - pos;
        }
    }
    println!("{}", counter);
}
