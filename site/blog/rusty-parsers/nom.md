---
date: '2018-09-22T00:00:00Z'
description: A look at various parsers in rust
slug: rusty-parsers-nom
tags:
- rust
- programming
- parsers
---

# Rusty Parsers - Nom

Nom is a combinator parser - which means it builds more complex parsers from
simpler sub parsers. There is no separate grammer like other parser libraries
use, each parser deals with parsing the input and translating it into some
useful struct. Each parser takes the signature
`fn parser(input: I) -> IResult<I, O, E>;` and you basically have to write each
one by hand. While you can write all of the parsers by hand, nom offers a bunch
of macros and convience functions to help make this much eaiser and faster.

Nom is the first parser I looked at as it seems to be one of the more popular
ones and lots of people recomending it from various places. In this post I am
not going to be walking you though creating a parser in rust and instead focus
on my opinion of the library (as someone completely new to parsers in general
and inperticular the pain points which I encountered while developing the
simple demo parsers (which you can find at the bottom of the post and [in this
repo].

This project was done using Nom v4.

[rust-parser]: /blog/rusty-parser
[in this repo]: https://gitlab.com/mdaffin/rusty-parsers/blob/master/src/nom.rs

## Getting started with nom

Getting started with nom is quite simple, the documentation offers some nice
simple _yet complete_ examples that you can use to get started. [The main
documentation] does an exellent job of describing how some of the common
parsers work and how you should use them.

The problems start when you want to dive a bit deeper and start implementing
your own parsers. The parts that are not mentioned in the main introductary
documentation are spotty, with some of the macro barly having anything more then the macro signature

## Implementation

Here is the full implementation of the nom parser.

```rust
#[macro_use]
extern crate nom;
extern crate rustyline;

mod repl;
#[cfg(test)]
mod tests;

use repl::Part;
use std::collections::HashMap;
use nom::*;
use nom::ErrorKind::Custom;

fn main() {
    repl::start();
}

pub fn parse(buffer: &str) -> Result<Option<Part>, String> {
    use nom::Err::{Error, Failure, Incomplete};
    match part(buffer) {
        Err(Incomplete(_)) => Ok(None),
        Err(Error(e)) => Err(render_error(e)),
        Err(Failure(e)) => Err(render_error(e)),
        Ok((remaining, _)) if !remaining.trim().is_empty() => {
            // Should not happen if the parser logic is correct
            panic!(
                "unexpected tailing characters: '{}' for '{}'",
                remaining, buffer
            );
        }
        Ok((_, part)) => Ok(Some(part)),
    }
}

fn render_error(err: Context<&str, u32>) -> String {
    use self::Context::Code;
    use self::ErrorKind::Custom;
    let (unexpected, expected) = match err {
        Code(a, Custom(1)) => (a, "')'"),
        Code(a, Custom(2)) => (a, "'='"),
        Code(a, Custom(3)) => (a, "wire_name"),
        Code(a, Custom(4)) => (a, "')' or wire_name"),
        Code(a, Custom(5)) => (a, "';'"),
        e => panic!("Unknown error: {:?}", e),
    };
    let unexpected = unexpected.chars().next().unwrap();
    format!("unexpected {:?}, expected {}", unexpected, expected)
}

named!(part<&str, Part>,
    terminated!(
        ws!(do_parse!(
            name: chip_name >>
            add_return_error!(Custom(1), tag!("(")) >>
            wire_pairs: wire_pairs >>
            add_return_error!(Custom(4), tag!(")")) >>
            ({
                let mut wires = HashMap::new();
                for (inner, outer) in wire_pairs {
                    wires.insert(inner, outer);
                }
                Part { name, wires }
            })
        )),
    add_return_error!(Custom(5), tag!(";")))
);

named!(wire_pairs<&str, Vec<(&str, &str)>>,
    separated_list!(
        tag!(","),
        separated_pair!(
            wire_name,
            return_error!(add_return_error!(Custom(2), tag!("="))),
            return_error!(add_return_error!(Custom(3), wire_name))
        )
    )
);

named!(wire_name<&str, &str>,
    ws!(recognize!(preceded!(
        alpha,
        take_while!(|c| is_alphanumeric(c as u8) || c == '_' )
    )))
);
named!(chip_name<&str, &str>, recognize!(preceded!(alpha, alphanumeric0)));
```

```
error: no rules expected the token `,`
  --> src/nom.rs:61:32
   |
61 |         return_error!(Custom(2), tag!("=")),
   |                                ^
```
