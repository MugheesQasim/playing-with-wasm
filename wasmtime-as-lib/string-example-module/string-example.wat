(module
(memory 2 3)
(data (i32.const 0) "Hello wasm")
(export "memory" (memory 0))
(func $get_str (result i32 i32) (i32.const 0) (i32.const 10))
(export "get_hello_string" (func $get_str))
)