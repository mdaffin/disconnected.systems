---
date: "2018-09-16T00:00:00Z"
description: A look at implementing a simple nom based REPL application in rust.
slug: repl-in-rust-with-nom
tags:
  - rust
  - parser
---

# A simple REPL written in rust with the nom parser

REPLs are interactive programs that take some input from a user, evaluate that
input, print it out then loop back to the beginning. Most, if not all,
interperated languages and a REPL and these types of programs are useful in
quite a few different contexts. In this post we are going to build a very basic
REPL in rust making use of the nom parser library for decoding the users input.

The goal in this post is to take a detailed look at nom and how to build it
into an interactive application.

Nom is a _parser combinator library_ which basically means a parser is built up
from smaller sub parsers each responsible for parsing smaller parts of some
input. It is one of the more popular parser libraries for rust as well as rated
as one of the fastest.

## The Language

For this application we are going to implement a simple maths application able
to add, substract, multiple and divide different values. In addition to this
expressions can be grouped with `(` and `)` and each expression is line
terminated (so long as it is not preceded by an operator or opening bracer).
Any ammount of white space is allowed between the tokens. `>>>` is the main
prompt and `...` is used for partitally complete or multiline statements.

```
>>> 1+2
3
>>> 1+2+(3*4)
15
>>> 6 / 2
3
>>> 2 +
... 4 + (
... 6 - 5
... )
7
```

The focus here is not the language itself, but how we implement it with nom so
I am explicitly limiting this grammer as to keep things simple while still
being able to see different aspects of nom in use (such as how it handles
partial input).

## The REPL

The read, print and loop parts are all really easy to implement thanks to the
[rustyline] package.

```rust
#[macro_use]
extern crate rustyline;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        match &rl.readline(">>> ") {
            Err(e) => panic!(e),
            Ok(line) => println!("{:?}", line),
        }
    }
}
```

This forms our main loop and in it rustyline will print our prompt, wait for
the user to enter a line of text and return us the line or and error if it
failed for any reason. This is the bulk of the REPL, the only bit left is to
parse and evaluate the user input.

### Handling errors and exiting cleanly

But before we do that lets handle that error more cleanly, we don't want our
program panicing everying time the user wants to exit the application.

There are various errors returned by rustyline, most are due to underlying I/O
errors which we cannot not do much about but print a message and exit.

```rust
        match &rl.readline(">>> ") {
            ...
            Err(e) => {
                eprintln!("Could not read from stdin: {}", e);
                std::process::exit(1);
            }
        }
```

Now, two common errors we do want to handle separatly are for EOF which happens
when the user hits `CTRL+D` and Interrupted for when the user presses `CTRL+C`
(rustlyline stops this from triggering a SIGTERM signal so we need to implement
its behaviour ourselves).

```rust
use rustyline::error::ReadlineError;

fn main() {
    ...

    loop {
        match &rl.readline(">>> ") {
            ...
            Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => break,
            ...
        }
    }
}
```

Now, it would also be nice to type `exit` as another way to exit he program.
This one we could handle with the parser but it is simpler to handle it
directly in the loop.

```rust
        match &rl.readline(">>> ") {
            ...
            Ok(line) if line.trim() == "exit" => break,
            ...
        }
```

### Partial input and buffering

We also want to handle partial input, that is when the user partly types in an
expression and the parser needs more input before it can parse it. For this we
need a buffer to store the previous lines between each loop.

```rust
fn main() {
    let mut buffer = String::new();
    ...

    loop {
        ...
    }
}
```

When we get a new line we need to append it to this buffer along with the
newline that was stripped by rustyline (as the newline char will be required by
the parser). Then we print and clear the buffer which we will make conditional
later based on the output of the parser.

```rust
        match &rl.readline(">>> ") {
            ...
            Ok(line) => {
                buffer.push_str(line);
                buffer.push('\n');
                println!("{:?}", &buffer);
                buffer.clear();
            }
            ...
        }
    }
}
```

We also want to ignore empty lines as there is nothing to process and it is not
really a valid statement but also not really an error. This is done by trimming
the buffer and seeing if it is empty.

```rust
            Ok(line) => {
                buffer.push_str(line);
                if buffer.trim().is_empty() {
                    continue;
                }
                buffer.push('\n');
                ...
            }
```

### Command History

One very nice feature of rustyline is history, this allows the user to use the
up/down arrows to select previous entries, much like you are use to from
shells and other popular REPLs. This feature is already enabled and all we need
to do is tell rustyline which lines to save.

```rust
            Ok(line) => {
                ...
                println!("{:?}", &buffer);
                rl.add_history_entry(buffer.clone());
                buffer.clear();
            }
```

Now we have a buffer we can tell if we have a partial input or not and print
the appropate prompt by checking to see if the buffer is empty.

```rust
fn main() {
    ...
    loop {
        let prompt = if buffer.is_empty() {
            ">>> "
        } else {
            "... "
        }
        match &rl.readline(prompt) {
            ...
        }
    }
}
```

### Input Processing

Finally we will complete out main function with a stub for processing the
input, we will expand this later when we look at the parser but for now it will
take the buffer and return an `Option<String>` which we will to indicate if
more input is needed if `None` is returned or a sanitised version of the buffer
with whitespace stripped if the parsing was successful. It will also handle
printing the result or any errors returned by the parser.

```rust
fn main() {
    ...
    loop {
        ...
        match &rl.readline(prompt) {
            Ok(line) => {
                ...
                buffer.push('\n');
                if let Some(statement) = process_input(buffer.as_str()) {
                    rl.add_history_entry(statement);
                    buffer.clear();
                }
            }
        }
    }
}

fn process_input(buffer: &str) -> Option<String> {
    Some(buffer.replace("\n", ""))
}
```

## The Abstract Syntax Tree (AST)

Well, at least as much as you can call it that. Nom allows you to parse things
into any datastructure you wish whish allows you to parse into very specific
structures if you want to and make extensive use of rusts strong type system.

```rust
    #[derive(Debug)]
    pub enum Operator {
        Add,
        Subtract,
        Multiply,
        Divide,
    }
```

```rust
mod ast {
    #[derive(Debug)]
    pub enum Expr {
        Number(f64),
        Operation(Box<Expr>, Operator, Box<Expr>),
    }
```

```rust
    impl Expr {
        /// Evaluate the expression
        pub fn eval(&self) -> f64 {
            use self::Expr::*;
            use self::Operator::*;
            match self {
                Number(n) => *n,
                Operation(l, Add, r) => l.eval() + r.eval(),
                Operation(l, Subtract, r) => l.eval() - r.eval(),
                Operation(l, Multiply, r) => l.eval() * r.eval(),
                Operation(l, Divide, r) => l.eval() / r.eval(),
            }
        }
    }
}
```

[rustyline]: TODO

## Appendix

```rust
#[macro_use]
extern crate nom;
extern crate rustyline;
use rustyline::error::ReadlineError;

fn main() {
    let mut buffer = String::new();
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        let prompt = if buffer.is_empty() {
            ">>> "
        } else {
            "... "
        }
        match &rl.readline(prompt) {
            // Exit if the user types "exit"
            Ok(line) if line.trim() == "exit" => break,
            // Exit on Eof (ctrl+d) or if the user hits Ctrl+c
            Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => break,
            Ok(line) => {
                buffer.push_str(line);
                if buffer.trim().is_empty() {
                    continue;
                }
                // The new line is stripped by readline, so we add it back as it is part of our
                // grammer to indicate the statement termination
                buffer.push('\n');
                if let Some(statement) = process_input(buffer.as_str()) {
                    rl.add_history_entry(statement);
                    buffer.clear();
                }
            }
            // For other errors print them and exit with an error. This should not happen often and
            // means there is a problem with reading from stdin, the chars read are not UTF8.
            Err(e) => {
                eprintln!("Could not read from stdin: {}", e);
                std::process::exit(1);
            }
        }
    }
}

/// Process the input buffer with the parser. It returns the complete and witespace-stripped
/// statement it decoded.
fn process_input(buffer: &str) -> Option<String> {
    match parser::statement(buffer) {
        Err(nom::Err::Incomplete(_)) => return None,
        Err(nom::Err::Error(e)) => println!("{}", parser::render_error(e)),
        Err(nom::Err::Failure(e)) => println!("{}", parser::render_error(e)),
        Ok((remaining, _)) if !remaining.trim().is_empty() => {
            // Should not happen if the parser logic is correct
            panic!(
                "unexpected tailing characters: '{}' for '{}'",
                remaining, buffer
            );
        }
        Ok((_, value)) => {
            println!("{}", value.eval());
        }
    }
    Some(buffer.replace("\n", ""))
}

mod ast {
    /// Any valid expression.
    #[derive(Debug)]
    pub enum Expr {
        /// A single 64 bit floating point number. This is the leaf node of the AST, all
        /// expressions eventually end in a number.
        Number(f64),
        /// Some operation, which consists of a left and right expression and some operation to
        /// appliy to them. The left and right expressions are nestable and will eventually resolve
        /// to some number.
        Operation(Box<Expr>, Operator, Box<Expr>),
    }

    /// An operator that can be used in an expression
    #[derive(Debug)]
    pub enum Operator {
        /// Adds both expressions together
        Add,
        /// Subtracts the right expression from the left expression
        Subtract,
        /// Multiplies both expressions together
        Multiply,
        /// Adds the left expression to the right expression
        Divide,
    }

    impl Expr {
        /// Evaluate the expression
        pub fn eval(&self) -> f64 {
            use self::Expr::*;
            use self::Operator::*;
            match self {
                Number(n) => *n,
                Operation(l, Add, r) => l.eval() + r.eval(),
                Operation(l, Subtract, r) => l.eval() - r.eval(),
                Operation(l, Multiply, r) => l.eval() * r.eval(),
                Operation(l, Divide, r) => l.eval() / r.eval(),
            }
        }
    }
}

mod parser {
    use ast::{Expr, Operator};
    use nom::{double_s, Context, ErrorKind, multispace0};

    /// Consumes whitespace between each token, similar to ws! but does not consume newlines.
    macro_rules! sp (
        ($i:expr, $($args:tt)*) => ({
            use nom::Convert;
            use nom::Err;
            use nom::space0;

            match sep!($i, space0, $($args)*) {
                Err(e) => Err(e),
                Ok((i1,o))    => {
                    match space0(i1) {
                        Err(e) => Err(Err::convert(e)),
                        Ok((i2,_))    => Ok((i2, o))
                    }
                }
            }
        })
    );

    /// Parses a single statement which is any expression followed by a newline.
    named!(pub statement<&str, Expr>, terminated!(
        expr,
        add_return_error!(ErrorKind::Custom(1), tag!("\n"))
    ));

    /// Parses a single expression which is one or more numbers or sub expressions seperated by
    /// some operator.
    named!(expr<&str, Expr>, sp!(do_parse!(
        left: operand >>
        right: many0!(pair!(terminated!(operator, multispace0), return_error!(operand))) >>
        (right.into_iter().fold(left, |l, (o, r)| Expr::Operation(Box::new(l), o, Box::new(r))))
    )));

    /// Parses a sub expressions which is any expression surrounded by `(` and `)`.
    named!(subexpr<&str, Expr>, delimited!(tag!("("), ws!(expr), tag!(")")));

    /// Parses a single 64bit floating point number.
    named!(number<&str, Expr>, map!(double_s, |i| Expr::Number(i)));

    /// One side of an operation, essentially a number of sub expression
    named!(operand<&str, Expr>, add_return_error!(ErrorKind::Custom(2), alt!(number | subexpr)));

    /// Parses a single basic math operator, `+` (addition), `-` (subtraction), `*`
    /// (multiplication) or `/` (division).
    named!(operator<&str, Operator>, alt!(
        value!(Operator::Add, tag!("+")) |
        value!(Operator::Subtract, tag!("-")) |
        value!(Operator::Multiply, tag!("*")) |
        value!(Operator::Divide, tag!("/"))
    ));

    /// Converts an error to a human readable message.
    pub fn render_error(err: Context<&str, u32>) -> String {
        use self::Context::Code;
        use self::ErrorKind::Custom;
        match err {
            Code(a, Custom(1)) => format!(
                "unexpected {:?}, was expecting a operator ('+' '-' '*' '/') or new line",
                a.chars().next().unwrap()
            ),
            Code(a, Custom(2)) => format!("expected a number or sub expression, found {:?}", a),
            e => panic!("Unknown error: {:?}", e),
        }
    }

}
```
