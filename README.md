### [[English Here]](README.en.md)

# ssbu-better-latency-slider

[latency-slider-de](https://github.com/Naxdy/latency-slider-de)の調整版．

## 操作

VIPのキャラ選択画面または専用部屋の待機画面で，十字キーを押して遅延を変更します．  
このとき，画面上に現在の遅延などは表示されません．

| 入力 | レイテンシ |
| --- | --- |
| 左 | 0F |
| 上 | 1F |
| 右 | 2F |
| 下 | 3F |

## 設定

初回起動時に`sd:/config/ssbu-better-latency-slider/config.txt`が生成されます．

```
default_latency=2
```

`default_latency`はゲーム起動時に適用される初期値 (0-3) で，この値を書き換えることで初期値を変更できます．  
デフォルトで2Fになっています．

## ビルド

```shell
cargo skyline build --release
```

### 初期値固定版

十字キーを無効化し，レイテンシを特定の値に固定することができます (実際にビルドしたものがそれぞれリリースにあります)．  
`0f`，`1f`，`2f`，`3f`のいずれかでfeatureを指定するか，`build.ps1`を使うと，初期値固定版を`_2f`のような接尾辞付きの名前で出力します．

```powershell
.\build.ps1 2f       # dist/libssbu_better_latency_slider_2f.nro
.\build.ps1          # 通常版+固定版4種すべて
```

featureを直接指定した場合 (例: `cargo skyline build --release --features 2f`) は，cargo-skylineに出力名を変えるオプションがないため通常と同じ`libssbu_better_latency_slider.nro`が出力されます．

## ライセンス

[LICENSE](LICENSE)

## 謝辞

- Naxdy ([latency-slider-de](https://github.com/Naxdy/latency-slider-de))