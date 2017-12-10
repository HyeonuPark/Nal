use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // alias nal_ast::ast
    let ast_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("../nal_ast/src/ast");
    fs::copy(&ast_dir.join("expr.rs"), &out_dir.join("ast_expr.rs")).unwrap();
    fs::copy(&ast_dir.join("stmt.rs"), &out_dir.join("ast_stmt.rs")).unwrap();
    fs::copy(&ast_dir.join("function.rs"), &out_dir.join("ast_function.rs")).unwrap();
    fs::copy(&ast_dir.join("module.rs"), &out_dir.join("ast_module.rs")).unwrap();
}
