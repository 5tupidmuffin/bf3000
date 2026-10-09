use crate::tokenizer::BFToken;

fn basic_setup() -> String {
    let mut setup = String::new();
    setup.push_str("#include <stdio.h>\n");
    setup.push_str("#include <stdint.h>\n");
    setup.push_str("int main() {\n");
    setup.push_str("int cursor = 0;\n");
    setup.push_str("uint8_t memory[30000] = {0};\n");
    setup
}

pub fn transpile_to_c(bf_tokens: Vec<BFToken>) -> String {
    let _ = bf_tokens;
    let mut c_src = basic_setup();

    for token in bf_tokens {
        match token {
            BFToken::Arithmetic(x) => {
                c_src.push_str(&format!("memory[cursor] += {};\n", x));
            }
            BFToken::Shift(x) => {
                c_src.push_str(&format!("cursor += {};\n", x));
            }
            BFToken::Print => {
                c_src.push_str("printf(%c, memory[cursor]);\n");
            }
            BFToken::Read => {
                c_src.push_str("memory[cursor] = getchar();\n");
            }
            BFToken::BracketOpen => {
                c_src.push_str("while (memory[cursor] != 0) {\n");
            }
            BFToken::BracketClose => {
                c_src.push_str("}\n");
            }
        }
    }
    c_src.push_str("}\n"); // closing main
    c_src
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_program() {
        let c_code = transpile_to_c(vec![]);
        assert_eq!(
            c_code,
            "#include <stdio.h>\n#include <stdint.h>\nint main() {\nint cursor = 0;\nuint8_t memory[30000] = {0};\n}\n"
        )
    }
}
