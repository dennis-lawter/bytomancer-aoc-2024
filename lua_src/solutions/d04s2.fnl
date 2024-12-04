(let [input _G.input]
  (local common (require :lua_src.common))
  (var answer 0)

  (fn confirm_xmas [center tl tr bl br]
    (local tlbr (.. (.. tl center) br))
    (local bltr (.. (.. bl center) tr))
    
    (case [tlbr bltr]
      ["SAM" "SAM"] true
      ["MAS" "SAM"] true
      ["SAM" "MAS"] true
      ["MAS" "MAS"] true
      _ false
    )
  )

  (fn is_xmas [x y input]
    (local center (common.lookup_2d x y input))
    (local tl? (common.lookup_2d (- x 1) (- y 1) input))
    (local tr? (common.lookup_2d (+ x 1) (- y 1) input))
    (local bl? (common.lookup_2d (- x 1) (+ y 1) input))
    (local br? (common.lookup_2d (+ x 1) (+ y 1) input))

    (case [tl? tr? bl? br?]
      [tl tr bl br] (confirm_xmas center tl tr bl br)
      _ false
    )
    
  )

  (fn find_xmas [x y input]
    (if
      (is_xmas x y input) 1
      0
    )
  )

  (each [y row (pairs input)]
    (each [x char (pairs row)]
      (set answer (+ answer (find_xmas x y input)))
    )
  )

  answer)
