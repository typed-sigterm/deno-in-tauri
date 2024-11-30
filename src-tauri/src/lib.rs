use rustyscript::{
    deno_core::{anyhow, ResolutionKind, ModuleSpecifier, RequestedModuleType},
    module_loader::ImportProvider,
    Module,
    Runtime,
    RuntimeOptions,
};

/// A custom import provider that resolves `module:main` to the provided code.
/// 
/// Note: `module:main` can only be resolved once, or it will throw a `TypeError`.
struct MyImportProvider {
    module_source: String,
    resolved: bool,
}

impl MyImportProvider {
    fn new(code: String) -> Self {
        Self {
            module_source: code,
            resolved: false,
        }
    }
}

impl ImportProvider for MyImportProvider {
    fn resolve(
        &mut self,
        specifier: &ModuleSpecifier,
        _: &str,
        _: ResolutionKind,
    ) -> Option<Result<ModuleSpecifier, anyhow::Error>> {
        if !self.resolved && specifier.to_string() == "module:main" {
            self.resolved = true;
            Some(Ok(specifier.clone()))
        } else {
            None
        }
    }

    fn import(
        &mut self,
        specifier: &ModuleSpecifier,
        _: Option<&ModuleSpecifier>,
        _: bool,
        _: RequestedModuleType,
    ) -> Option<Result<String, anyhow::Error>> {
        if !self.resolved && specifier.to_string() == "module:main" {
            self.resolved = true;
            Some(Ok(self.module_source.clone()))
        } else {
            None
        }
    }
}

#[tauri::command]
fn execute(code: String) -> Result<String, String> {
    let wrapper = Module::new(
        "<bootstrap>",
        r#"
        import mod from "module:main";
        export default () => Deno.inspect(mod());
        "#,
    );

    let import_provider = MyImportProvider::new(code);
    let mut runtime = Runtime::new(RuntimeOptions {
        import_provider: Some(Box::new(import_provider)),
        ..Default::default()
    }).expect("Failed to create runtime");

    match runtime.load_module(&wrapper) {
        Ok(handle) => {
            runtime
                .call_entrypoint::<String>(&handle, &())
                .map_err(|e| e.to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![execute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
