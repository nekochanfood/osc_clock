---
sidebar_position: 1
---

# ようこそ

OSC Clock の使い方や説明を集めたドキュメントサイトです。

## OSC Clock とは？

OSC Clock はOSCを通して日付と時刻を VRChat に送信するプログラムです。

これを使うことで、正確な時間を表示する腕時計を簡単に作成することができます。


## プログラムをダウンロードする

[https://github.com/nekochanfood/osc_clock/releases](https://github.com/nekochanfood/osc_clock/releases/latest) から、最新版の OSC Clock をダウンロードすることができます。

### 自分でプログラムをビルドする

コンパイル済みの実行ファイルをダウンロードして実行することに不安を感じていますか？安心してください。

このソフトウェアはオープンソースソフトウェアであり、構成するすべてのソースコードを自由に閲覧できるほか、ご自身のマシン上でビルドすることも可能です。

Rust および Cargo がインストールされていない場合は、事前に [https://www.rust-lang.org/](https://www.rust-lang.org/) よりセットアップしてください。

まずは、OSC Clock のリポジトリをローカル環境にクローンします。
```
git clone https://github.com/nekochanfood/osc_clock.git
cd osc_clock
```

次に、依存関係をインストールします。
```
cargo fetch
```

ビルドを開始します。
```
cargo build --release
```

ビルドが成功すると、実行ファイルは次の場所に生成されます。
```
target/release/osc_clock
```

## リソースをインストールする

:::danger
OSC Clock のリソースのほとんどは、Modular Avatarを使用しているため、このリソースを使用するにはプロジェクトに Modular Avatar がインストールされている必要があります。

[ここ](https://modular-avatar.nadena.dev/ja/docs/intro)の手順に従って、Modular Avatarをインストールしてください。
:::

[ここ](vcc://vpm/addRepo?url=https://vpm.chanfoo.net/vpm.json)をクリックして、VCC に OSC Clock のレポジトリを追加します。

"I understand, Add Repository" をクリックして、レポジトリを追加します。

![レポジトリを追加](../static/img/add_repo.png)

インストールしたいプロジェクトの管理画面を開き、"OSC Clock"と書かれているパッケージの "+" か、ドロップダウンから最新のバージョンをクリックしてインストールします。

![パッケージを追加](../static/img/install_package.png)
