# atcoder-rust

AtCoder の問題を Rust で解いたコードをまとめるリポジトリです。

## 構成
- 問題ごとにディレクトリを分けてコードを管理
- 各問題ディレクトリには以下のファイルが含まれます：
  - `src/main.rs` - 解答コード
  - `tests/input_test.rs` - テストケース

## 使い方

### 実行
```bash
cd PracticeA
cargo run
```

### テスト
```bash
cd PracticeA
cargo test
```

## 方針
- 入力処理には `proconio` クレートを使用
- テストは統合テストとして実装し、標準入出力をテスト

## 依存ライブラリ
- [proconio](https://docs.rs/proconio/) - 競技プログラミング用の入力処理ライブラリ
