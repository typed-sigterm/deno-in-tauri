use rustyscript::deno_core::{anyhow, ResolutionKind, ModuleSpecifier, RequestedModuleType};
use rustyscript::{module_loader::ImportProvider, Module, Runtime, RuntimeOptions};

fn module_not_found(specifier: &ModuleSpecifier) -> anyhow::Error {
    anyhow::anyhow!("Module not found \"{}\"", specifier.to_string())
}

#[derive(Default)]
struct MyImportProvider {
    module_source: String
}

impl MyImportProvider {
    fn new(code: String) -> Self {
        Self {
            module_source: code
        }
    }
}

impl ImportProvider for MyImportProvider {
    fn resolve(
        &mut self,
        specifier: &ModuleSpecifier,
        _referrer: &str,
        _kind: ResolutionKind,
    ) -> Option<Result<ModuleSpecifier, anyhow::Error>> {
        if specifier.scheme() == "module" {
            match specifier.path() {
                "main" => Some(Ok(specifier.clone())),
                _ => Some(Err(module_not_found(specifier))),
            }
        } else {
            None
        }
    }

    fn import(
        &mut self,
        specifier: &ModuleSpecifier,
        _referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: RequestedModuleType,
    ) -> Option<Result<String, anyhow::Error>> {
        if specifier.scheme() == "module" {
            match specifier.path() {
                "main" => Some(Ok(self.module_source.clone())),
                _ => Some(Err(module_not_found(specifier))),
            }
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
