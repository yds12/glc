use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Grammar(pub NonTerminal, pub Vec<Rule>);

#[derive(Debug, Clone)]
pub struct Rule(pub NonTerminal, pub RuleBody);

#[derive(Debug, Clone)]
pub enum RuleBody {
    Or(Or),
    Sequence(Sequence),
}

#[derive(Debug, Clone)]
pub struct Or(pub Vec<Sequence>);

#[derive(Debug, Clone)]
pub struct Sequence(pub Vec<Symbol>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    NonTerminal(NonTerminal),
    Terminal(Terminal),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonTerminal(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Terminal(pub String);

#[derive(Debug, Clone)]
pub struct Derivation(pub Vec<Symbol>);

#[derive(Debug, Clone)]
pub struct Expression(pub Vec<Terminal>);

#[macro_export]
macro_rules! seq_tsym {
    ($string:expr) => {
        Sequence(vec![Symbol::Terminal(Terminal($string.into()))])
    };
}

#[macro_export]
macro_rules! rb_or_tsym {
    ($( $string:expr ),*) => {
        RuleBody::Or(Or(vec![
            $(
                Sequence(vec![Symbol::Terminal(Terminal($string.into()))])
            ),*
        ]))
    };
}

#[macro_export]
macro_rules! ntsym {
    ($string:expr) => {
        Symbol::NonTerminal(NonTerminal($string.into()))
    };
}

#[macro_export]
macro_rules! nt {
    ($string:expr) => {
        NonTerminal($string.into())
    };
}


impl Grammar {
    /// Generate a random expression using the grammar
    pub fn gen(&self) -> String {
        let d = self.start_derivation();
        let expr = d.derive(&self);
        expr.to_string()
    }

    fn start_derivation(&self) -> Derivation {
        Derivation(vec![Symbol::NonTerminal(self.0.clone())])
    }

    fn choose_rule(&self, nt: NonTerminal) -> &Rule {
        let mut rules = Vec::new();

        for rule in self.1.iter() {
            if rule.0 == nt {
                rules.push(rule);
            }
        }

        rules.shuffle(&mut rand::thread_rng());
        rules.iter().next().unwrap_or_else(||
            panic!("no rule for {nt:?}")
        )
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
        if self.0.is_empty() {
            panic!("empty or");
        }

        let rand_index = rand::thread_rng().gen_range(0..self.0.len());
        &self.0[rand_index]
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
                Rule(
                    nt!("A"),
                    rb_or_tsym!("a", "b", "c", "d", "e", "f", "g", "h", "i",
                                "j", "k", "l", "m", "n", "o", "p", "q", "r",
                                "s", "t", "u", "v", "w", "x", "y", "z")
                ),
                Rule(
                    nt!("N"),
                    rb_or_tsym!("0", "1", "2", "3", "4", "5", "6", "7", "8", "9")
                ),
            ],
        )
    }

    #[test]
    fn it_works() {
        let grammar = build_grammar();
        let mut strings = Vec::new();

        for _ in 0..10 {
            strings.push(grammar.gen());
        }
        assert_eq!(strings, Vec::<String>::new());
    }
}
