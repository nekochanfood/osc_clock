# osc_clock
Send date and time to your VRChat avatar via OSC

## Available units
Example DateTime: `2012/03/14 12:34:56`
| Unit | Parameter Address  | Type | Range | Example Value |
| :- | - | - | - | :- |
| Second (Float) | `/avatar/parameters/second_f`  | Float  | 0 ~ 1 | `0.933333` |
| Second (Int) | `/avatar/parameters/second_i`  | Int  | 0 ~ 59 | `56` |
| Minute (Float) | `/avatar/parameters/minute_f`  | Float  | 0 ~ 1 | `0.566666` |
| Minute (Int) | `/avatar/parameters/minute_i`  | Int  | 0 ~ 59 | `34` |
| 24 Hour (Float) | `/avatar/parameters/hour24_f`  | Float  | 0 ~ 1 | `0.5` |
| 24 Hour (Int) | `/avatar/parameters/hour24_i`  | Int  | 0 ~ 23 | `12` |
| 12 Hour (Float) | `/avatar/parameters/hour12_f`  | Float  | 0 ~ 1 | `0` |
| 12 Hour (Int) | `/avatar/parameters/hour12_i`  | Int  | 0 ~ 11 | `0` |
| isPM (Bool) | `/avatar/parameters/hour12_i`  | Bool  | True or False | `True` |
| Day (Int) | `/avatar/parameters/day`  | Int  | 1 ~ 31 | 14 |
| Weekday (Int) | `/avatar/parameters/dofw`  | Int  | 0 ~ 6 (Starts from monday) | `2` |
| Month (Int) | `/avatar/parameters/month`  | Int  | 1 ~ 12 | `3` |
| Year (Int) | `/avatar/parameters/year`  | Int  | 0 ~ ? | `2012` |
| Year_0 (Int) | `/avatar/parameters/year_0`  | Int  | 0 ~ 9 | `2` |
| Year_1 (Int) | `/avatar/parameters/year_1`  | Int  | 0 ~ 9 | `0` |
| Year_2 (Int) | `/avatar/parameters/year_2`  | Int  | 0 ~ 9 | `1` |
| Year_3 (Int) | `/avatar/parameters/year_3`  | Int  | 0 ~ 9 | `2` |

## Advice on use

These parameters are set to be sent every second by default.

To reduce the transmission load on the VRChat side, it is recommended to turn off parameter synchronisation.

If you still want to synchronise, it is recommended to set the rate to 60x (send every minute) from config.json.

## Troubleshooting
Use `.\osc_clock.exe repair` to repair `.\config.json`.
