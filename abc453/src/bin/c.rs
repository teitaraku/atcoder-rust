use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i64; n],
    }

    let mut pos: i64 = 5;
    let mut count = 0;
    for i in 0..n {
        let pre_pos = pos;
        if pos > 0 {
            pos = pos - l[i] * 10;
        } else {
            pos = pos + l[i] * 10;
        }
        if ((pre_pos > 0) && (pos < 0)) || ((pre_pos < 0) && (pos > 0)) {
            count += 1;
        }
    }
    println!("{}", count);
}
