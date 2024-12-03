(let [input _G.input]
  (local common (require :lua_src.common))

  (fn setup_lists [l r pair]
    (common.table_print pair)
    (tset l (+ (length l) 1) (. pair 1))
    (tset r (+ (length r) 1) (. pair 2)))

  (fn get_counts [lv r]
    (var ans 0)
    (each [_ rv (pairs r)]
      (set ans (+ ans (if (= lv rv) lv 0))))
    ans)

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
        (set answer (+ answer (get_counts lv r))))))
  answer)
