(module 
    (func $hello (import "" "hello"))
    (func (export "invoke") (call $hello))
)