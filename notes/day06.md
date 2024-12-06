# Day 06

## Optimizing for p2
- I did use O for the inserted obstacle, and my algorithm recognizes # and O.
- Brute force solution took 77.46s in debug mode.
    - One small optimization I used: not replacing a # with an O.
- Clearly creating cloned permutations is not sufficient.
- So far I can only think of optimizations:
    - Memory reallocation
        - reset the guard
        - unset the O with a .
    - Avoid useless permutations
        - First calculate the hashset of visited squares (direction not important).
        - Now only replace those positions.
        - This works because you can only add 1 new obstacle, so it must be on the original path.
