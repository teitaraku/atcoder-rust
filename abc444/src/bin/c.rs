use proconio::input;

// すべてのケースを出さなければならないが1ケースしか出せていない
fn main() {
    input! {
        n: usize,
        a: [i32;n],
    }

    let max = a.iter().max().unwrap();
    let min = a.iter().min().unwrap();
    if n % 2 != 0 {
        println!("{}", max);
    } else if max == min {
        println!("{}", max);
    } else {
        let mut sorted = a.clone();
        sorted.sort_by(|x, y| x.cmp(&y));
        let mut pre = 0;
        let mut flag = false;
        for i in 0..n / 2 {
            let sum = sorted[i] + sorted[n - 1 - i];
            if 0 < pre && pre != sum {
                flag = true;
                break;
            }
            pre = sum;
        }
        if flag {
            println!("{}", max);
        } else {
            println!("{}", max + min);
        }
    }
}
