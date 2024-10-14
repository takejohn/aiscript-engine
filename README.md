# AiScript Engine **(開発中)**

Rustで書かれたAiScriptエンジンです。
[AiScript](https://github.com/aiscript-dev/aiscript)リポジトリの`master`ブランチを参照しています。

- パーサ移植済み
- インタプリタ着手中

# テスト方法
```
git submodule update --init
cd aiscript
npm install
npm run build
cd ../aiscript-js-build-tests
pnpm install
cd ..
cargo test
```
