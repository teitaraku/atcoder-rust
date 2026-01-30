use proconio::input;

/// 選び方の組み合わせは膨大(100000C50000)になるのでループでは不可
/// どう考えてもわからず、解説を見て端から長さx以上で切れるかどうかを試せばO(N)で解けるということが分かった。
/// 解説ではさらに長さxを二分探索で見つけると良いと書いてあり、確かに提出時のテストケースは二分探索でないと実行時間オーバーだった
fn main() {
    input! {
        n: usize,
        l: i32, // 羊羹の長さ
        k: i32, // 切る数
        _a: [i32;n], // 切れ目の位置
    }
    // a_0=0 と終端 a_{n+1}=lを置く
    let mut a = vec![0];
    a.extend(_a);
    a.extend([l]);
    // 二分探索で求める
    let mut left = 0;
    let mut right = l / (k + 1) + 1; // スコアは平均の長さより大きくなれない
    let mut p = right / 2;
    while 1 < right - left {
        let mut temp = 0;
        let mut count = 0;
        for i in 1..a.len() {
            temp += a[i] - a[i - 1];
            // 長さを確保できたらカウント
            if p <= temp {
                temp = 0;
                count += 1;
            }
        }
        if k + 1 <= count {
            // k+1個確保できればOK. pを大きくして右を探索
            left = p;
            p = (p + right) / 2;
        } else {
            // そうでなければpを小さくして左を探索
            right = p;
            p = (left + p) / 2;
        }
    }
    println!("{}", p);
}
