(local common {})

(fn common.table_print [t]
    (each [k v (pairs t)]
      (print (.. (.. k ":") v))))

(fn common.table_shallow_copy [t]
    (var copy [])
    (each [k v (pairs t)]
      (tset copy k v))
    copy)

common
