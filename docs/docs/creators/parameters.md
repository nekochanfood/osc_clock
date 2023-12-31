---
sidebar_position: 5
---

# パラメータ

`秒`から`年`まで様々な値をアバター内で使用できるようにします。

## 全てのパラメータの内訳

例にした日付と自国: `2012/03/14 12:34:56`
| 種類 | パラメータのアドレス  | 型 | 範囲 | 例を基にした値 |
| :- | - | - | - | :- |
| [Second (Float)](#second---秒) | `/avatar/parameters/osc_clock@second_f`  | Float  | 0 ~ 1 | `0.933333` |
| [Second (Int)](#second---秒) | `/avatar/parameters/osc_clock@second_i`  | Int  | 0 ~ 59 | `56` |
| [Minute (Float)](#minute---分) | `/avatar/parameters/osc_clock@minute_f`  | Float  | 0 ~ 1 | `0.566666` |
| [Minute (Int)](#minute---分) | `/avatar/parameters/osc_clock@minute_i`  | Int  | 0 ~ 59 | `34` |
| [24 Hour (Float)](#24-hour---24時間表記) | `/avatar/parameters/osc_clock@hour24_f`  | Float  | 0 ~ 1 | `0.5` |
| [24 Hour (Int)](#24-hour---24時間表記) | `/avatar/parameters/osc_clock@hour24_i`  | Int  | 0 ~ 23 | `12` |
| [12 Hour (Float)](#12-hour---12時間表記) | `/avatar/parameters/osc_clock@hour12_f`  | Float  | 0 ~ 1 | `0.0472221666666667` |
| [12 Hour (Int)](#12-hour---12時間表記) | `/avatar/parameters/osc_clock@hour12_i`  | Int  | 0 ~ 11 | `0` |
| [AM/PM (Bool)](#午前午後---ampm) | `/avatar/parameters/osc_clock@hour_isPM`  | Bool  | True or False | `True` |
| [Day (Int)](#day---日) | `/avatar/parameters/osc_clock@day`  | Int  | 1 ~ 31 | `14` |
| [Day Of Week (Int)](#day-of-week---曜日) | `/avatar/parameters/osc_clock@dofw`  | Int  | 0 ~ 6 (月曜日スタート) | `2` |
| [Month (Int)](#month---月) | `/avatar/parameters/osc_clock@month`  | Int  | 1 ~ 12 | `3` |
| [Year_0 (Int)](#year---年) | `/avatar/parameters/osc_clock@year_0`  | Int  | 0 ~ 9 | `2` |
| [Year_1 (Int)](#year---年) | `/avatar/parameters/osc_clock@year_1`  | Int  | 0 ~ 9 | `0` |
| [Year_2 (Int)](#year---年) | `/avatar/parameters/osc_clock@year_2`  | Int  | 0 ~ 9 | `1` |
| [Year_3 (Int)](#year---年) | `/avatar/parameters/osc_clock@year_3`  | Int  | 0 ~ 9 | `2` |

## パラメータの解説

### Second - 秒

Float型 (0 ~ 1) と Int型 (0 ~ 59) が利用できます。

| 種類 | パラメータのアドレス  | 型 | 範囲 |
| :- | - | - | - |
| [Second (Float)](#second---秒) | `/avatar/parameters/osc_clock@second_f`  | Float  | 0 ~ 1 |
| [Second (Int)](#second---秒) | `/avatar/parameters/osc_clock@second_i`  | Int  | 0 ~ 59 |

### Minute - 分

Float型 (0 ~ 1) と Int型 (0 ~ 59) が利用できます。

| 種類 | パラメータのアドレス  | 型 | 範囲 |
| :- | - | - | - |
| [Minute (Float)](#minute---分) | `/avatar/parameters/osc_clock@minute_f`  | Float  | 0 ~ 1 |
| [Minute (Int)](#minute---分) | `/avatar/parameters/osc_clock@minute_i`  | Int  | 0 ~ 59 |

### 時間 - Hour

#### 24 Hour - 24時間表記

Float型 (0 ~ 1) と Int型 (0 ~ 23) が利用できます。

| 種類 | パラメータのアドレス  | 型 | 範囲 |
| :- | - | - | - |
| [24 Hour (Float)](#24-hour---24時間表記) | `/avatar/parameters/osc_clock@hour24_f`  | Float  | 0 ~ 1 |
| [24 Hour (Int)](#24-hour---24時間表記) | `/avatar/parameters/osc_clock@hour24_i`  | Int  | 0 ~ 23 |


#### 12 Hour - 12時間表記

Float型で (0 ~ 1) と Int型 (0 ~ 11) が利用できます。

| 種類 | パラメータのアドレス  | 型 | 範囲 |
| :- | - | - | - |
| [12 Hour (Float)](#12-hour---12時間表記) | `/avatar/parameters/osc_clock@hour12_f`  | Float  | 0 ~ 1 |
| [12 Hour (Int)](#12-hour---12時間表記) | `/avatar/parameters/osc_clock@hour12_i`  | Int  | 0 ~ 11 |

#### 午前/午後 - AM/PM 

Bool型で、`True/False`のどちらかの値が利用できます。

| 　 | 値  |
| - | - |
| AM/午前 | `false` |
| PM/午後 | `true` |

| 種類 | パラメータのアドレス  | 型 | 範囲 |
| :- | - | - | - |
| [AM/PM (Bool)](#午前午後---ampm) | `/avatar/parameters/osc_clock@hour_isPM`  | Bool  | True or False |

### Day - 日

Int型で、`1 ~ 31`までの値が利用できます。

| 種類 | パラメータのアドレス  | 型 | 範囲 |
| :- | - | - | - |
| [Day (Int)](#day---日) | `/avatar/parameters/osc_clock@day`  | Int  | 1 ~ 31 |

### Day Of Week - 曜日

Int型で、`0 ~ 6`までの値が利用できます。

| 曜日 | 値  |
| - | - |
| 月 | `0` |
| 火 | `1` |
| 水 | `2` |
| 木 | `3` |
| 金 | `4` |
| 土 | `5` |
| 日 | `6` |

| 種類 | パラメータのアドレス  | 型 | 範囲 |
| :- | - | - | - |
| [Day Of Week (Int)](#day-of-week---曜日) | `/avatar/parameters/osc_clock@dofw`  | Int  | 0 ~ 6 (月曜日スタート) |

### Month - 月

Int型で、`1 ~ 12`までの値が利用できます。

| 種類 | パラメータのアドレス  | 型 | 範囲 |
| :- | - | - | - |
| [Month (Int)](#month---月) | `/avatar/parameters/osc_clock@month`  | Int  | 1 ~ 12 |

### Year - 年

Int型で、それぞれ`0 ~ 9`までの値が利用できます。

例を`2019`とするなら、4つのパラメータの値は `2`, `0`, `1`, `9` になります。

| 種類 | パラメータのアドレス  | 型 | 範囲 |
| :- | - | - | - |
| [Year_0 (Int)](#year---年) | `/avatar/parameters/osc_clock@year_0`  | Int  | 0 ~ 9 |
| [Year_1 (Int)](#year---年) | `/avatar/parameters/osc_clock@year_1`  | Int  | 0 ~ 9 |
| [Year_2 (Int)](#year---年) | `/avatar/parameters/osc_clock@year_2`  | Int  | 0 ~ 9 |
| [Year_3 (Int)](#year---年) | `/avatar/parameters/osc_clock@year_3`  | Int  | 0 ~ 9 |

## 同期について

これらのパラメータを全て同期すると、他のギミックの妨げになってしまいます。

そのため、[`osc_clock@Parameters`](./about_prefabs_in_Resources#internalosc_clockparameters)内の[`MA Parameters`](https://modular-avatar.nadena.dev/ja/docs/reference/parameters)から、使用するパラメータにのみ`同期する`をオンにし、それ以外はオフにするようにしてください。