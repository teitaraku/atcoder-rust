use proconio::input;

fn main() {
    input! {
        n: i32,
        mut a: [i32;n],
    }
    // 昇順でソート（末尾が最大）
    a.sort_by(|a, b| a.cmp(&b));
    let mut alice: Vec<i32> = vec![];
    let mut bob: Vec<i32> = vec![];
    loop {
        if let Some(x) = a.pop() {
            alice.push(x);
        } else {
            break;
        }
        if let Some(x) = a.pop() {
            bob.push(x);
        } else {
            break;
        }
    }
    let diff = alice.iter().sum::<i32>() - bob.iter().sum::<i32>();
    println!("{}", diff);
}
