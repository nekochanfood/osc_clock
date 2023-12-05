# OSC Clock
Send date and time to VRChat via OSC

## Available units
Example DateTime: `2012/03/14 12:34:56`
| Unit | Parameter Address  | Type | Range | Example Value |
| :- | - | - | - | :- |
| Second (Float) | `/avatar/parameters/osc_clock@second_f`  | Float  | 0 ~ 1 | `0.933333` |
| Second (Int) | `/avatar/parameters/osc_clock@second_i`  | Int  | 0 ~ 59 | `56` |
| Minute (Float) | `/avatar/parameters/osc_clock@minute_f`  | Float  | 0 ~ 1 | `0.566666` |
| Minute (Int) | `/avatar/parameters/osc_clock@minute_i`  | Int  | 0 ~ 59 | `34` |
| 24 Hour (Float) | `/avatar/parameters/osc_clock@hour24_f`  | Float  | 0 ~ 1 | `0.5` |
| 24 Hour (Int) | `/avatar/parameters/osc_clock@hour24_i`  | Int  | 0 ~ 23 | `12` |
| 12 Hour (Float) | `/avatar/parameters/osc_clock@hour12_f`  | Float  | 0 ~ 1 | `0.0472221666666667` |
| 12 Hour (Int) | `/avatar/parameters/osc_clock@hour12_i`  | Int  | 0 ~ 11 | `0` |
| isPM (Bool) | `/avatar/parameters/osc_clock@hour_isPM`  | Bool  | True or False | `True` |
| Day (Int) | `/avatar/parameters/osc_clock@day`  | Int  | 1 ~ 31 | `14` |
| Weekday (Int) | `/avatar/parameters/osc_clock@dofw`  | Int  | 0 ~ 6 (Starts from monday) | `2` |
| Month (Int) | `/avatar/parameters/osc_clock@month`  | Int  | 1 ~ 12 | `3` |
| Year_0 (Int) | `/avatar/parameters/osc_clock@year_0`  | Int  | 0 ~ 9 | `2` |
| Year_1 (Int) | `/avatar/parameters/osc_clock@year_1`  | Int  | 0 ~ 9 | `0` |
| Year_2 (Int) | `/avatar/parameters/osc_clock@year_2`  | Int  | 0 ~ 9 | `1` |
| Year_3 (Int) | `/avatar/parameters/osc_clock@year_3`  | Int  | 0 ~ 9 | `2` |

## Advice on use

These parameters are set to be sent every second by default.

To reduce the transmission load on the VRChat side, it is recommended to turn off parameter synchronisation.

If you still want to synchronise, increase the existing rate in config.json by a factor of 10 to 60 (send every 10 seconds to 1 minute).

## Troubleshooting

Use `.\osc_clock.exe repair` to repair `.\config.json`.
