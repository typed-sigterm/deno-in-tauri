use rustyscript::{
    deno_core::{anyhow, ModuleSpecifier, RequestedModuleType, ResolutionKind},
    module_loader::ImportProvider,
    Runtime, RuntimeOptions, StaticModule,
};

/// A custom import provider that resolves `script:main` to the provided code.
///
/// Note: `script:main` can only be imported once, or it will throw a `TypeError`.
struct ScriptImportProvider {
    module_source: String,
    imported: bool,
    locked: bool,
    // After a module is imported, it will be resolved again. So there is [`locked`].
}

impl ScriptImportProvider {
    fn new(code: String) -> Self {
        Self {
            module_source: code,
            imported: false,
            locked: false,
        }
    }
}

impl ImportProvider for ScriptImportProvider {
    fn resolve(
        &mut self,
        specifier: &ModuleSpecifier,
        _: &str,
        _: ResolutionKind,
    ) -> Option<Result<ModuleSpecifier, anyhow::Error>> {
        if !self.locked && specifier.to_string() == "script:main" {
            if self.imported {
                self.locked = true;
            }
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
        if !self.imported && specifier.to_string() == "script:main" {
            self.imported = true;
            Some(Ok(self.module_source.clone()))
        } else {
            None
        }
    }
}

#[tauri::command]
fn execute(code: String) -> Result<String, String> {
    let wrapper = StaticModule::new(
        "<bootstrap>",
        r#"
        import mod from "script:main";
        export default () => {
            if (typeof mod !== 'function') {
                throw new TypeError("The script must export a function named 'default'");
            }
            return Deno.inspect(mod()); // To prettify the output
        }
        "#,
    );

    let import_provider = ScriptImportProvider::new(code);
    let mut runtime = Runtime::new(RuntimeOptions {
        import_provider: Some(Box::new(import_provider)),
        ..Default::default()
    })
    .expect("Failed to create runtime");

    match runtime.load_module(&wrapper.to_module()) {
        Ok(handle) => runtime
            .call_entrypoint::<String>(&handle, &())
            .map_err(|e| e.to_string()),
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
