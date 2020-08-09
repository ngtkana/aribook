# あり本付属の問題を解いていきましょう。

[![](https://github.com/ngtkana/aribook/workflows/Rust/badge.svg)](https://github.com/ngtkana/aribook/actions)

## レポジトリの運用

### 方針

問題ごとに bin crate を作っていく方針です。

### 作り方

`generate.sh ${CRATE_NAME}` で作れるようにしています。

これが自動で行われます。

- bin crate の追加
- `proconio`, `proconcli` への依存の追加 (via `cargo-edit`)
- `src/main` へのテンプレートのコピーができます。(from `src/tempalte.rs`, via `cat` and `sed`)

手動で行う必要のあることです。

- Work space root の `Cargo.toml` への member の追加です。


### テスト

自動生成される crate の中にあるテストを書き換えると、標準入出力テストができます。
そこにあり本付属のサンプルを書き込みましょう。
すると、`cargo-test` が通ることが、サンプルが通ることを保証します。
