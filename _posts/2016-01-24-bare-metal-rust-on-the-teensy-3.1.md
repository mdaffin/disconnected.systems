---
layout: post
title: Bare Metal Rust on the Teensy 3.1
description: A bare metal example of blink written in rust for the teensy 3.1
tags: [rust, teensy, arm]
excerpt_separator: <!--more-->
---

Now that we have a good understanding of basic bare metal programming using the
traditional languages I wanted to look at [rust](https://www.rust-lang.org/). In
this post I will port the [bare metal c](/bare-metal-c-on-the-teensy-3.1/)
example to rust with cargo, rusts dependency manager and build manager.

<!--more-->

The final source can be found in [this github
repository](https://github.com/james147/teensy-3-rust). It contains a fair few
more files and cargo enforces a stricter layout (ie the source must be in the
`src/` directory. Most of the addition files are for rust and are meant to make
life simpler in the long run. The final project structure is as follows:

~~~
├── build.rs
├── .cargo
│   └── config
├── Cargo.toml
├── layout.ld
├── src
│   └── main.rs
└── thumbv7em-none-eabi.json
~~~

The linker script
[`layout.ld`](https://github.com/james147/teensy-3-rust/blob/master/layout.ld)
is identical to the c version so we will skip over it, I recommend reading my
[previous post](/bare-metal-c-on-the-teensy-3.1/) for more details about it.

## The Rust Code: [`src/main.rs`](https://github.com/james147/teensy-3-rust/blob/master/src/main.rs)

This is a simple port of blink.c from the c example. We have moved it to src as
this is where cargo looks for our code and have renamed it to main.rs to tell
cargo we want to build a binary.

Rust makes use of feature guards to stop accidental use of unsable/experimental
features. Unfortunately bare metal programming still requires a few of these
features so we must state which ones we want to use.

* [lang_items](https://doc.rust-lang.org/book/lang-items.html): so we can define
  some functions that are needed by rust to work (these are normally defined in
  the standard libraries)
* [no_std](https://doc.rust-lang.org/book/no-stdlib.html): to disable the
  standard libraries as they require an operating system to work
* [core_intrinsics](https://doc.rust-lang.org/core/intrinsics/): to make use of
  the `core::intrinsics::volatile_store` which is normally wrapped by the
  standard libraries.
* [asm](https://doc.rust-lang.org/book/inline-assembly.html): to allow us to
  call inline assembly directly
* [start](https://gist.github.com/luqmana/fa40eb63ff653fdfb3cf): to allow us to
  override the entry point to our program

We then disable the standard library with `#![no_std]`. Then tell rust we want a
statically linked executable `#![crate_type="staticlib"]` and declare we want to
use `volatile_store` from `core::intrinsics`.

<div class="code-header"><a href="https://github.com/james147/teensy-3-rust/blob/master/src/main.rs#L1-L5">src/main.rs</a></div>

~~~
#![feature(lang_items,no_std,core_intrinsics,asm,start)]
#![no_std]
#![crate_type="staticlib"]

use core::intrinsics::{volatile_store};
~~~

And now some required language functions which just cause the code to halt if we encounter an error.

<div class="code-header"><a href="https://github.com/james147/teensy-3-rust/blob/master/src/main.rs#L7-L26">src/main.rs</a></div>

~~~
#[lang="stack_exhausted"] extern fn stack_exhausted() {}
#[lang="eh_personality"] extern fn eh_personality() {}
#[lang="panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> !
{
    loop {}
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() -> ()
{
    loop {}
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1() -> ()
{
    loop {}
}
~~~

The last bit of new code is the `lang_start` function. In truth the actual start
point is the function that reset vector points to: the second `ISRVector` for
the teensy 3. In this example it is the `startup` function, which unfortunately
needs a different signature then the one rust expects. Instead we create a
simple wrapper to satisfy the compiler.

<div class="code-header"><a href="https://github.com/james147/teensy-3-rust/blob/master/src/main.rs#L137-L143">src/main.rs</a></div>

~~~
#[start]
fn lang_start(_: isize, _: *const *const u8) -> isize {
    unsafe {
        startup();
    }
    0
}
~~~

The rest of the code is a direct port from the [C
example](https://gist.github.com/james147/f9132c388fae9ef5f5fe#file-blink-c).
The only real difference is the syntax, its function is identical the the C
version and is described in my [previous post](/bare-metal-c-on-the-teensy-3.1/).

One thing to note however is there is no volatile keyword in rust. Instead we
define the macros without it and use `volatile_store` to write the value and
stop the compiler optimizing out the code.

## Target Specification: [`thumbv7em-none-eabi.json`](https://github.com/james147/teensy-3-rust/blob/master/thumbv7em-none-eabi.json)

The rust compiler needs some information to tell it how to compile for the arm
architecture, the `thumbv7em-none-eabi.json` file is what does this. This file
was obtained from the
[zinc](https://github.com/hackndev/zinc/blob/master/thumbv7em-none-eabi.json)
project. More details on the options can be found
[here](http://smallcultfollowing.com/rust-int-variations/imem-umem/rustc_back/target/struct.TargetOptions.html)
but you only need to edit it if you want to target a different platform.

## Cargo

At this point we are able to build a rust program by downloading the rust core,
compiling it then compiling `src/main.rs` against the new rust core. If you want
to take this approach I recommend reading [this blog
post](http://www.hashmismatch.net/2015/05/18/pragmatic-bare-metal-rust.html) but
in this post we are going to look at compiling our program with cargo.

Cargo is the dependency manager/rust build tool. It will help automate some of
the steps required to build the application including downloading and building
our dependencies (currently only `rust-core`).

### Cargo Configuration: [`Cargo.toml`](https://github.com/james147/teensy-3-rust/blob/master/Cargo.toml)

This file tells cargo some basic details about our project, such as its name,
version, authors as well as the dependencies required to build the project
(`rust-libcore` in our case). It also tells cargo to use `build.rs` as our build
script.

### Build Script: [`build.rs`](https://github.com/james147/teensy-3-rust/blob/master/build.rs)

This file allows us to preform any custom build steps before cargo tried to
build our application. This is useful for building c components or running other
tools that generate files required to build our program. In this case we simply
tell rust to use the `OUT_DIR` as part of the linker search path.

### Cargo Options: [`.cargo/config`](https://github.com/james147/teensy-3-rust/blob/master/.cargo/config)

This file contains options used to customize cargo's behavior. It is used here
to specify the linker and archive tool for the arm processor. We could specify
these on the command line, but since they wont change it is nicer to put them in
a configuration file instead.

### Compile and upload

Compiling is very simple and unlike the c and assembly examples it doesn't get
more complex as the project grows as cargo handles this for us.

~~~bash
cargo build --target thumbv7em-none-eabi
arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabi/debug/blink blink.hex
echo "Reset teensy now"
teensy-loader-cli -w --mcu=mk20dx256 blink.hex
~~~

Note that this compiles a debug version of the application, to compile for
release you simply pass the `--release` flag to cargo. However when I tried to
do this rustc decided none of my code was 'used' and optimized it all away. I
could not find any satisfying solution to this. Most of the example I found used
some c code to handle the startup code and rust to handle the application logic
but I wanted to see if it was possible to avoid. I believe the
[zinc](https://zinc.rs/) project have gotten around it some how, but I was
unable to get their example to compile correctly and I cannot see from their
source how/if they achieved it.

## Conclusion

Overall this project took quite a bit longer then I expected. Rust has a higher
learning curve then I thought it had and it takes a while to get use to its
ownership model. It feels like getting your head around pointers for the first
time in c, except the compiler refuses to compile when you get it wrong. Which
in a way is better then your program randomly crashing for no apparent reason.

This use case for rust is still highly experimental, hence the need for
rust-nightly and prone to randomly breaking in newer version. I have already
seen this problem in most of the examples out there, they now just refuse to
compile against the latest version of rust-nightly, which will likely happen to
this one at some point. This makes the whole process more tedious then it should
be as allot of the information out there is out of date.

I don't feel bare metal/embedded rust is quite ready yet for a serious project.
It is still a very interesting language which I hope to play around with a bit
more on a more stable branch.
