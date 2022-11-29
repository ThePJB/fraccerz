# Fraccerz

Mandelbrot-esque fractal program. Muck around with poles and zeros, its fun.

- shift click to zoom in
- ctrl click to zoom out
- consider making the window small while you explore

### Building
`cargo run --release`

### Samples

![plz work](f1.png)

$$ z = \frac{z^2}{(z + 0.01i)(z - 0.02i)} + c$$

![plz work](f2.png)

$$ z = \frac{z^2}{(z + \frac{1}{\sqrt{2}i})(z - \frac{1}{\sqrt{2}}i)} + c$$

### TODO
- antialiased screenshot taker
- SIMD
- radix option might make it faster?? if the overhead of the mpmc is stupid
- storing what z got up to? maybe faster