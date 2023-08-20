# glc

This crate's aim is to generate random expressions based on a context-free
grammar.

The acronym stands for "gramática livre de contexto" (*context-free grammar*).

## How to Use

```rust
use glc::{Grammar, t_or_rule, nt_seq_rule};

let grammar = Grammar(
    // starting symbol
    "S".into(),

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
```

A simplified version of the macro above is available:

```rust
// You may need to tune this parameter depending on how large is your grammar,
// since the `grammar` macro is recursive.
#![recursion_limit = "256"]
use glc::grammar;

let _grammar = grammar!{
    // The first non-terminal seen (head of the 1st rule) will be
    // the starting symbol (in this case: `S`).
    S => A B;
    B => A B N;
    B => E;
    E => "";
    // Or transform a non-terminal in one among many terminals
    A => "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k",
         "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v",
         "w", "x", "y", "z";
    N => "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
};

```

For a real-life example take a look at
[mexe](https://github.com/yds12/mexe/blob/master/tests/integration.rs).

## Links

* Documentation: [docs.rs](https://docs.rs/glc/latest)
* Crate: [crates.io](https://crates.io/crates/glc) and [lib.rs](https://lib.rs/crates/glc)
* Repository: [Github](https://github.com/yds12/glc)
