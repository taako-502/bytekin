# Bytekin

Bytekinは、Rustを学びながら作るオリジナルのデジタル生物育成シミュレーションです。

小さな関数とテストから始め、育成状態、レベル、行動、時間経過といった仕組みを段階的に追加します。完成品を先に設計し切るのではなく、動く範囲を少しずつ広げる学習用プロジェクトです。

## 現在の段階

現在は、経験値からレベルを計算する最初の関数とテストを実装した段階です。

最初の到達点は、ターミナル上で次の操作ができる小さな育成ループです。

- Bytekinの状態を確認する
- 育成行動を1つ選ぶ
- 行動によって経験値や状態が変化する
- 一定条件でレベルが上がる

詳しい方針は[コンセプト](docs/CONCEPT.md)を参照してください。

## 実行

```bash
cargo run
```

## テスト

```bash
cargo test
```

## 開発時のチェック

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D clippy::all
cargo test --all-features
```
