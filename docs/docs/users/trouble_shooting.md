---
sidebar_position: 999
---

# トラブルシューティング

### `You are trying to replace or create~` というエラーが発生する

[Modular Avatar](https://modular-avatar.nadena.dev/ja) を導入してください。

### パラメータが更新されない

既にアップロードされているアバターに OSC Clock を導入すると、パラメータが更新されない問題があります。

1. Win + Rを押して**ファイル名を指定して実行**ウィンドウを開きます。
2. `%APPDATA%\..\LocalLow\VRChat\VRChat\OSC\` と入力して実行します。
3. 開いたウィンドウの中にあるフォルダの中から、フォルダ名が自分のユーザーIDと一致するフォルダを開いてください。
4. 開いたフォルダの中から、ファイル名が問題のアバターのIDと一致するファイルを見つけ、削除してください。

上記の手順を完了させた上で、問題のアバターを着用し直すとパラメータが更新されるはずです。

### `config.json` を破壊してしまった

`.\osc_clock.json repair` を実行することで、config.jsonを修復・生成することができます。
