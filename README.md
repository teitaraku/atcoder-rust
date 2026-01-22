# atcoder-rust

AtCoder の問題を Rust で解いたコードをまとめるリポジトリです。

## 構成

[cargo-compete](https://github.com/qryxip/cargo-compete) を使用してコンテストごとにパッケージを管理しています。

```
.
├── compete.toml          # cargo-compete の設定
├── template-cargo-lock.toml
└── abs/                  # コンテストごとのパッケージ
    ├── Cargo.toml
    ├── src/bin/          # 各問題の解答
    │   ├── abc081a.rs
    │   └── ...
    └── testcases/        # テストケース (YAML)
        ├── abc081a.yml
        └── ...
```

## 使い方

### 新しいコンテストを追加
```bash
cargo compete new <contest-id>
# 例: cargo compete new abc300
```

### テストケースをダウンロード
```bash
cargo compete download <contest-id>
```

### 特定の問題をテスト
```bash
cargo compete test <problem-alias>
# 例: cargo compete test abc081a
```

### 解答を提出
```bash
cargo compete submit <problem-alias>
# 例: cargo compete submit abc081a
```

### 直接実行（cargo run）
```bash
cargo run --bin abs-abc081a
```

## 環境
- Rust 1.89.0 (AtCoder 2025 言語アップデート対応)
- 依存ライブラリは AtCoder 2025 のジャッジ環境に合わせて固定

参考:
- [言語アップデート 2025 言語一覧](https://img.atcoder.jp/file/language-update/2025-10/language-list.html)
- [Rust 環境設定 (rustc.toml)](https://img.atcoder.jp/file/language-update/2025-10/088-1-82-0_rustc.toml)

## 依存ライブラリ
- その他 AtCoder 2025 で利用可能なクレート群（`compete.toml` 参照）
