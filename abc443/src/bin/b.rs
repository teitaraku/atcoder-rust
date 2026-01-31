use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
    }

    let mut i = 0;
    let mut sum = 0;
    loop {
        sum = sum + (n + i);
        if k <= sum {
            break;
        }
        i += 1;
    }
    println!("{}", i);
}
