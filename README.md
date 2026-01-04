# OSC Clock

Send date and time to VRChat via OSC

## Advice on Use

These parameters are set to be sent every second by default.

To reduce the transmission load on the VRChat side, it is recommended to turn off parameter synchronisation in VRChat.

If you still want to synchronise, increase the `check_rate_ms` value in `config.json` by a factor of 10 to 60 (send every 10 seconds to 1 minute).

## License

See [LICENSE](LICENSE) for details.

## Troubleshooting

Use `.\osc_clock.exe --repair` to repair/regenerate the config file.

## Build

To build osc_clock (like previously):
```
cargo build 
```

To build the osc_clock app:
```
cd packages/app
bunx tauri build
```