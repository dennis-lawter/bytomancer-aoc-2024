(let [input _G.input]
  (fn table_print [t]
    (each [k v (pairs t)]
      (print (.. (.. k ":") v))))
  
  (fn set_contains [t key]
    (not= nil (. t key))
  )

  (fn is_allowed [line i increasing]
    (let [allowed [1 2 3]
    curr (. line i)
    prev (. line (- i 1))]
      (if
        (= nil curr) 
        true
        (set_contains allowed (if increasing (- curr prev) (- prev curr)))
        (is_allowed line (+ i 1) increasing)
        false
      )
    )
  )

  (fn test_safe [line]
    (var increasing (> (. line 2) (. line 1)))
    (is_allowed line 2 increasing)
  )

  (var answer 0)
  (each [i line (pairs input)]
    (set answer (if (test_safe line) (+ answer 1) answer))
  )

  answer)
