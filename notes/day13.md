# Day 13

Part 2 is brutal.

Observations:
- What if the prize location contains a prime?
    - Possible
    - No factor things make sense then
- What if A gains 100x more "ground" than B?
    - see EX1
    - My naive approach initially was to favor B presses then trade them for As.
- What if A and B share the same "ratio"?
    - In the above example, both cause the claw to travel at 45 deg.
    - Obviously then, the prize must be located at 45 deg.
    - When A and B are not the same (most cases) you cannot target the prize this way.
- My current solution is to favor all B presses, then trade for As.
    - The downside to this is that it takes _forever_ to find a good answer.
    - And frankly, much of that time is spent off in the wrong "degree".
    - see EX2
    - Here we can see the Bs have led us too far to the "east"
    - We travel along some sort of "radius" CCW
    - Eventually we hit the goal in this scenario
    - Then we continue traveling past
    - And this won't solve the A:B ratio problem.
    - But maybe the A:B ratio problem is a special case.

## Examples

### EX1
```
Button A: X+100, Y+100
Button B: X+1, Y+1
Prize: X=100, Y=100
```

### EX2
```
GOAL:(10000000018641, 10000000010279)
Starting at             A:0, B:140845070568, CLAW: (3802816905336, 10000000010328)
Now at          A:2608696100, B:140000000000, CLAW: (3960000030900, 10000000010300)
Now at          A:33478261317, B:130000000000, CLAW: (5820000030873, 10000000010291)
Now at          A:64347826534, B:120000000000, CLAW: (7680000030846, 10000000010282)
Now at          A:95217391752, B:110000000000, CLAW: (9540000030888, 10000000010296)
FOUND ONE!  A: 102851800151     B: 107526881786 cost: 416082282239
Now at          A:105797101720, B:100000000000, CLAW: (10000000018680, 9533333339560)
Now at          A:109710145198, B:90000000000, CLAW: (10000000018662, 8913333339554)
```

### EX3
```
I believe this input:

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Yields this answer:

459236326669

FOUND ONE!  A: 118679050709     B: 103199174542 cost: 459236326669
```

## WRONG ANSWERS
- 85707154934703 too low
