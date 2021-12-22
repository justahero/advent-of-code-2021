# Day 19

## 1st solution

* beacons and scanners
* each scanner is capable of detecting all beacons in an area
  * scanner is in the center
  * beacons are at most 1000 units away in each of three axes
* beacon's precise position determined relative to the scanner
* scanners do not detect other scanners
* reports are beacons
* scanner reports all beacon positions relative to it
  * scanners do not know their positions
* scanners do not know their rotation / or facing direction
  * 24 possible combinations of alignment
  * alignment changes beacon points all at the same time, same way
* a scanner has N beacons (for example 1st scanner)
  * other beacons may share the same beacons

Goal is to find positions of all beacons / scanners: Ho many beacons are there?

Reconstruct by finding pairs of **scanners** that have overlapping regions, such that there are at least **12** beacons that both scanners detect within the overlap.
By establishing 12 common beacons, it's possibleto precisely determine where scanners are relative to each other.

Example in 2D:

```
// input
--- scanner 0 ---
0,2
4,1
3,3

--- scanner 1 ---
-1,-1
-5,0
-2,1
```

```
...B.  Scanner 0 at 0,0
B....
....B
S....

...B..  Scanner 1 at 0,0
B....S
....B.

Overlapping area

...B..
B....S
....B.
S.....
```

There are 24 possible alignments, given the coordinates: `x: 1, y: 2, 3: 3` these are:

```
( 1,  2,  3)
( 1, -2, -3)
( 1,  3, -2)
( 1, -3,  2)

(-1,  2, -3)
(-1, -2,  3)
(-1,  3,  2)
(-1, -3, -2)

( 2,  1, -3)
( 2, -1,  3)
( 2,  3,  1)
( 2, -3, -1)

(-2,  1,  3)
(-2, -1, -3)
(-2,  3, -1)
(-2, -3,  1)

( 3,  1,  2)
( 3, -1, -2)
( 3,  2, -1)
( 3, -2,  1)

(-3,  1, -2)
(-3, -1,  2)
(-3,  2,  1)
(-3, -2, -1)
```

Two scanners with overlap, how to determine these?

```
--- scanner 0 ---   --- scanner 1 ---
 404,-588,-901       686, 422, 578
 528,-643, 409       605, 423, 415
-838, 591, 734       515, 917,-361
 390,-675,-793      -336, 658, 858
-537,-823,-458        95, 138,  22
-485,-357, 347      -476, 619, 847
-345,-311, 381      -340,-569,-846
-661,-816,-575       567,-361, 727
-876, 649, 763      -460, 603,-452
-618,-824,-621       669,-402, 600
 553, 345,-567       729, 430, 532
 474, 580, 667      -500,-761, 534
-447,-329, 318      -322, 571, 750
-584, 868,-557      -466,-666,-811
 544,-627,-890      -429,-592, 574
 564, 392,-477      -355, 545,-477
 455, 729, 728       703,-491,-529
-892, 524, 684      -328,-685, 520
-689, 845,-530       413, 935,-424
 423,-701, 434      -391, 539,-444
   7, -33, -71       586,-435, 557
 630, 319,-379      -364,-763,-893
 443, 580, 662       807,-499,-711
-789, 900,-551       755,-354,-619
 459,-707, 401       553, 889,-390

The following beacons are the same, in the same order

-618,-824,-621       686, 422, 578
-537,-823,-458       605, 423, 415
-447,-329, 318       515, 917,-361
 404,-588,-901      -336, 658, 858
 544,-627,-890      -476, 619, 847
 528,-643, 409      -460, 603,-452
-661,-816,-575       729, 430, 532
 390,-675,-793      -322, 571, 750
 423,-701, 434      -355, 545,-477
-345,-311, 381       413, 935,-424
 459,-707, 401      -391, 539,-444
-485,-357, 347       553, 889,-390
```
