# Day 05

(sorry this is probably better viewed raw)

Revision goals

In theory, the ordering rules can produce a ranked list.

```
97|13
97|61
61|13
```

using X|Y notation

97 occurs only as X, so it is always first
13 occurs only as Y, so it is always last
61 then fits between 97 and 13

Thus we can produce a rank stack:
97 61 13

Once we have a guaranteed first number, we can actually ignore it.

```
97|13
97|61
75|29
61|13
29|13
97|29
61|29
97|75
75|61
75|13
```

97...


```
75|29
61|13
29|13
61|29
75|61
75|13
```

Now only 75 appears on the left, so it must be next

97 75...

```
61|13
29|13
61|29
```

Now only 61 appears on the left, so it must be next

97 75 61...

repeat until the list is a single [(X|Y)] then push X then Y onto this vector.

97 75 61 29 13

so now you've reduced order to a guaranteed ranking...
how can this become a "verify" function?

```rust
fn validate(ord: &Vec<u64>, upd: &Vec<u64>) -> bool {
    let mut i = 0;
    for o in ord {
        let new_i = upd.iter().position(o);
        if new_i < i {
            return false;
        } else {
            i = new_i;
        }
    }
    true
}
```

there is a somewhat simple approach to produce a fixed list...
copy ord, then remove all numbers not present in upd
alternatively, produce a new upd out of each number in ord only if it appears in the original upd

```rust
fn produce_fixed_upd(ord: &Vec<u64>, upd: &Vec<u64>) -> Vec<u64> {
    let mut out = vec![];
    let mut ord = ord.clone();

    while let next_ord = ord.pop() {
        if upd.contains(next_ord) {
            out.push(next_ord);
        }
    }

    out
}
```
