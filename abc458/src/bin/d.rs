use proconio::input;

use std::collections::BTreeMap;

fn main() {
    input! {
        x: usize,
        q: usize,
    }
    let mut values = vec![];
    values.push(x);
    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
        }
        values.push(a);
        values.push(b);
        values.sort();
        let medium = values.get(values.len() / 2).unwrap();
        println!("{}", medium);
    }
}
