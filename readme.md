
A wrapper library using [rquickjs](https://github.com/DelSkayn/rquickjs) and [svgo](https://github.com/svg/svgo) to compress svg in environments that do not support node and deno

## install

```bash
cargo binstall osvg
or
cargo install osvg --features=cli
```

## usage

```bash

osvg ./input.svg ./output.svg
```


## config

Pass the svgo configuration object as a string

```rust
let s = osvg(&svg, Some(r#"
{
  plugins: [
    {
      name: "preset-default",
      params: {
        overrides: {
          cleanupIds: false,
          inlineStyles: false,
          // minifyStyles: false,
        },
      },
    },
  ],
}
"#)).unwrap();
```

## perf
```bash
hyperfine --shell fish 'svgo ./bench.svg -o ./b.svg' 'osvg ./bench.svg ./b.svg'
```

```
Benchmark 1: svgo ./bench.svg -o ./b.svg
  Time (mean ± σ):     591.6 ms ±   9.9 ms    [User: 399.4 ms, System: 246.6 ms]
  Range (min … max):   579.1 ms … 603.5 ms    10 runs

Benchmark 2: osvg ./bench.svg ./b.svg
  Time (mean ± σ):      3.328 s ±  0.029 s    [User: 3.165 s, System: 0.126 s]
  Range (min … max):    3.282 s …  3.368 s    10 runs

Summary
  svgo ./bench.svg -o ./b.svg ran
    5.63 ± 0.11 times faster than osvg ./bench.svg ./b.svg
```