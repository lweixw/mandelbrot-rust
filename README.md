# mandelbrot-rust
This repository is to test the CPU performance by timing the mandelbrot benchmark game. The output is an image.

## Speed Test Results

### Apple M1 Ultra

```shell
time ./target/release/mandelbrot 32000 > output.jpeg
./target/release/mandelbrot 32000 > output.txt  14.76s user 0.13s system 1729% cpu 0.861 total
```

### Apple M1 Max

```shell
time ./target/release/mandelbrot 32000 > output.jpeg
./target/release/mandelbrot 32000 > output.txt  14.27s user 0.16s system 848% cpu 1.700 total
```

### Apple M1

```shell

```

### Intel 

```shell

```
