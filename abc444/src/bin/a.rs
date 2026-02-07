use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let n3 = n / 100;
    let n2 = (n % 100) / 10;
    let n1 = n % 10;
    if n3 == n2 && n2 == n1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
