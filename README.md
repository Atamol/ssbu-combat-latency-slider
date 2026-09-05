### ![ja](https://flagcdn.com/20x15/jp.png) [日本語版はこちら](README.ja.md)

# ssbu-combat-latency-slider

An adjusted version of [latency-slider-de](https://github.com/Naxdy/latency-slider-de) (13.0.5 compatible).

Compared to the original: no on-screen display at all, 0-3F instead of 0 to 25F, and builds with the d-pad locked to a single value.

## Controls

On the Elite Smash character select screen or the arena waiting room, press the d-pad to change the delay. The current delay is not shown on screen.

| Input | Latency |
| ----- | ------- |
| Left  | 0F      |
| Up    | 1F      |
| Right | 2F      |
| Down  | 3F      |

## Config

`sd:/config/ssbu-combat-latency-slider/config.txt` is generated on first launch.

```
default_latency=2
```

`default_latency` is the value applied at startup (0-3). Edit it to change the default. It is 2F by default.

## Build

```shell
cargo skyline build --release
```

### Locked builds

You can disable the d-pad and lock the latency to a fixed value. Prebuilt ones are on the releases page. Pass one of the `0f`, `1f`, `2f`, `3f` features, or use `build.ps1`, to output the locked build with a `_2f` style suffix.

```powershell
.\build.ps1 2f       # dist/libssbu_combat_latency_slider_2f.nro
.\build.ps1          # dpad version plus all four locked builds
```

Passing the feature directly (e.g. `cargo skyline build --release --features 2f`) outputs the usual `libssbu_combat_latency_slider.nro`, since cargo-skyline has no option to change the output name.

## License

[LICENSE](LICENSE) (AGPL-3.0)

## Credit

- Naxdy ([latency-slider-de](https://github.com/Naxdy/latency-slider-de))
- [HewDraw-Remix](https://github.com/HDR-Development/HewDraw-Remix) team
