use glc::{Grammar, Expression, nt, t_or_rule, nt_seq_rule};

fn build_grammar() -> Grammar {
    Grammar(
        nt!("S"),
        vec![
            nt_seq_rule!("S" => "A", "B"),
            nt_seq_rule!("B" => "A", "B", "N"),
            nt_seq_rule!("B" => "E"),
            t_or_rule!("E" => ""),
            t_or_rule!(
                "A" => "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k",
                       "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v",
                       "w", "x", "y", "z"
            ),
            t_or_rule!("N" => "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"),
        ],
    )
}

#[test]
fn test() {
    let grammar = build_grammar();
    let mut strings = Vec::new();

    for _ in 0..100 {
        strings.push(grammar.gen());
    }

    for s in strings {
        assert!(!s.is_empty());
        assert!(s.chars().next().unwrap().is_ascii_alphabetic());
    }
}

#[test]
fn test_derivation() {
    let grammar = build_grammar();
    let mut derivation = grammar.start_derivation();

    while !derivation.is_done() {
        derivation.derive_step(&grammar);
    }

    let expr: Expression = derivation.into();
    dbg!(expr);
}
