(module
    (import "host" "log" (func $log (param i32)))

    (func $sub (param $lhs i32) (param $rhs i32) (result i32)
          local.get $lhs
          local.get $rhs
          i32.sub
    )

    (func $pow (param $base i32) (param $exp i32)
          (local $res i32)
          (local.set $res (i32.const 1))

          (loop $multiply
                ;; res += base*base
                local.get $base
                local.get $res
                i32.mul
                local.set $res

                ;; exp--
                local.get $exp
                i32.const 1
                i32.sub
                local.set $exp

                ;; if(exp > 0) continue
                local.get $exp
                i32.const 0
                i32.gt_u
                br_if $multiply
          )

          local.get $res
          call $log
    )

    (export "sub" (func $sub))

    (export "pow" (func $pow))
)
