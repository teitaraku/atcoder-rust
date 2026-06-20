use std::process::exit;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: String,
        s: [String; n],
    }
    for i in 0..n {
        let seat = match &*x {
            "A" => &s[i][0..1],
            "B" => &s[i][1..2],
            "C" => &s[i][2..3],
            "D" => &s[i][3..4],
            "E" => &s[i][4..5],
            _ => exit(1),
        };
        if seat == "o" {
            println!("Yes");
            exit(0);
        }
    }
    println!("No");
}
