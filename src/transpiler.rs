use crate::tokenizer::BFToken;

/*
 * TODO:
 * indentation insertion could be better
 * there should be an option to choose tabs or spaces for indentation
 * probably try proper code generation with simple AST translation
 * probably more test cases
 */

fn basic_setup() -> String {
    let mut setup = String::new();
    setup.push_str("#include <stdio.h>\n");
    setup.push_str("#include <stdint.h>\n");
    setup.push_str("\nint main() {\n");
    setup.push_str("    int cursor = 0;\n");
    setup.push_str("    uint8_t memory[30000] = {0};\n");
    setup
}

pub fn transpile_to_c(bf_tokens: Vec<BFToken>) -> String {
    let _ = bf_tokens;
    let mut c_src = basic_setup();
    let mut indent = 1;

    for token in bf_tokens {
        match token {
            BFToken::Arithmetic(x) => {
                c_src.push_str(&format!(
                    "{}memory[cursor] += {};\n",
                    "    ".repeat(indent),
                    x
                ));
            }
            BFToken::Shift(x) => {
                c_src.push_str(&format!("{}cursor += {};\n", "    ".repeat(indent), x));
            }
            BFToken::Print => {
                c_src.push_str(&format!(
                    "{}printf(\"%c\", memory[cursor]);\n",
                    "    ".repeat(indent)
                ));
            }
            BFToken::Read => {
                c_src.push_str(&format!(
                    "{}memory[cursor] = getchar();\n",
                    "    ".repeat(indent)
                ));
            }
            BFToken::BracketOpen => {
                c_src.push('\n');
                c_src.push_str(&format!(
                    "{}while (memory[cursor] != 0) {{\n",
                    "    ".repeat(indent),
                ));
                indent += 1;
            }
            BFToken::BracketClose => {
                indent -= 1;
                c_src.push_str(&format!("{}}}\n\n", "    ".repeat(indent)));
            }
        }
    }
    c_src.push_str("    return 0;\n}\n"); // closing main
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
            "#include <stdio.h>\n#include <stdint.h>\n\nint main() {\n    int cursor = 0;\n    uint8_t memory[30000] = {0};\n    return 0;\n}\n"
        )
    }
}
