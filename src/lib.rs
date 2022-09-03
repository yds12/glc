struct Grammar(NonTerminal, Vec<Rule>);
struct Rule(NonTerminal, RuleBody);
enum RuleBody {
    Or(Or),
    Sequence(Sequence),
}
struct Or(Vec<Symbol>);
struct Sequence(Vec<Symbol>);
enum Symbol {
    NonTerminal(NonTerminal),
    Terminal(Terminal),
}
struct NonTerminal(String);
struct Terminal(String);

macro_rules! tsym {
    ($string:expr) => {
        Symbol::Terminal(Terminal($string.into()))
    };
}

macro_rules! ntsym {
    ($string:expr) => {
        Symbol::NonTerminal(NonTerminal($string.into()))
    };
}

macro_rules! nt {
    ($string:expr) => {
        NonTerminal($string.into())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_grammar() {
        let grammar = Grammar(
            nt!("S"),
            vec![
                Rule(
                    nt!("S"),
                    RuleBody::Sequence(Sequence(vec![ntsym!("A"), ntsym!("N")])),
                ),
                Rule(
                    nt!("A"),
                    RuleBody::Or(Or(vec![
                        tsym!("a"),
                        tsym!("b"),
                        tsym!("c"),
                        tsym!("d"),
                        tsym!("e"),
                        tsym!("f"),
                        tsym!("g"),
                    ])),
                ),
                Rule(
                    nt!("N"),
                    RuleBody::Or(Or(vec![
                        tsym!("0"),
                        tsym!("1"),
                        tsym!("2"),
                        tsym!("3"),
                        tsym!("4"),
                        tsym!("5"),
                        tsym!("6"),
                        tsym!("7"),
                        tsym!("8"),
                        tsym!("9"),
                    ])),
                ),
            ],
        );
    }
}
