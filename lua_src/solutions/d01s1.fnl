(let [input _G.input]
  (local common (require :lua_src.common))

  (fn setup_lists [l r pair]
    (common.table_print pair)
    (tset l (+ (length l) 1) (. pair 1))
    (tset r (+ (length r) 1) (. pair 2)))

  (fn get_dist [l r]
    (if (< l r) (- r l)
        (= l r) 0
        (- l r)))

  (var answer 0)
  (let [l []
        r []]
    (each [_ line (pairs input)]
      (let [t (icollect [token (string.gmatch line "[^%s]+")]
                (tonumber token))]
        (setup_lists l r t)))
    (table.sort l)
    (table.sort r)
    (each [k lv (pairs l)]
      (let [rv (. r k)]
        (set answer (+ answer (get_dist lv rv))))))
  answer)
