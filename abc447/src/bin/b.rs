use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
    }
    let mut counts = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let max_count = counts.values().max().unwrap();

    let mut st = s.clone();
    for (c, count) in &counts {
        if count == max_count {
            st.retain(|x| x != *c);
        }
    }

    if st != "" {
        println!("{}", st);
    }
}
