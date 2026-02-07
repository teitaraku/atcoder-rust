use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
    }

    let count = (1..=n).map(|x| ketawa(x)).filter(|x| *x == k).count();
    println!("{}", count);
}

fn ketawa(_x: i32) -> i32 {
    let mut sum = 0;
    let mut x = _x;
    while 0 < x {
        sum += x % 10;
        x = x / 10;
    }
    sum
}
