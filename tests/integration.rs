use glc::*;

fn build_grammar() -> Grammar {
    Grammar(
        nt!("S"),
        vec![
            Rule(
                nt!("S"),
                RuleBody::Sequence(Sequence(vec![ntsym!("A"), ntsym!("B")])),
            ),
            Rule(
                nt!("B"),
                RuleBody::Or(
                    Or(vec![
                       Sequence(vec![ntsym!("A"), ntsym!("B"), ntsym!("N")]),
                       Sequence(vec![ntsym!("E")])
                    ])
                )
            ),
            Rule(
                nt!("E"),
                rb_or_tsym!("")
            ),
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
