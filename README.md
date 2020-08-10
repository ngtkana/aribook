# あり本付属の問題を解いていきましょう。

[![](https://github.com/ngtkana/aribook/workflows/Rust/badge.svg)](https://github.com/ngtkana/aribook/actions)

## レポジトリの運用

### 方針

問題ごとに bin crate を作っていく方針です。

### 作り方

`generate.sh ${CHAP_NAME} ${PROBLEM_NAME}` で作れるようにしています。

これが自動で行われます。

- ディレクトリの追加
- bin crate の追加
- `proconio`, `proconcli` への依存の追加 (via `cargo-edit`)
- `src/main`, `Cargo.toml` へのテンプレートのコピーができます。
    - テンプレートは `src` 以下にあります。

### 仕様

- Crate のディレクトリは、`crates/${CHAP_NAME}/${PROBLEM_NAME}` です。
- Crate のバイナリ名は、`${CHAP_NAME}_${PROBLEM_NAME}` です。
- Crate 直下のテストのモジュール名は、`${CHAP_NAME}_${PROBLEM_NAME}_test` です。
    - `chap_2_3` 以前はテストのお名前が違うので、気が向いたら移植です。


### テスト

自動生成される crate の中にあるテストを書き換えると、標準入出力テストができます。
そこにあり本付属のサンプルを書き込みましょう。
すると、`cargo-test` が通ることが、サンプルが通ることを保証します。


### 便利機能

- crate の copy ができるようになりました。


## 欲しい機能

- `generate.sh` をしたときに、`README.md` に自動でリンクを追加していきたいです。
- 機能が増えてきたら `clap` などで集約したいです。
- オンラインハック機能（遠い将来）
