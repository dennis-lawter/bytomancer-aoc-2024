(let [input _G.input]
  ; (fn )

  (local common (require :lua_src.common))
  (var answer 0)

  ; parse inputs into ordering_raw and updates_raw
  (var ordering_raw [])
  (var updates_raw [])
  (each [_ in (pairs input)]
    (
      if
      (string.find in "|")
      (common.append_to_table_end ordering_raw (common.str_split_on in "|"))
      (string.find in ",")
      (common.append_to_table_end updates_raw (common.collect_split_string_on in ","))
    )
  )

  (each [_ ord (pairs ordering_raw)]
    (each [_ o_i (pairs ord)]
      (print o_i)
    )
    (print "")
  )

  (print "=====")
    (print "")

  (each [_ upd (pairs updates_raw)]
    (each [_ u_i (pairs upd)]
      (print u_i)
    )
    (print "")
  )

  answer)
