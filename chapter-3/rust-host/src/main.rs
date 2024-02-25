use std::{error::Error, fs::File, io::Read};
use wasmer::{imports, Instance, Module, Store, Value};

fn main() -> Result<(), Box<dyn Error>> {
    let mut wat_file = File::open("calc.wasm")?;
    let mut wat_bytes = vec![];
    wat_file.read_to_end(&mut wat_bytes)?;

    let mut store = Store::default();
    let module = Module::new(&store, &wat_bytes)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add = instance.exports.get_function("add")?;
    let mul = instance.exports.get_function("mul")?;
    let sub = instance.exports.get_function("sub")?;
    let div = instance.exports.get_function("div")?;
    let result = add.call(&mut store, &[Value::I32(42), Value::I32(27)])?;
    assert_eq!(result[0], Value::I32(69));

    println!("add 123 and 321 = {:?}", add.call(&mut store, &[Value::I32(123), Value::I32(321)]).unwrap()[0]);
    println!("mul 123 and 321 = {:?}", mul.call(&mut store, &[Value::I32(123), Value::I32(321)]).unwrap()[0]);
    println!("sub 123 and 321 = {:?}", sub.call(&mut store, &[Value::I32(123), Value::I32(321)]).unwrap()[0]);
    println!("div 123 and 321 = {:?}", div.call(&mut store, &[Value::I32(123), Value::I32(321)]).unwrap()[0]);

    Ok(())
}
