use std::collections::BTreeMap;

use crate::tokenizer::BFToken;

pub fn optimize_tokens(tokens: &[BFToken]) -> Vec<BFToken> {
    let mut optimized = optimize_for_clear_loops(tokens);
    optimized = optimize_for_multiplication_loops(&optimized);
    optimized
}

fn optimize_for_clear_loops(tokens: &[BFToken]) -> Vec<BFToken> {
    let mut optimized = vec![];
    let mut idx = 0;
    while idx < tokens.len() {
        match &tokens[idx..] {
            [
                BFToken::BracketOpen,
                BFToken::Arithmetic(_),
                BFToken::BracketClose,
                ..,
            ] => {
                optimized.push(BFToken::ClearLoop);
                idx += 3;
            }
            _ => {
                optimized.push(tokens[idx]);
                idx += 1;
            }
        }
    }
    optimized
}

fn optimize_for_multiplication_loops(tokens: &[BFToken]) -> Vec<BFToken> {
    let mut optimized = vec![];
    let mut idx = 0;

    while idx < tokens.len() {
        if tokens[idx] != BFToken::BracketOpen {
            optimized.push(tokens[idx]);
            idx += 1;
            continue;
        }

        // scan the body without consuming anything yet
        let mut j = idx + 1;
        let mut ptr = 0;
        let mut effects: BTreeMap<i32, i32> = BTreeMap::new();
        let mut optimizable = true;

        while j < tokens.len() && tokens[j] != BFToken::BracketClose {
            match tokens[j] {
                BFToken::Arithmetic(n) => *effects.entry(ptr).or_insert(0) += n,
                BFToken::Shift(n) => ptr += n,
                _ => {
                    optimizable = false;
                    break;
                }
            }
            j += 1;
        }

        let found_close = j < tokens.len() && tokens[j] == BFToken::BracketClose;

        if optimizable && found_close && ptr == 0 && effects.get(&0) == Some(&-1) {
            for (&offset, &factor) in &effects {
                if offset != 0 && factor != 0 {
                    optimized.push(BFToken::Multiply(offset, factor));
                }
            }
            optimized.push(BFToken::ClearLoop); // must come last
            idx = j + 1; // skip past ']'
        } else {
            optimized.push(tokens[idx]); // just the '['
            idx += 1; // body is handled by the normal path
        }
    }

    optimized
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::{BFToken, tokenize};

    #[test]
    fn test_clear_loop() {
        let tests = vec![
            ("[+++++]", vec![BFToken::ClearLoop]),
            ("[--]", vec![BFToken::ClearLoop]),
        ];

        for (input, expected) in tests {
            let tokens = tokenize(input);
            assert_eq!(optimize_for_clear_loops(&tokens), expected)
        }
    }

    #[test]
    fn test_multi_loop() {
        let tests = vec![
            (
                tokenize("[->+++<]"),
                vec![BFToken::Multiply(1, 3), BFToken::ClearLoop],
            ),
            (
                tokenize("[->>+++>>++<<<<]"),
                vec![
                    BFToken::Multiply(2, 3),
                    BFToken::Multiply(4, 2),
                    BFToken::ClearLoop,
                ],
            ),
            (tokenize("[-->++<]"), tokenize("[-->++<]")),
        ];
        for (input, expected) in tests {
            assert_eq!(optimize_for_multiplication_loops(&input), expected)
        }
    }
}
