use crate::tokenizer::BFToken;

pub fn optimize_tokens(tokens: &[BFToken]) -> Vec<BFToken> {
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
                optimized.push(tokens[idx].clone());
                idx += 1;
            }
        }
    }

    optimized
}

#[cfg(test)]
mod tests {
    use crate::{optimizer::optimize_tokens, tokenizer::BFToken};

    #[test]
    fn test_clear_loop() {
        let tests = vec![
            (
                vec![
                    BFToken::BracketOpen,
                    BFToken::Arithmetic(5),
                    BFToken::BracketClose,
                ],
                vec![BFToken::ClearLoop],
            ),
            (
                vec![
                    BFToken::BracketOpen,
                    BFToken::Arithmetic(-2),
                    BFToken::BracketClose,
                ],
                vec![BFToken::ClearLoop],
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(optimize_tokens(&input), expected)
        }
    }
}
