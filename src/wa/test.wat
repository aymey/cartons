(module
    (import "host" "log" (func $log (param i32 i32)))
    (import "fromjsstuff" "memseg1" (memory 1))

    (data (i32.const 0) "Hai!")
    (func $hello_world
        i32.const 0
        i32.const 4
        call $filter_a
        call $log
    )

    (func $filter_a (param $offset i32) (param $length i32) (result i32 i32) (local $output_offset i32) (local $output_length i32) (local $is_a i32)
        local.get $offset
        local.get $length
        i32.add
        local.set $output_offset

        (loop $checker
            local.get $offset
            i32.load
            i32.const 65 ;; 'A'
            i32.ne
            (if
                (then
                    local.get $offset
                    local.get $length
                    i32.add
                    local.get $offset
                    i32.load
                    i32.store

                    local.get $output_length
                    i32.const 1
                    i32.add
                    local.set $output_length
                )
            )

            local.get $offset
            i32.const 1
            i32.add
            local.set $offset

            local.get $output_offset
            local.get $offset
            i32.gt_u
            br_if $checker
        )

        local.get $output_offset
        local.get $output_length
    )

    (export "hello_world" (func $hello_world))
)
