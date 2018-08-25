[Rust速習会1.pdf - Speaker Deck](https://speakerdeck.com/qnighy/rustsu-xi-hui-1?slide=164) を見つつ写経したりコメントを追加したものです。

# 環境構築

```
$ curl https://sh.rustup.rs -sSf | sh
```

# 開発

```
// ビルド、実行
$ cargo build
$ cargo run
// リリースビルド、実行（時間かかるけど高速なバイナリができる）
$ cargo build --release
$ cargo run --release
// コンパイルが通るかチェック
$ cargo check
```
