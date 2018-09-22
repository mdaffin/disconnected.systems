Recently I started a project where I needed to parse some text in the form of a hardware discription language (a way of describing how logic gates are connected on a chip). Nothing very complex as far as parsers are conserned, quite short input and a quite simple structure - quite liekly something that I could do by hand but I wanted to take this as an oppotinity to learn some of rusts parsing libraries.

I first tried nom, being one of the more popular libraries that lots of people seemed to rave about. I hit quite a few issues, and decided to take a step back and look at pest, another popular library. Quite quickly I ran into some other issues. I relised that this was not going to be as simple as I first thourght and decided to learn a selection of different libraries out there to compare and contrast them. I went back to nom, taking a subset of the full language and started to learn nom in more details, push through the problems and get a complete solution to my partital problem. Then repeate this process for pest, and a selection of other popular or intresting parser libraries.

As part of this I am looking at erganomics over preformance, there are other resources out there that benchmark the various libraries (and all seem to come to different conslusions). But I want to know about the developers side, how well are they documented? How nice are they to use? What are the best practices of using them? How well do they handle errors? In this series I will be attempting to answer these questions as well as gain a better understanding of parsing in general and create some minimal - but complete - examples for each of the libraries (as I found a lot of partial example and very few that tie everything togeather that were not hugly complex).

The language I will be parsing is a subset of a HDL language that is used by the [nand2tetris](https://www.nand2tetris.org/) course. The full HDL looks like:

```
CHIP Xor {
    IN a, b;
    OUT out;
    PARTS:
    Not(in=a, out=nota);
    Not(in=b, out=notb);
    And(a=a, b=notb, out=w1);
    And(a=nota, b=b, out=w2);
    Or(a=w1, b=w2, out=out);
}
```

But for this exersise I am only going to be implementing a line from the PARTS subsection, such as `Not(in=a, out=nota);`. This takes the form `<chip_name> ( <internal_wire>=<outter_wire>, <internal_wire>=<outter_wire> );`. White space can exist between any of the tokens and there can be any number of inner/outter wire pairs. For this we are going to ignore comments.

## Boiler plate code

For this exersise I built a simple repl and test suit that I will use to develop each parser, the only requirement is that each parser implements a function `pub fn parse(buffer: &str) -> Result<Option<Part>, String>` and parses the code into `Part` struct defiend in `repl.rs`. A string error is returned on parser error, `None` when more data is required by the parser (for multiline support) or the parsed part on a successful parse.

Each parser demo is defined as a separate binary in `Cargo.toml`.

`repl.rs`

```rust
use std::collections::HashMap;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use super::parse;

#[derive(Debug, PartialEq)]
pub struct Part<'a> {
    pub name: &'a str,
    pub wires: HashMap<&'a str, &'a str>,
}

pub fn start() {
    let mut buffer = String::new();
    let mut rl = Editor::<()>::new();

    loop {
        let prompt = if buffer.is_empty() { ">>> " } else { "... " };
        let line = match rl.readline(prompt) {
            Ok(line) => line,
            Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => break,
            // For other errors print them and exit with an error. This should not happen often and
            // means there is a problem with reading from stdin or the chars read are not UTF8.
            Err(e) => {
                eprintln!("Could not read from stdin: {}", e);
                ::std::process::exit(1);
            }
        };

        buffer.push_str(&line);
        // Input can span multiple lines and the grammer can deal with this but readline above
        // strips the trailing new line, so we add it back
        buffer.push('\n');

        match buffer.trim() {
            // No input, such as if the user just hits enter
            "" => {
                buffer.clear();
                continue;
            }
            "exit" => break,
            _ => (),
        }

        match parse(buffer.as_str()) {
            Ok(None) => continue, // Need more input
            Ok(Some(result)) => println!("{:?}", result),
            Err(msg) => eprintln!("{}", msg),
        }

        // Add the buffer to the history without the last newline, we do this for any complete
        // input, even if it is invalid so the user can correct any mistake they make.
        rl.add_history_entry(buffer.trim_right_matches('\n').to_string());
        buffer.clear();
    }
}
```

`tests.rs`:

```rust
use super::{parse, Part};
use std::collections::HashMap;

macro_rules! test_case {
    ($test:ident, $input:expr, None) => {
        #[test]
        fn $test() {
            assert_eq!(parse($input).unwrap(), None);
        }
    };
    ($test:ident, $input:expr, Err) => {
        #[test]
        fn $test() {
            assert!(parse($input).is_err());
        }
    };
    ($test:ident, $input:expr, $name:expr, [ $($inner:expr => $outer:expr),* ]) => {
        #[test]
        fn $test() {
            #![allow(unused_mut)]
            let mut wires = HashMap::new();
            $(wires.insert($inner, $outer);)*
            let expected = Part {
                name: $name,
                wires: wires
            };
            assert_eq!(parse($input).unwrap().unwrap(), expected);
        }
    };
}

test_case!(simple_1, "Foo();", "Foo", []);
test_case!(simple_2, "Bar();", "Bar", []);
test_case!(simple_3, "FooBar();", "FooBar", []);
test_case!(single_wire_1, "Foo(a=b);", "Foo", ["a"=>"b"]);
test_case!(single_wire_2, "Foo(in=in);", "Foo", ["in"=>"in"]);
test_case!(single_wire_3, "Foo(input=input);", "Foo", ["input"=>"input"]);
test_case!(single_wire_4, "FOO(INPUT=INPUT);", "FOO", ["INPUT"=>"INPUT"]);
test_case!(single_wire_5, "Foo(a_b=c_d);", "Foo", ["a_b"=>"c_d"]);
test_case!(multi_wire_1, "Foo(a=a,b=b);", "Foo", ["a"=>"a", "b"=>"b"]);
test_case!(multi_wire_2, "Foo(a=z,b=y,c=x);", "Foo", ["a"=>"z", "b"=>"y", "c"=>"x"]);

test_case!(whitespace_1, " Foo ( a = z , b = y ) ; ", "Foo", ["a"=>"z", "b"=>"y"]);
test_case!(whitespace_2, "  Foo  (  a  =  z  ,  b  =  y  )  ;  ", "Foo", ["a"=>"z", "b"=>"y"]);
test_case!(whitespace_3, "\tFoo\t(\ta\t=\tz\t,\tb\t=\ty\t)\t;\t", "Foo", ["a"=>"z", "b"=>"y"]);
test_case!(whitespace_4,
       "\t\tFoo\t\t(\t\ta\t\t=\t\tz\t\t,\t\tb\t\t=\t\ty\t\t)\t\t;\t\t",
       "Foo", ["a"=>"z", "b"=>"y"]
    );
test_case!(whitespace_5, "\nFoo\n(\na\n=\nz\n,\nb\n=\ny\n)\n;\n", "Foo", ["a"=>"z", "b"=>"y"]);
test_case!(whitespace_6,
       "\n\nFoo\n\n(\n\na\n\n=\n\nz\n\n,\n\nb\n\n=\n\ny\n\n)\n\n;\n\n",
       "Foo", ["a"=>"z", "b"=>"y"]
    );

test_case!(partial_1, "F", None);
test_case!(partial_2, "Foo", None);
test_case!(partial_3, "Foo(", None);
test_case!(partial_4, "Foo(a", None);
test_case!(partial_5, "Foo(a=b", None);
test_case!(partial_6, "Foo(a=b,", None);
test_case!(partial_7, "Foo(a=b,c", None);
test_case!(partial_8, "Foo(a=b,c=", None);
test_case!(partial_9, "Foo(a=b,c=d", None);
test_case!(partial_10, "Foo(a=b,c=d)", None);
test_case!(partial_11, "Foo(a=b)", None);
test_case!(partial_12, "Foo()", None);

test_case!(error_1, "Foo(;", Err);
test_case!(error_2, "Foo(a;", Err);
test_case!(error_3, "Foo;", Err);
test_case!(error_4, "Foo()a;", Err);
```