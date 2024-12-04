(local common {})

(fn common.table_print [t]
    (each [k v (pairs t)]
      (print (.. (.. k ":") v))))

(fn common.table_shallow_copy [t]
    (var copy [])
    (each [k v (pairs t)]
      (tset copy k v))
    copy)
  
(fn common.str_split_at [s i]
  (var left (string.sub 1 i))
  (var right (string.sub (+ i 1) -1))

  [left right]
)

(fn common.str_split_on [s p]
  (print (string.find s p))
  (common.str_split_at s (string.find s p))
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

common
