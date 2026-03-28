use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut c = vec![];
    for i in 1..n {
        input! {
            ci: [usize;n-i],
        }
        c.push(ci);
    }

    println!("{:?}", c);

    println!("Yes");
}
