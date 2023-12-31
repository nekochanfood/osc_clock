---
sidebar_position: 10
---

# `Resources`内のPrefab

これらの Prefab には、OSC Clock プログラムと通信ができるようにするコンポーネントが含まれています。

## 各 Prefab の解説

### `Internal/osc_clock@Parameters`

アバター内にパラメータを追加し、OSC Clock プログラムから送信された値を使用できるようにします。

:::warning

**この Prefab は `Internal/osc_clock@FX` を必要とします**

`Internal/osc_clock@FX` は アバターの初期化時に`osc_clock@ForceSync`の値を瞬間的に変更し、プログラムに更新をリクエストします。

この Prefab だけでは正しく動作しない可能性があります。

:::

### `Internal/osc_clock@FX`

アバターの初期化時に`osc_clock@ForceSync`の値を瞬間的に変更し、プログラムに更新をリクエストする機能を追加します。

:::warning

**この Prefab は `Internal/osc_clock@Parameters` を必要とします**

`Internal/osc_clock@Parameters` は `osc_clock@ForceSync`をアバターに追加します。

この Prefab だけでは動作しません。

:::

### `osc_clock@Modules`

上記の Prefab をひとつにまとめた Prefab です。

この Prefab を、オブジェクトに追加することをおすすめします。

## やってはいけないこと

:::danger
### 値を Prefab 元から変更しないでください

`Resources/Prefab`内にある Prefab を直接開いて変更を加えないでください。

他の OSC Clock を使用しているアセットにも影響します。

ヒエラルキー内の Prefabの値の変更は適用しなければ OK です。
:::

:::danger
### Unpack Prefab しないでください

Unpack Prefabを行うと、後にこれらの Prefab に変更を加えたときに、その変更が適用されません。
:::


