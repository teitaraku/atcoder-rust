use proconio::input;

fn main() {
    input! {
        n: usize,
        t: i32,
        a: [i32;n],
    }

    let mut sum = 0;
    let mut next_up = 0;
    for i in 0..n {
        if a[i] < next_up {
            continue;
        }
        sum += a[i] - next_up;
        next_up = a[i] + 100;
    }
    if 0 < n {
        if a[n - 1] < t && next_up < t {
            sum += t - next_up;
        }
    } else {
        sum = t;
    }
    println!("{}", sum);
}
