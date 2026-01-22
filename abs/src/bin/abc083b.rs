use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }
    let matched_num: Vec<i32> = (1..=n)
        .map(|x| (x, sum_of_each_digit(x)))
        .filter(|(_, s)| a <= *s && *s <= b)
        .map(|(x, _)| x)
        .collect();
    println!("{}", matched_num.iter().sum::<i32>());
}

fn sum_of_each_digit(x: i32) -> i32 {
    let mi = x / 10000 % 10;
    let th = x / 1000 % 10;
    let hu = x / 100 % 10;
    let te = x / 10 % 10;
    let on = x % 10;
    mi + th + hu + te + on
}
