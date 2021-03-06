# mandelbrot-rust
This repository is to test the CPU performance by timing the mandelbrot benchmark game. The output is an image.

## Speed Test Results on rust 1.59.0

### Apple M1 Ultra - Power Connected ( No battery option, :-) )

```shell
time ./target/release/mandelbrot 32000 > output.jpeg
./target/release/mandelbrot 32000 > output.txt  14.76s user 0.13s system 1729% cpu 0.861 total
```

### Apple M1 Max - Performance Mode Automatic on battery

```shell
time ./target/release/mandelbrot 32000 > output.jpeg
./target/release/mandelbrot 32000 > output.txt  14.27s user 0.16s system 848% cpu 1.700 total
```

### Apple M1 - Normal Power Connected

```shell
time ./target/release/mandelbrot 32000 > output.jpeg
./target/release/mandelbrot 32000 > output.jpeg  18.42s user 0.21s system 746% cpu 2.497 total
```


### Apple M1 - Low Power Mode on battery

```shell
time ./target/release/mandelbrot 32000 > output.jpeg
./target/release/mandelbrot 32000 > output.jpeg  26.71s user 0.26s system 750% cpu 3.595 total
```

### Intel "Skylake" 2.9 GHz Intel "Core i7" processor (6920HQ) Power Connected

```shell
time ./target/release/mandelbrot 32000 > output.jpeg
./target/release/mandelbrot 32000 > output.jpeg  24.00s user 0.27s system 732% cpu 3.314 total
```

### Intel "Coffee Lake" 1.7 GHz Intel "Core i7" processor (I7-8557U) Power Connected

```shell
time ./target/release/mandelbrot 32000 > output.jpeg
./target/release/mandelbrot 32000 > output.jpeg  20.82s user 0.24s system 731% cpu 2.877 total
```
