#[derive(Debug, PartialEq)]
// TODO: implement optimizer
#[derive(Clone, Copy)]
pub enum BFToken {
    Arithmetic(i32),
    Shift(i32),
    BracketOpen,
    BracketClose,
    Print,
    Read,
    // optmization tokens, not part of actual specification
    ClearLoop,
    Multiply(i32, i32),
}

fn tokenize_repeating_operators(src_chars: &Vec<char>, mut idx: usize) -> (BFToken, usize) {
    let token = src_chars[idx];
    let mut freq = 0;
    while idx < src_chars.len() && src_chars[idx] == token {
        idx += 1;
        freq += 1;
    }
    return match token {
        '>' => (BFToken::Shift(freq), idx - 1),
        '<' => (BFToken::Shift(-freq), idx - 1),
        '+' => (BFToken::Arithmetic(freq), idx - 1),
        '-' => (BFToken::Arithmetic(-freq), idx - 1),
        _ => panic!("invalid token in tokenize_repeating_operators: {}", token),
    };
}

pub fn tokenize(src: &str) -> Vec<BFToken> {
    let mut tokens = vec![];
    let mut idx = 0;
    let mut bracket_stack = vec![];
    let src_chars: Vec<_> = src.chars().filter(|x| "[]<>.,-+".contains(*x)).collect();

    while idx < src_chars.len() {
        match src_chars[idx] {
            '.' => tokens.push(BFToken::Print),
            ',' => tokens.push(BFToken::Read),
            '[' => {
                bracket_stack.push(1);
                tokens.push(BFToken::BracketOpen);
            }
            ']' => {
                if bracket_stack.is_empty() {
                    panic!("invalid brackets");
                }
                bracket_stack.pop();
                tokens.push(BFToken::BracketClose);
            }
            '+' | '-' | '>' | '<' => {
                let (token, new_idx) = tokenize_repeating_operators(&src_chars, idx);
                tokens.push(token);
                idx = new_idx;
            }
            _ => {}
        }
        idx += 1;
    }
    if !bracket_stack.is_empty() {
        panic!("invalid brackets");
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_tokens() {
        let tests = vec![
            ("+", vec![BFToken::Arithmetic(1)]),
            ("-", vec![BFToken::Arithmetic(-1)]),
            (">", vec![BFToken::Shift(1)]),
            ("<", vec![BFToken::Shift(-1)]),
            (".", vec![BFToken::Print]),
            (",", vec![BFToken::Read]),
            ("[]", vec![BFToken::BracketOpen, BFToken::BracketClose]),
        ];

        for (input, expected) in tests {
            assert_eq!(tokenize(input), expected);
        }
    }

    #[test]
    fn freq_tokens() {
        let tests = vec![
            ("----", vec![BFToken::Arithmetic(-4)]),
            ("++++++", vec![BFToken::Arithmetic(6)]),
            (">>>>>>>", vec![BFToken::Shift(7)]),
            ("<<", vec![BFToken::Shift(-2)]),
        ];

        for (input, expected) in tests {
            assert_eq!(tokenize(input), expected);
        }
    }

    #[test]
    fn slightly_complex() {
        #[rustfmt::skip]
        let tests = vec![
            ("[>>>+++]", vec![BFToken::BracketOpen, BFToken::Shift(3), BFToken::Arithmetic(3), BFToken::BracketClose]),
            ("[]", vec![BFToken::BracketOpen, BFToken::BracketClose]),
            ("[+]", vec![BFToken::BracketOpen, BFToken::Arithmetic(1), BFToken::BracketClose])
        ];

        for (input, expected) in tests {
            assert_eq!(tokenize(input), expected);
        }
    }

    #[test]
    fn valid_brackets() {
        let tests = vec![
            (
                "[[[]]]",
                vec![
                    BFToken::BracketOpen,
                    BFToken::BracketOpen,
                    BFToken::BracketOpen,
                    BFToken::BracketClose,
                    BFToken::BracketClose,
                    BFToken::BracketClose,
                ],
            ),
            (
                "[][][]",
                vec![
                    BFToken::BracketOpen,
                    BFToken::BracketClose,
                    BFToken::BracketOpen,
                    BFToken::BracketClose,
                    BFToken::BracketOpen,
                    BFToken::BracketClose,
                ],
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(tokenize(input), expected);
        }
    }

    #[test]
    #[should_panic]
    fn invalid_brackets_1() {
        tokenize("[");
    }

    #[test]
    #[should_panic]
    fn invalid_brackets_2() {
        tokenize("[[]");
    }

    #[test]
    #[should_panic]
    fn invalid_brackets_3() {
        tokenize("[[][");
    }

    #[test]
    #[should_panic]
    fn invalid_brackets_4() {
        tokenize("]");
    }
}
