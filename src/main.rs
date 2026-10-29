use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

use bf3000::{optimize_tokens, tokenize, transpile_to_c};

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!("Usage: bf3000 [bf script]");
    }

    let bf_script_path = Path::new(&args[0]);

    let Ok(bf_src) = fs::read_to_string(bf_script_path) else {
        panic!("failed to read file: {}", args[0]);
    };

    let tokens = optimize_tokens(&tokenize(&bf_src));
    let c_code = transpile_to_c(tokens);
    let c_file_path = bf_script_path.with_extension("c");

    let Ok(mut c_file) = File::create(&c_file_path) else {
        panic!("failed to create c source code file");
    };

    if let Err(err) = c_file.write_all(c_code.as_bytes()) {
        panic!("failed to write c_code to file with error = {}", err);
    }

    let Ok(c_compiler) = env::var("CC") else {
        return;
    };

    let compiler_flags = env::var("CFLAGS").unwrap_or_else(|_| "".to_string());

    let is_compiled = Command::new(c_compiler)
        .arg(compiler_flags)
        .arg(&c_file_path)
        .arg("-o")
        .arg(&c_file_path.with_extension(""))
        .status()
        .expect("failed to compile c code");

    if !is_compiled.success() {
        panic!("failed to compile c code");
    }
}
