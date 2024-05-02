use anyhow::{anyhow, Context as _, Error};
use clap::builder::OsStr;
/// Here, we initialize those modules that are defined in JS, as
/// opposed to native modules defined in Rust. The sources for
/// these modules should be put under the internal_js_modules dir
/// that lives next to this source file.
///
/// The name of the modules will be equivalent to their path on disk,
/// but forward slashes (/) will be changed to a colon (:) and the .js
/// extension will be stripped. So, to create a node:buffer module,
/// one must put the source under `internal_js_modules/node/buffer.js`.
///
/// Note: Files that don't have a .js extension will be ignored.
use include_dir::{include_dir, Dir, File};
use ion::{
    module::{Module, ModuleRequest},
    Context,
};

const MODULES_DIR: Dir = include_dir!("src/builtins/js_modules/modules");
const INIT_DIR: Dir = include_dir!("src/builtins/js_modules");

pub(super) fn define(cx: &Context) -> bool {
    match scan_dir(cx, &MODULES_DIR, Some(&INIT_DIR)) {
        Ok(()) => true,
        Err(e) => {
            tracing::error!(
                error = %e,
                "Failed to load internal modules"
            );
            false
        }
    }
}

fn scan_dir(cx: &Context, modules_dir: &Dir, init_dir: Option<&Dir>) -> anyhow::Result<()> {
    for file in modules_dir.files() {
        if file.path().extension() == Some(&OsStr::from("js")) {
            compile_and_register_modules(cx, file)?;
        }
    }

    if let Some(dir) = init_dir {
        for file in dir.files() {
            if file.path().extension() == Some(&OsStr::from("js")) {
                register_global_modules(cx, file)?;
            }
        }
    }

    for dir in modules_dir.dirs() {
        scan_dir(cx, dir, None)?;
    }

    Ok(())
}

fn compile_and_register_modules(cx: &Context, script_file: &File) -> anyhow::Result<()> {
    let module_name = format!(
        "jsmodule_{}",
        script_file
            .path()
            .file_stem()
            .and_then(|os_str| os_str.to_str())
            .context("Failed to convert module path to string")?
    );

    let contents = script_file
        .contents_utf8()
        .context("Failed to convert file contents to UTF-8")?;

    let module = Module::compile(cx, &module_name, None, contents)
        .map_err(|e| anyhow::anyhow!("Module compilation failed: {e:?}"))?;

    match unsafe { &mut (*cx.get_inner_data().as_ptr()).module_loader } {
        Some(loader) => {
            let request = ModuleRequest::new(cx, module_name);
            loader
                .register(cx, module.module_object(), &request)
                .map_err(|e| anyhow!("Failed to register internal module due to: {e}"))?;
            Ok(())
        }
        None => anyhow::bail!("No module loader present, cannot register internal module"),
    }
}

fn register_global_modules(cx: &Context, script_file: &File) -> anyhow::Result<(), Error> {
    let module_name = script_file
        .path()
        .to_str()
        .context("Failed to convert module path to string")?
        .strip_suffix(".js")
        .context("Script file path must have a .js suffix")?
        .replace('/', ":");

    let contents = script_file
        .contents_utf8()
        .context("Failed to convert file contents to UTF-8")?;

    match Module::compile_and_evaluate(cx, &module_name, None, contents)
        .map_err(|e| anyhow::anyhow!("Module compilation failed: {e:?}"))
    {
        Ok((_, _)) => Ok(()),
        Err(_) => Err(anyhow!(
            "No module loader present, cannot register internal module"
        )),
    }
}
