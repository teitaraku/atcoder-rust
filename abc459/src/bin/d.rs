use proconio::input;

fn main() {
    input! {
        t: usize,
        cases: [String; t],
    }
    for i in 0..t {
        let case = cases[i];
        let l = case.len();
        // アルファベットそれぞれの個数がが l/2 より小さければ隣合わせにならないはず

        println!("{}", case[i]);
    }
}
