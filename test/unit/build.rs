use std::path::{PathBuf};
use glob::glob;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct CompileCommand {
    directory: String,
    command: Option<String>,
    arguments: Option<Vec<String>>,
    file: String,
}

#[derive(Debug, Default)]
struct ExtractedArgs {
    includes: Vec<String>,
    defines: std::collections::HashSet<String>,
}

fn extract_args_from_compile_commands(path: &std::path::Path) -> ExtractedArgs {
    let mut extracted = ExtractedArgs::default();
    if let Ok(content) = std::fs::read_to_string(path) {
        if let Ok(commands) = serde_json::from_str::<Vec<CompileCommand>>(&content) {
            for cmd in commands {
                let parts_vec: Vec<String> = if let Some(cmd_str) = cmd.command {
                    cmd_str.split_whitespace().map(|s| s.to_string()).collect()
                } else if let Some(args) = cmd.arguments {
                    args
                } else {
                    continue;
                };
                let mut parts = parts_vec.iter().map(|s| s.as_str()).peekable();
                let dir_path = std::path::Path::new(&cmd.directory);
                
                while let Some(part) = parts.next() {
                    if part == "-I" || part == "-isystem" {
                        if let Some(next) = parts.next() {
                            let p = std::path::Path::new(next);
                            let resolved = if p.is_absolute() { p.to_path_buf() } else { dir_path.join(p) };
                            let resolved_str = resolved.to_string_lossy().into_owned();
                            if !extracted.includes.contains(&resolved_str) {
                                extracted.includes.push(resolved_str);
                            }
                        }
                    } else if part.starts_with("-I") || part.starts_with("-isystem") {
                        let path_str = if part.starts_with("-isystem") {
                            part.strip_prefix("-isystem").unwrap()
                        } else {
                            part.strip_prefix("-I").unwrap()
                        };
                        let p = std::path::Path::new(path_str);
                        let resolved = if p.is_absolute() { p.to_path_buf() } else { dir_path.join(p) };
                        let resolved_str = resolved.to_string_lossy().into_owned();
                        if !extracted.includes.contains(&resolved_str) {
                            extracted.includes.push(resolved_str);
                        }
                    } else if part == "-D" {
                        if let Some(next) = parts.next() {
                            extracted.defines.insert(next.to_string());
                        }
                    } else if part.starts_with("-D") {
                        extracted.defines.insert(part.strip_prefix("-D").unwrap().to_string());
                    }
                }
            }
        }
    }
    extracted
}

/// Represents the `package.metadata.foreigntest` configuration table
/// This configuration is loaded directly from Cargo.toml to manage the C test environment.
#[derive(Debug, Default)]
struct Config {
    /// Relative or absolute path to the main C project
    pub project_path: String,
    /// Path to compile_commands.json for clang tooling (optional)
    pub compile_commands_path: String,
    /// Custom support header files to include
    pub support_header_files_path: Option<String>,
    /// Glob patterns for files to exclude from mocking/binding
    pub exclude_header_files_paths: Option<Vec<String>>,
    /// Glob patterns for extra files to forcefully include
    pub extra_header_files_paths: Option<Vec<String>>,
    /// Custom C compiler arguments (e.g., -std=c99, -fprofile-arcs)
    pub compile_args: Option<Vec<String>>,
    /// Custom Linker arguments (e.g., -lm, --coverage)
    pub linker_args: Option<Vec<String>>,
}

fn read_config(manifest_path: &std::path::Path) -> Option<Config> {
    let cargo_toml = {
        let mut manifest_file =
            std::fs::File::open(manifest_path).expect("Failed to open Cargo.toml");

        let mut content = String::new();

        std::io::Read::read_to_string(&mut manifest_file, &mut content)
            .expect("Failed to read Cargo.toml");

        content.parse::<toml::Table>().expect("Failed to parse Cargo.toml")
    };

    let foreigntest = cargo_toml
        .get("package")
        .and_then(|table| table.get("metadata"))
        .and_then(|table| table.get("foreigntest"))
        .expect("Failed to find configuration")
        .as_table()
        .expect("Failed to parse configuration");

    let mut config = Config::default();

    for (key, value) in foreigntest {
        match (key.as_str(), value.clone()) {
            ("project_path", toml::Value::String(project_path)) => {
                config.project_path = project_path;
            }
            ("compile_commands_path", toml::Value::String(compile_commands_path)) => {
                config.compile_commands_path = compile_commands_path;
            }
            ("support_header_files_path", toml::Value::String(support_header_files_path)) => {
                config.support_header_files_path = Some(support_header_files_path);
            }
            ("exclude_header_files_paths", toml::Value::Array(exclude_header_files_paths)) => {
                config.exclude_header_files_paths = Some(parse_string_array(exclude_header_files_paths));
            }
            ("extra_header_files_paths", toml::Value::Array(extra_header_files_paths)) => {
                config.extra_header_files_paths = Some(parse_string_array(extra_header_files_paths));
            }
            ("compile_args", toml::Value::Array(compile_args)) => {
                config.compile_args = Some(parse_string_array(compile_args));
            }
            ("linker_args", toml::Value::Array(linker_args)) => {
                config.linker_args = Some(parse_string_array(linker_args));
            }
            _ => return None,
        }
    }
    Some(config)
}

fn parse_string_array(array: Vec<toml::Value>) -> Vec<String> {
    let mut parsed = Vec::new();
    for value in array {
        if let toml::Value::String(s) = value {
            parsed.push(s);
        }
    }
    parsed
}

fn is_excluded(path: &PathBuf, exclude_patterns: &Option<Vec<String>>, project_path: &PathBuf) -> bool {
    if let Some(excludes) = exclude_patterns {
        for ex in excludes {
            let mut pattern = ex.clone();
            if pattern.ends_with("**") {
                pattern.push_str("/*");
            }
            
            let ex_dir = project_path.join(ex.replace("**", ""));
            if path.starts_with(&ex_dir) {
                return true;
            }
        }
    }
    false
}

fn find_files(dir: &std::path::Path, ext: &str) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    if dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    files.extend(find_files(&path, ext));
                } else if path.is_file() {
                    if let Some(e) = path.extension() {
                        if e.to_string_lossy() == ext {
                            files.push(path);
                        }
                    }
                }
            }
        }
    }
    files
}


fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to find manifest dir.");
    let manifest_path = std::path::Path::new(&manifest_dir).join("Cargo.toml");
    let config = read_config(&manifest_path).expect("Failed to read config");

    let project_path = std::path::PathBuf::from(config.project_path)
        .canonicalize()
        .expect("Failed to canonicalize path");

    let mut extracted_args = extract_args_from_compile_commands(&project_path.join(&config.compile_commands_path));

    if extracted_args.includes.is_empty() {
        let h_files = find_files(&project_path, "h");
        let mut include_dirs = std::collections::HashSet::new();
        for path in h_files {
            if !is_excluded(&path, &config.exclude_header_files_paths, &project_path) {
                if let Some(parent) = path.parent() {
                    include_dirs.insert(parent.to_string_lossy().into_owned());
                }
            }
        }
        for dir in include_dirs {
            if !extracted_args.includes.contains(&dir) {
                extracted_args.includes.push(dir);
            }
        }
    }

    let bindings_path = std::path::PathBuf::from("bindings");
    let mocks_path = std::path::PathBuf::from("mocks");
    
    std::fs::create_dir_all(&bindings_path).unwrap();
    std::fs::create_dir_all(&mocks_path).unwrap();

        let spec_path = std::path::Path::new(&manifest_dir).join("spec");
    let mut c_sources_to_compile = std::collections::HashSet::new();

    struct SpecData {
        name: String,
        headers: Vec<std::path::PathBuf>,
        mock_headers: Vec<std::path::PathBuf>,
    }
    let mut specs_data = Vec::new();

    if spec_path.exists() {
        for entry in glob(spec_path.join("**/*.toml").to_str().unwrap()).expect("Failed to read spec glob") {
            if let Ok(path) = entry {
                let content = std::fs::read_to_string(&path).unwrap();
                if let Ok(value) = content.parse::<toml::Table>() {
                    let relative_spec_path = path.strip_prefix(&spec_path).unwrap();
                    let spec_name = relative_spec_path.file_stem().unwrap().to_str().unwrap().to_string();
                    
                    let mut spec_headers = Vec::new();
                    let mut spec_mock_headers = Vec::new();

                    // Parse [headers] array
                    if let Some(headers) = value.get("headers").and_then(|v| v.as_array()) {
                        for header_val in headers {
                            if let Some(h_path) = header_val.as_str() {
                                spec_headers.push(project_path.join(h_path));
                            }
                        }
                    } else {
                        // Fallback Inference
                        let module_h = project_path.join(relative_spec_path.with_extension("h"));
                        spec_headers.push(module_h);
                    }

                    if let Some(mocks) = value.get("mocks").and_then(|v| v.as_array()) {
                        for mock_val in mocks {
                            if let Some(mock_rel_path) = mock_val.as_str() {
                                let mock_h = project_path.join(format!("{}.h", mock_rel_path));
                                spec_mock_headers.push(mock_h);
                            }
                        }
                    }

                    if let Some(sources) = value.get("sources").and_then(|v| v.as_array()) {
                        for src_val in sources {
                            if let Some(src_rel_path) = src_val.as_str() {
                                let src_c = project_path.join(src_rel_path);
                                c_sources_to_compile.insert(src_c);
                            }
                        }
                    } else {
                        // Fallback: If no [sources] block, assume the module itself is the only source
                        let src_c = project_path.join(relative_spec_path.with_extension("c"));
                        c_sources_to_compile.insert(src_c);
                    }

                    specs_data.push(SpecData {
                        name: spec_name,
                        headers: spec_headers,
                        mock_headers: spec_mock_headers,
                    });
                }
            }
        }
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir_path = std::path::Path::new(&out_dir);

    // Build Global Base Builder
    let mut global_base_builder = bindgen::Builder::default().prepend_enum_name(false);
    if let Some(ref support_path) = config.support_header_files_path {
        global_base_builder = global_base_builder.clang_arg(format!("-I{}", project_path.join(support_path).to_string_lossy()));
    }
    for inc in &extracted_args.includes {
        global_base_builder = global_base_builder.clang_arg(format!("-I{}", inc));
    }
    for def in &extracted_args.defines {
        global_base_builder = global_base_builder.clang_arg(format!("-D{}", def));
    }
    if let Some(ref args) = config.compile_args {
        for arg in args {
            if arg.starts_with("-I") {
                let path = arg.strip_prefix("-I").unwrap();
                let full_path = project_path.join(path);
                global_base_builder = global_base_builder.clang_arg(format!("-I{}", full_path.to_string_lossy()));
            } else {
                global_base_builder = global_base_builder.clang_arg(arg.clone());
            }
        }
    }

    // Phase 1: Generate Global Mocks
    let mut unique_mocks = std::collections::HashMap::new();
    for spec in &specs_data {
        for m in &spec.mock_headers {
            let relative_path = m.strip_prefix(&project_path).unwrap();
            unique_mocks.insert(relative_path.to_path_buf(), m.clone());
        }
    }

    for (rel_path, m) in unique_mocks {
        let file_stem = rel_path.file_stem().unwrap().to_str().unwrap();
        let parent_dir = rel_path.parent().unwrap();
        let out_mock_dir = mocks_path.join(parent_dir);
        std::fs::create_dir_all(&out_mock_dir).unwrap();
        
        let out_mock = out_mock_dir.join(format!("mock_{}.rs", file_stem));
        
        let mut mock_str = format!("use mockall::automock;\n\n");
        mock_str.push_str("#[cfg_attr(test, automock)]\npub(crate) mod ffi {\n");
        mock_str.push_str("    use super::*;\n");
        
        let file_name = m.file_name().unwrap().to_str().unwrap();
        let allowlist_pattern = format!(".*{}", file_name.replace(".", "\\.").replace("-", "_"));

        let bindings_str_only_functions = global_base_builder.clone()
            .header(m.to_str().unwrap())
            .use_core()
            .layout_tests(false)
            .blocklist_type(".*")
            .blocklist_var(".*")
            .allowlist_file(&allowlist_pattern)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .unwrap()
            .to_string();
            
        mock_str.push_str(&bindings_str_only_functions);
        mock_str.push_str("}\n");
        mock_str.push_str("pub use mock_ffi::*;\n");
        std::fs::write(&out_mock, mock_str).unwrap();
    }

    // Phase 2: Per-Spec Generation
    for spec in specs_data {
        // 1. Generate the spec's Umbrella header
        let umbrella_h_path = out_dir_path.join(format!("{}_umbrella.h", spec.name));
        let mut umbrella_content = String::new();
        for h in &spec.headers {
            umbrella_content.push_str(&format!("#include \"{}\"\n", h.to_str().unwrap()));
        }
        for m in &spec.mock_headers {
            umbrella_content.push_str(&format!("#include \"{}\"\n", m.to_str().unwrap()));
        }
        std::fs::write(&umbrella_h_path, umbrella_content).unwrap();

        // 2. Base Builder for this spec
        let base_builder = global_base_builder.clone()
            .header(umbrella_h_path.to_str().unwrap());

        // Get relative path of the first header for binding hierarchy
        let main_header = &spec.headers[0];
        let rel_main_header = main_header.strip_prefix(&project_path).unwrap();
        let main_parent_dir = rel_main_header.parent().unwrap();

        // 3. Generate the main Spec Binding (`bindings/<path>/<spec_name>.rs`)
        let out_binding_dir = bindings_path.join(main_parent_dir);
        std::fs::create_dir_all(&out_binding_dir).unwrap();
        let out_binding = out_binding_dir.join(format!("{}.rs", spec.name));
        
        let mut spec_builder = base_builder.clone()
            .use_core()
            .layout_tests(false)
            .allowlist_type(".*")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

        for h in &spec.headers {
            let file_name = h.file_name().unwrap().to_str().unwrap();
            let allowlist_pattern = format!(".*{}", file_name.replace(".", "\\.").replace("-", "_"));
            spec_builder = spec_builder.allowlist_file(&allowlist_pattern);
        }

        spec_builder
            .generate()
            .unwrap()
            .write_to_file(&out_binding)
            .unwrap();

        // 4. Generate the Per-Spec Mock Wrapper (`mocks/<path>/<spec_name>_mocks.rs`)
        if !spec.mock_headers.is_empty() {
            let out_mock_wrapper_dir = mocks_path.join(main_parent_dir);
            std::fs::create_dir_all(&out_mock_wrapper_dir).unwrap();
            let out_mock_wrapper = out_mock_wrapper_dir.join(format!("{}_mocks.rs", spec.name));
            let mut wrapper_str = String::from("#![allow(ambiguous_glob_reexports)]\n\n");
            
            for m in &spec.mock_headers {
                let rel_mock_path = m.strip_prefix(&project_path).unwrap();
                let mock_file_stem = rel_mock_path.file_stem().unwrap().to_str().unwrap();
                
                // Calculate relative path from wrapper to global mock
                // Wrapper is at mocks/<main_parent_dir>/<spec_name>_mocks.rs
                // Global mock is at mocks/<mock_parent_dir>/mock_<mock_file_stem>.rs
                
                let mock_parent_dir = rel_mock_path.parent().unwrap();
                let mut go_up = String::new();
                for _ in 0..main_parent_dir.components().count() {
                    go_up.push_str("../");
                }
                let relative_to_mock = format!("{}{}/mock_{}.rs", go_up, mock_parent_dir.to_string_lossy(), mock_file_stem);
                
                wrapper_str.push_str(&format!("pub mod mock_{} {{\n", mock_file_stem));
                wrapper_str.push_str(&format!("    use crate::{}::*;\n", spec.name));
                wrapper_str.push_str(&format!("    include!(\"{}\");\n", relative_to_mock));
                wrapper_str.push_str("}\n");
                wrapper_str.push_str(&format!("pub use mock_{}::*;\n\n", mock_file_stem));
            }
            std::fs::write(&out_mock_wrapper, wrapper_str).unwrap();
        }
    }
    // 3. Compile all C sources into a static library
    let mut build = cc::Build::new();
    
    if let Some(ref support_path) = config.support_header_files_path {
        build.include(project_path.join(support_path));
    }
    
    // Inject includes and defines from compile_commands.json
    for inc in &extracted_args.includes {
        build.include(inc);
    }
    for def in &extracted_args.defines {
        let parts: Vec<&str> = def.splitn(2, '=').collect();
        if parts.len() == 2 {
            build.define(parts[0], Some(parts[1]));
        } else {
            build.define(parts[0], None);
        }
    }
    
    // extracted_args.includes already contains the fallback recursive dirs if needed
    
    for src in &c_sources_to_compile {
        build.file(src);
    }
    
    if let Some(args) = config.compile_args {
        for arg in args {
            if arg.starts_with("-I") {
                let path = arg.strip_prefix("-I").unwrap();
                build.include(project_path.join(path));
            } else {
                build.flag_if_supported(&arg);
            }
        }
    }
    
    // Output linker arguments if any
    if let Some(ref linker_args) = config.linker_args {
        for arg in linker_args {
            println!("cargo:rustc-link-arg={}", arg);
        }
    }
    
    // Crucial for test isolation with GNU linker: put each function in its own section,
    // so unused functions (like unmocked ones) can be garbage collected or overridden
    // easily if necessary.
    build.flag_if_supported("-ffunction-sections");
    build.flag_if_supported("-fdata-sections");

    build.compile("project");
}
