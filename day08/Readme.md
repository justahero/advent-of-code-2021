# Day 08

Figure out how the 7-segment display works:
Each digit of a 7-segment display is represented by turning on / off specific segments:

```
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
```

In working mode the following segments are turned on to display a digit from `0` to `9`


| digit |   segments    | unique |  # |
|-------|--------------:|-------:|---:|
|     0 | a b c e f g   |        |  6 |
|     1 | c f           |      y |  2 |
|     2 | a c d e g     |        |  5 |
|     3 | a c d f g     |        |  5 |
|     4 | b c d f       |      y |  4 |
|     5 | a b d f g     |        |  5 |
|     6 | a b e f g     |        |  5 |
|     7 | a c f         |      y |  3 |
|     8 | a b c d e f g |      y |  7 |
|     9 | a b c d f g   |        |  6 |

Start:

* signals that control the segments have been mixed up on each display
* wires are connected to segments randomly
* wire / segment connections are mixed up for each four digit display.
* all of the digits within a display use the same connections
* 4 digits after `|` are in the same display, use the 

## Example:

Single entry after observation:

```
acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
```

* 10 unique signal patterns, then a four digit output value

```
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
```

Deduction of finding the right rewiring.
Given the following input:

```
acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab
```

leads to solution:

```
  0000             3333
 1    2           4    0
 1    2           4    0
  3333     -->     5555
 4    5           6    1
 4    5           6    1
  6666             2222
```

How is this determined?

* 2 bits => digit 1
* 4 bits => digit 4
