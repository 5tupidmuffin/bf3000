use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

use bf3000::{tokenizer::tokenize, transpiler::transpile_to_c};

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!("Usage: bf3000 [bf script]");
    }

    let bf_script_path = Path::new(&args[0]);

    let Ok(bf_src) = fs::read_to_string(bf_script_path) else {
        panic!("failed to read file: {}", args[0]);
    };

    let tokens = tokenize(&bf_src);
    let c_code = transpile_to_c(tokens);

    let Ok(mut c_file) = File::create(bf_script_path.with_extension("c")) else {
        panic!("failed to create c source code file");
    };

    if let Err(err) = c_file.write_all(c_code.as_bytes()) {
        panic!("failed to write c_code to file with error = {}", err);
    }

    // TODO: implement compilation process call based on CC env
}
