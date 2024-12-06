(local common {})

(fn common.table_print [t]
    (print "[")
    (each [k v (pairs t)]
      (print (.. (.. k ":") v)))
      (print "]"))

(fn common.table_shallow_copy [t]
    (var copy [])
    (each [k v (pairs t)]
      (tset copy k v))
    copy)
  
(fn common.str_split_at [s i]
  (var left (string.sub s 1 i))
  (var right (string.sub s (+ i 1) -1))

  [left right]
)

(fn common.str_split_on [s p]
  (var i (string.find s p))

  ; must accomodate for pattern length
  (var left (string.sub s 1 (- i 1)))
  (var right (string.sub s (+ i (length p)) -1))

  [left right]
)

(fn common.lookup_2d [x y input]
  (var h (length input))
  (var w (length (. input 1)))
  (if
    (and (and (>= y 1) (<= y h)) (and (>= x 1) (<= x w)))
    (. (. input y) x)
    nil
  )
)

(fn common.append_to_table_end [t ins]
  (tset t (+ 1 (length t)) ins)
)

(fn common.collect_split_string_on [s p]
  (local gpat (.. "[^" p "]+"))
  (var out [])
  (each [token (string.gmatch s gpat)]
    (common.append_to_table_end out token)
  )
  out
)

common
