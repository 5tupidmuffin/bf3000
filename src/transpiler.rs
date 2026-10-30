use crate::tokenizer::BFToken;

/*
 * TODO:
 * indentation insertion could be better
 * there should be an option to choose tabs or spaces for indentation
 * probably try proper code generation with simple AST translation
 * probably more test cases
 */
const INDENT: &str = "    ";

fn basic_setup() -> String {
    let mut setup = String::new();
    setup.push_str("#include <stdio.h>\n");
    setup.push_str("#include <stdint.h>\n");
    setup.push_str("\nint main() {\n");
    setup.push_str(&format!("{}int cursor = 0;\n", INDENT));
    setup.push_str(&format!("{}uint8_t memory[30000] = {};\n", INDENT, "{0}"));
    setup
}

pub fn transpile_to_c(bf_tokens: Vec<BFToken>) -> String {
    let mut c_src = basic_setup();
    let mut indent = 1;

    for token in bf_tokens {
        match token {
            BFToken::Arithmetic(x) => {
                c_src.push_str(&format!(
                    "{}memory[cursor] += {};\n",
                    INDENT.repeat(indent),
                    x
                ));
            }
            BFToken::Shift(x) => {
                c_src.push_str(&format!("{}cursor += {};\n", INDENT.repeat(indent), x));
            }
            BFToken::Print => {
                c_src.push_str(&format!(
                    "{}printf(\"%c\", memory[cursor]);\n",
                    INDENT.repeat(indent)
                ));
            }
            BFToken::Read => {
                c_src.push_str(&format!(
                    "{}memory[cursor] = getchar();\n",
                    INDENT.repeat(indent)
                ));
            }
            BFToken::BracketOpen => {
                c_src.push('\n');
                c_src.push_str(&format!(
                    "{}while (memory[cursor] != 0) {{\n",
                    INDENT.repeat(indent),
                ));
                indent += 1;
            }
            BFToken::BracketClose => {
                indent -= 1;
                c_src.push_str(&format!("{}}}\n\n", INDENT.repeat(indent)));
            }

            // optimization tokens
            BFToken::ClearLoop => {
                c_src.push_str(&format!("{}memory[cursor] = 0;\n", INDENT.repeat(indent)));
            }
            BFToken::Multiply(offset, factor) => {
                c_src.push_str(&format!(
                    "{}memory[cursor + {}] += memory[cursor] * {};\n",
                    INDENT.repeat(indent),
                    offset,
                    factor,
                ));
            }
        }
    }
    c_src.push_str(&format!("{}return 0;\n}}\n", INDENT)); // closing main
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
