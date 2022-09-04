# glc

This crate's aim is to generate random expressions based on a context-free
grammar.

The acronym stands for "gramática livre de contexto" (*context-free grammar*).

## How to Use

    use glc::{Grammar, nt, t_or_rule, nt_seq_rule};

    fn main() {
        let grammar = Grammar(
            // starting symbol
            nt!("S"),

            // vector of rules
            vec![
                // a rule that generates a sequence of non-terminals: "A B"
                nt_seq_rule!("S" => "A", "B"),
                nt_seq_rule!("B" => "A", "B", "N"),
                nt_seq_rule!("B" => "E"),
                t_or_rule!("E" => ""),

                // a rule that is an "or" of terminals: any letter from a-z
                t_or_rule!(
                    "A" => "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k",
                           "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v",
                           "w", "x", "y", "z"
                ),
                t_or_rule!("N" => "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"),
            ],
        )

        // generate a random string with this grammar
        println!("{}", grammar.gen());
    }

For a real-life example take a look at
[mexe](https://github.com/yds12/mexe/blob/master/tests/integration.rs).

## Links

* Documentation: [docs.rs](https://docs.rs/glc/latest)
* Crate: [crates.io](https://crates.io/crates/glc) and [lib.rs](https://lib.rs/crates/glc)
* Repository: [Github](https://github.com/yds12/glc)
