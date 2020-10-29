use wasmtime::*;

/// A wasm executor used to call wasm functions
pub struct Executor {
    engine: Engine,
    module: Module,
}

/*
 *
 * let instance = Instance::new(&store, &module, &[log_str.into()])?;
let foo = instance.get_func("foo").unwrap().get0::<()>()?;
*/
impl Executor {
    pub fn new(wasm_code: &[u8]) -> Result<Self, String> {
        let engine = Engine::default();
        let module = Module::new(&engine, wasm_code).map_err(|e| e.to_string())?;
        Ok(Self { engine, module })
    }

    pub fn call(&self, method: &str, call_data: Option<&[Val]>) -> Result<Box<[Val]>, String> {
        let store = Store::new(&self.engine);
        let instance =
            Instance::new(&store, &self.module, Default::default()).map_err(|e| e.to_string())?;

        let func = instance
            .get_func(method)
            .ok_or(format!("method {} not found", method))?;

        func.call(call_data.unwrap_or(&[]))
            .map_err(|trap| trap.to_string())
    }
}
