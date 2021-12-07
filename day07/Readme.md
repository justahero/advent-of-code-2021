# Day 7

Advent Of Code site is offline, therefore the first half is taken from a different source.

## The task

Crab submarines all need to be aligned before they'll have enough power to blast a large enough hole for your submarine to get through.

* crab submarines can only move horizontally
* input is the list of horizontal positions of each crab submarine
* crab submarines have limited fuel
* goal: crab submarines in total need to spend as little fuel as possible

Example:

```
16,1,2,0,4,2,7,1,2,14

a crab with horizontal position: 16
a crab with horizontal position: 1
..
```

* each change of 1 step in horizontal position of a single crab costs `1` fuel
* in the example above: horizontal value 2 is the most efficient.

```
16 -> 2 = 14
 1 -> 2 = 1
 2 -> 2 = 0
 0 -> 2 = 2
 4 -> 2 = 2
 2 -> 2 = 0
 7 -> 2 = 5
 1 -> 2 = 1
 2 -> 2 = 0
14 -> 2 = 12
```

Total fuel: 37 (the cheapest possible outcome)

Can calculating the mean help here?
