use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut people: Vec<Vec<usize>> = (0..=n).map(|_| (0..=n).map(|_| 0).collect()).collect();
    for i in 1..=n {
        input! {
            k: usize,
            a: [usize; k],
        }
        for j in 0..k {
            people[a[j]][0] += 1;
            people[a[j]][i] = 1;
        }
    }

    for i in 1..=n {
        print!("{}", people[i][0]);
        for j in 1..=n {
            if people[i][j] == 1 {
                print!(" {}", j);
            }
        }
        println!();
    }
}
