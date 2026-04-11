use proconio::input;

fn main() {
    input! {
        t: usize,
        x: i32,
        a: [i32; t+1],
    }

    println!("{} {}", 0, a[0]);
    let mut tmp: i32 = a[0];
    for i in 1..=t {
        if (tmp - a[i]).abs() >= x {
            tmp = a[i];
            println!("{} {}", i, a[i]);
        }
    }
}
