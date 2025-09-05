// FILE: ./build.rs
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let destination = Path::new(&out_dir).join("rules_list.rs");
    let mut f = fs::File::create(&destination).unwrap();

    let mut rule_modules = Vec::new();

    // Scan the src/rules directory for rule files
    for entry in fs::read_dir("src/rules").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                // Exclude non-rule files like mod.rs and any utils.rs
                if filename.ends_with(".rs")
                    && filename != "mod.rs"
                    && !filename.starts_with("utils")
                {
                    // Get the module name without the .rs extension
                    let module_name = Path::new(filename).file_stem().unwrap().to_str().unwrap();
                    rule_modules.push(module_name.to_string());
                }
            }
        }
    }

    // Sort modules alphabetically for a consistent build output
    rule_modules.sort();

    // Write the vec! macro call with all the rules
    writeln!(f, "vec![").unwrap();
    for module in &rule_modules {
        writeln!(f, "    {}::get_rule(),", module).unwrap();
    }
    writeln!(f, "]").unwrap();
}
