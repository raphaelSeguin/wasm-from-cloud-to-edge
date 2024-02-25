(module
  (func $sub (param $lhs i32) (param $rhs i32) (result i32)
     (i32.sub
       (local.get $lhs)
       (local.get $rhs)
     )
  )
  (export "sub" (func $sub))
)