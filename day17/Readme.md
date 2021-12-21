# Day 17

## 1st solution

Directly calculate min & max velocity x.

* example: `target x=20..30`
* each `x` value is added with `vel_x`
* goal is to go `up` as high as possible
* final `x` should not exceed `20..30`

Seems like a sum of first n natural numbers is fine here:

```
(n * (n + 1)) / 2 = x, where x is between left..right
```

```
(n * (n + 1)) / 2 - ((n + 1) * (n + 2)) / 2 = x  // ?

Example:

n = 9

(9 * (9 + 1)) / 2 - ((9 + 1) * (9 + 2)) / 2 = x
90 / 2 - 110 / 2 = x
45 - 55 = -10
```

```
(n * (n + 1)) / 2 - ((n + 1) * (n + 2) / 2) = x
(n^2 + n) / 2 - (n^2 + n + 2n + 2) / 2 = x
(n^2 + n) - (n^2 + 3n + 2) = 2x
n^2 + n - n^2 - 3n - 2 = 2x
n - 3n - 2 = 2x
-2n - 2 = 2x
n - 1 = x
n = abs(x + 1)
```
