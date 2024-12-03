(let [input _G.input]
  (fn table_print [t]
    (each [k v (pairs t)]
      (print (.. (.. k ":") v))))

  (fn set_contains [t key]
    (not= nil (. t key)))

  (fn is_allowed [line i increasing]
    (let [allowed [1 2 3]
          curr (. line i)
          prev (. line (- i 1))]
      (if (= nil curr) true
          (set_contains allowed (if increasing (- curr prev) (- prev curr))) (is_allowed line
            (+ i
              1)
            increasing)
          false)))

  (fn test_safe [line]
    (var increasing (> (. line 2) (. line 1)))
    (is_allowed line 2 increasing))

  (fn table_shallow_copy [t]
    (var copy [])
    (each [k v (pairs t)]
      (tset copy k v))
    copy)

  (fn copied_table_with_missing_key [t key]
    (var t_copy (table_shallow_copy t))
    (table.remove t_copy key)
    t_copy)

  (fn test_safe_with_perms [line i]
    (if (> i (length line)) false
        (test_safe (copied_table_with_missing_key line i)) true
        (test_safe_with_perms line (+ i 1))))

  (var answer 0)
  (each [_ line (pairs input)]
    (set answer (if (test_safe line) (+ answer 1)
      (test_safe_with_perms line 1) (+ answer 1)
      answer)))
  answer)
