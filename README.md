# OSC Clock
Send date and time to VRChat via OSC

## Available units
Check [here](https://osc-clock.chanfoo.net/docs/creators/parameters)

## Advice on use

These parameters are set to be sent every second by default.

To reduce the transmission load on the VRChat side, it is recommended to turn off parameter synchronisation.

If you still want to synchronise, increase the existing rate in config.json by a factor of 10 to 60 (send every 10 seconds to 1 minute).

## Troubleshooting

Use `.\osc_clock.exe repair` to repair `.\config.json`.
