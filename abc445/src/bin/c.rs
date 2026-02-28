use num::ToPrimitive;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    }
    let mut answers: Vec<u32> = vec![0; n];
    // 答えのメモ
    let mut map: HashMap<u32, u32> = HashMap::new();
    for i in 0..n {
        let mut next = a[i];
        loop {
            if next == a[next.to_usize().unwrap() - 1.to_usize().unwrap()] {
                break;
            }
            next = a[next.to_usize().unwrap() - 1.to_usize().unwrap()];
            // メモした答えがあればそれを使う
            if let Some(ans) = map.get(&next) {
                next = *ans;
                break;
            }
        }
        answers[i] = next;
        map.insert(a[i], next);
    }
    println!(
        "{}",
        answers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
