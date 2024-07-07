use std::{ffi::c_void, path::Path};
use wamr_rust_sdk::{
    function::Function, instance::Instance, module::Module, runtime::Runtime, value::WasmValue,
    wasi_context::WasiCtxBuilder, RuntimeError,
};

extern "C" fn log(arg: i32) {
    println!("{arg}")
}

fn main() -> Result<(), RuntimeError> {
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("log", log as *mut c_void)
        .build()?;
    let mut module = Module::from_file(&runtime, Path::new("test.wasm"))?;

    let wasi_ctx = WasiCtxBuilder::new()
        .set_pre_open_path(vec!["."], vec![])
        .build();

    module.set_wasi_context(wasi_ctx);

    let instance = Instance::new(&runtime, &module, 1024 * 64)?;

    let function = Function::find_export_func(&instance, "pow")?;

    let params: Vec<WasmValue> = vec![WasmValue::I32(2), WasmValue::I32(2)];
    function.call(&instance, &params)?;

    Ok(())
}
