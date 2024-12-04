(let [input _G.input]
  (local common (require :lua_src.common))

  (fn starts_with [s p]
    (local sub (string.sub s 1 (length p)))
    (= sub p))

  (fn split_out_digits [s]
    (var out [])
    (each [v (string.gmatch (string.sub s 4 -2) "(%d+)")]
      (tset out (+ (length out) 1) v))
    out)

  (fn mul_val [minput]
    (var digits (split_out_digits minput))
    (var ans (* (tonumber (. digits 1)) (tonumber (. digits 2))))
    ans)

  (fn is_mul_statement [minput]
    (not= nil (string.match minput "^mul%(%d+,%d+%)")))

  (var answer 0)
  (var en true)
  (var minput input)
  (while (not= 0 (length minput))
    (if (starts_with minput "don't()")
        (set en false)
        (starts_with minput "do()")
        (set en true)
        (and (and en (starts_with minput "mul(")) (is_mul_statement minput))
        (set answer (+ answer (mul_val minput))))
    (set minput (string.sub minput 2 -1)))
  answer)
