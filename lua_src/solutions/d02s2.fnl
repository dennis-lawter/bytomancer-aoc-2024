(let [input _G.input]
  (local common (require :lua_src.common))
  
  (fn set_contains [t key]
    (not= nil (. t key)))

  (fn is_allowed [line i increasing]
    (let [allowed [1 2 3]
          curr (. line i)
          prev (. line (- i 1))]
      (if
        (= nil curr)
        true
        (set_contains allowed (if increasing (- curr prev) (- prev curr)))
        (is_allowed line (+ i 1) increasing)
        false)))

  (fn test_safe [line]
    (var increasing (> (. line 2) (. line 1)))
    (is_allowed line 2 increasing))

  (fn copied_table_with_missing_key [t key]
    (var t_copy (common.table_shallow_copy t))
    (table.remove t_copy key)
    (common.table_print t_copy)
    (print "")
    t_copy)

  (fn test_safe_with_perms [line i]
    (if
      (> i (length line)) false
      (test_safe (copied_table_with_missing_key line i)) true
      (test_safe_with_perms line (+ i 1))))

  (var answer 0)
  (each [_ line (pairs input)]
    (set answer (if (test_safe line) (+ answer 1)
      (test_safe_with_perms line 1) (+ answer 1)
      answer)))
  answer)
