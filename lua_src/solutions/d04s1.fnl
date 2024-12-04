(let [input _G.input]
  (local common (require :lua_src.common))
  (var answer 0)

  (fn find_xmas_up_left [x y input]
    (var ans false)
    (if
      (= (common.lookup_2d x y input) "X")
      (if
        (= (common.lookup_2d (- x 1) (- y 1) input) "M")
        (if
          (= (common.lookup_2d (- x 2) (- y 2) input) "A")
          (if
            (= (common.lookup_2d (- x 3) (- y 3) input) "S")
            (set ans true)
          )
        )
      )
    )
    ans
  )

  (fn find_xmas_up_right [x y input]
    (var ans false)
    (if
      (= (common.lookup_2d x y input) "X")
      (if
        (= (common.lookup_2d (+ x 1) (- y 1) input) "M")
        (if
          (= (common.lookup_2d (+ x 2) (- y 2) input) "A")
          (if
            (= (common.lookup_2d (+ x 3) (- y 3) input) "S")
            (set ans true)
          )
        )
      )
    )
    ans
  )

  (fn find_xmas_up [x y input]
    (var ans false)
    (if
      (= (common.lookup_2d x y input) "X")
      (if
        (= (common.lookup_2d x (- y 1) input) "M")
        (if
          (= (common.lookup_2d x (- y 2) input) "A")
          (if
            (= (common.lookup_2d x (- y 3) input) "S")
            (set ans true)
          )
        )
      )
    )
    ans
  )

  (fn find_xmas_left [x y input]
    (var ans false)
    (if
      (= (common.lookup_2d x y input) "X")
      (if
        (= (common.lookup_2d (- x 1) y input) "M")
        (if
          (= (common.lookup_2d (- x 2) y input) "A")
          (if
            (= (common.lookup_2d (- x 3) y input) "S")
            (set ans true)
          )
        )
      )
    )
    ans
  )

  (fn find_xmas_right [x y input]
    (var ans false)
    (if
      (= (common.lookup_2d x y input) "X")
      (if
        (= (common.lookup_2d (+ x 1) y input) "M")
        (if
          (= (common.lookup_2d (+ x 2) y input) "A")
          (if
            (= (common.lookup_2d (+ x 3) y input) "S")
            (set ans true)
          )
        )
      )
    )
    ans
  )

  (fn find_xmas_down_left [x y input]
    (var ans false)
    (if
      (= (common.lookup_2d x y input) "X")
      (if
        (= (common.lookup_2d (- x 1) (+ y 1) input) "M")
        (if
          (= (common.lookup_2d (- x 2) (+ y 2) input) "A")
          (if
            (= (common.lookup_2d (- x 3) (+ y 3) input) "S")
            (set ans true)
          )
        )
      )
    )
    ans
  )

  (fn find_xmas_down_right [x y input]
    (var ans false)
    (if
      (= (common.lookup_2d x y input) "X")
      (if
        (= (common.lookup_2d (+ x 1) (+ y 1) input) "M")
        (if
          (= (common.lookup_2d (+ x 2) (+ y 2) input) "A")
          (if
            (= (common.lookup_2d (+ x 3) (+ y 3) input) "S")
            (set ans true)
          )
        )
      )
    )
    ans
  )

  (fn find_xmas_down [x y input]
    (var ans false)
    (if
      (= (common.lookup_2d x y input) "X")
      (if
        (= (common.lookup_2d x (+ y 1) input) "M")
        (if
          (= (common.lookup_2d x (+ y 2) input) "A")
          (if
            (= (common.lookup_2d x (+ y 3) input) "S")
            (set ans true)
          )
        )
      )
    )
    ans
  )

  (fn find_xmas [x y input]
    (var ans 0)
    (var funcs [find_xmas_up_left find_xmas_up_right find_xmas_up find_xmas_left find_xmas_right find_xmas_down_left find_xmas_down_right find_xmas_down])
    (each [_ func (pairs funcs)]
      (if
        (func x y input)
        (set answer (+ answer 1))
      )
    )
    ans
  )

  (each [y row (pairs input)]
    (each [x char (pairs row)]
      (set answer (+ answer (find_xmas x y input)))
    )
  )

  answer)
