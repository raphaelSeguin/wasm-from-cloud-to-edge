(module
  (func $mul (param $lhs i32) (param $rhs i32) (result i32)
     (i32.mul
       (local.get $lhs)
       (local.get $rhs)
     )
  )
  (export "mul" (func $mul))
)