use wasmtime::*;

fn main() -> Result<()> {
    let mut store : Store<()> = Store::default();
    let module = Module::from_file(store.engine(), "./string-example.wat")?;
    let instance = Instance::new(&mut store, &module, &[])?;

    let memory = instance.get_memory(&mut store , "memory").expect("Failed to get memory");

    let get_str_func = instance.get_typed_func::<(),(i32,i32)>(&mut store, "get_hello_string")?;
    let (start_add, length) = get_str_func.call(&mut store, ())?;

    let data = memory.data(&store)
                .get(start_add as usize..(start_add+length) as usize)
                .expect("Failed to fetch data");

    let result = std::str::from_utf8(data)?;
    println!("{:?}", result);

    Ok(())
}
