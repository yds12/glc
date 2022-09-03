#[derive(Debug, Clone)]
struct Grammar(NonTerminal, Vec<Rule>);

#[derive(Debug, Clone)]
struct Rule(NonTerminal, RuleBody);

#[derive(Debug, Clone)]
enum RuleBody {
    Or(Or),
    Sequence(Sequence),
}

#[derive(Debug, Clone)]
struct Or(Vec<Sequence>);

#[derive(Debug, Clone)]
struct Sequence(Vec<Symbol>);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Symbol {
    NonTerminal(NonTerminal),
    Terminal(Terminal),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NonTerminal(String);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Terminal(String);

#[derive(Debug, Clone)]
struct Derivation(Vec<Symbol>);

#[derive(Debug, Clone)]
struct Expression(Vec<Terminal>);

macro_rules! seq_tsym {
    ($string:expr) => {
        Sequence(vec![Symbol::Terminal(Terminal($string.into()))])
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

impl Grammar {
    /// Generate a random expression using the grammar
    fn gen(&self) -> String {
        todo!()
    }

    fn start_derivation(&self) -> Derivation {
        Derivation(vec![Symbol::NonTerminal(self.0.clone())])
    }

    fn choose_rule(&self, nt: NonTerminal) -> &Rule {
        for rule in self.1.iter() {
            if rule.0 == nt {
                return rule;
            }
        }
        panic!("no rule for {nt:?}");
    }
}

impl Derivation {
    fn derive(mut self, grammar: &Grammar) -> Expression {
        while !self.is_done() {
            self.derive_step(grammar);
        }

        self.into()
    }

    fn derive_step(&mut self, grammar: &Grammar) {
        if self.is_done() {
            return;
        }

        let nt = self.find_nt();

        if let Some(nt) = nt {
            let rule = grammar.choose_rule(nt);
            self.apply(rule);
        }
    }

    fn is_done(&self) -> bool {
        for symbol in &self.0 {
            if let Symbol::NonTerminal(_) = symbol {
                return false;
            }
        }

        true
    }

    fn find_nt(&self) -> Option<NonTerminal> {
        for symbol in &self.0 {
            if let Symbol::NonTerminal(nt) = symbol {
                return Some(nt.clone());
            }
        }

        None
    }

    fn find(&self, nt: &NonTerminal) -> Option<usize> {
        for (i, elem) in self.0.iter().enumerate() {
            if let Symbol::NonTerminal(el) = elem {
                if el == nt {
                    return Some(i);
                }
            }
        }

        None
    }

    fn apply(&mut self, rule: &Rule) {
        match self.find(&rule.0) {
            Some(index) => self.apply_at(rule, index),
            None => panic!("rule cannot be applied")
        }
    }

    fn apply_at(&mut self, rule: &Rule, index: usize) {
        let seq = match &rule.1 {
            RuleBody::Sequence(seq) => {
                seq.0.clone()
            }
            RuleBody::Or(or) => {
                or.choose_seq().0.clone()
            }
        };

        self.0.splice(index..(index + 1), seq);
    }
}

impl From<Derivation> for Expression {
    fn from(d: Derivation) -> Self {
        Self(d.0.into_iter().map(|sym| match sym {
            Symbol::NonTerminal(_) => panic!("derivation is not complete"),
            Symbol::Terminal(t) => t
        }).collect())
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for t in &self.0 {
            write!(f, "{}", t.0)?;
        }

        Ok(())
    }
}

impl Or {
    fn choose_seq(&self) -> &Sequence {
        self.0.iter().next().unwrap_or_else(|| panic!("or has no sequences"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_grammar() -> Grammar {
        Grammar(
            nt!("S"),
            vec![
                Rule(
                    nt!("S"),
                    RuleBody::Sequence(Sequence(vec![ntsym!("A"), ntsym!("N")])),
                ),
                Rule(
                    nt!("A"),
                    RuleBody::Or(Or(vec![
                        seq_tsym!("a"),
                        seq_tsym!("b"),
                        seq_tsym!("c"),
                        seq_tsym!("d"),
                        seq_tsym!("e"),
                        seq_tsym!("f"),
                        seq_tsym!("g"),
                    ])),
                ),
                Rule(
                    nt!("N"),
                    RuleBody::Or(Or(vec![
                        seq_tsym!("0"),
                        seq_tsym!("1"),
                        seq_tsym!("2"),
                        seq_tsym!("3"),
                        seq_tsym!("4"),
                        seq_tsym!("5"),
                        seq_tsym!("6"),
                        seq_tsym!("7"),
                        seq_tsym!("8"),
                        seq_tsym!("9"),
                    ])),
                ),
            ],
        )
    }

    #[test]
    fn it_works() {
        let g = build_grammar();
        let d = g.start_derivation();
        let expr = d.derive(&g);

        assert_eq!(expr.to_string(), "a0");
    }
}
