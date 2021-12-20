# Day 17

Directly calculate min & max velocity x.

* example: `target x=20..30`
* each `x` value is added with `vel_x`
* goal is to go `up` as high as possible
* final `x` should not exceed `20..30`

Seems like a sum of first n natural numbers is fine here:

```
(n * (n + 1)) / 2 = x, where x is between left..right
```

```rust
(n * (n + 1)) / 2 = 20  // * 2
(n * (n + 1)) = 40
n^2 + n - 40 = 0
```

