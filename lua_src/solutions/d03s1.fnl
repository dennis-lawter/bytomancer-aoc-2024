(let [input _G.input]
  (local common (require :lua_src.common))

  (fn split_out_digits [s]
    (var out [])
    (each [v (string.gmatch (string.sub s 4 -2) "(%d+)")]
      (tset out (+ (length out) 1) v)
    )
    out
  )

  (var answer 0)
  (each [mul (string.gmatch input "mul%(%d+,%d+%)")]
    (print mul)
    (var digits (split_out_digits mul))
    ; answer += tonumber(digits[1]) * tonumber(digits[2])
    (set answer 
      (+ answer
        (*
          (tonumber (. digits 1))
          (tonumber (. digits 2))
        )
      )
    )
  )
  
  answer
)
