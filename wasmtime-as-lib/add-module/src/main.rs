use wasmtime::*;

fn main() -> Result<()> {
    let engine: Engine = Engine::default();
    let module = Module::from_file(&engine, "./add.wat")?;

    let mut store = Store::new(&engine, 0);

    let instance = Instance::new(&mut store, &module, &[])?;

    let add_func = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;

    let result = add_func.call(&mut store, (16, 3))?;

    println!("Result of call: {:?}", result);

    Ok(())
}
