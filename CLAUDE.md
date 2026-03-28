# CLAUDE.md

このファイルは、Claude Code (claude.ai/code) がこのリポジトリで作業する際のガイダンスを提供します。

**重要: このリポジトリに関するすべての応答は日本語で行ってください。**

## 概要

`cargo-compete` で管理する AtCoder の競技プログラミング解答集（Rust）。コンテストごとに独立した Cargo パッケージがあり、問題ごとにバイナリターゲットを持つ。

## 主なコマンド

```bash
# 新しいコンテストパッケージを作成（問題とテストケースをダウンロード）
cargo compete new abc452

# 特定の問題をテスト（コンテストディレクトリ内で実行）
cargo compete test a
cargo compete test abc451-a   # フルネームでも可

# 解答を提出
cargo compete submit a

# 既存コンテストのテストケースをダウンロード
cargo compete download abc451
```

## プロジェクト構成

各コンテストは独立したディレクトリ（例: `abc451/`）に配置される:

```
abc451/
├── Cargo.toml          # 問題ごとの [[bin]] ターゲットを定義
├── src/bin/
│   ├── a.rs            # 問題 A の解答
│   ├── b.rs            # 問題 B の解答
│   └── ...
└── testcases/
    ├── a.yml           # 問題 A のテストケース
    └── ...
```

各コンテストパッケージは統合された Cargo ワークスペースではなく、それぞれ独立している。

## 解答のテンプレートパターン

すべての解答は入力解析に `proconio` を使用する:

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    // 解法のロジック
    println!("{}", answer);
}
```

## 問題文の参照

問題文は以下のURL形式でアクセスできる:

```
https://atcoder.jp/contests/{contest}/tasks/{contest}_{problem}
```

例: `https://atcoder.jp/contests/abc451/tasks/abc451_a`

## 環境

- Rust 1.89.0（AtCoder 2025 標準、`mise` で管理）
- AtCoder のジャッジ環境に合わせた 100 以上のピン留めされた依存クレート（`compete.toml` で定義）
- 主な利用可能クレート: `proconio`, `ac-library-rs`, `itertools`, `petgraph`, `num`, `rand`, `indexmap`, `rustc-hash`
