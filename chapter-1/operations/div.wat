(module
  (func $div (param $lhs i32) (param $rhs i32) (result i32)
     (i32.div_s
       (local.get $lhs)
       (local.get $rhs)
     )
  )
  (export "div" (func $div))
)