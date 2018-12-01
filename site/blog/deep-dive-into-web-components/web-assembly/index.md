---
date: '2018-11-27T00:00:00Z'
description: "A look into writing a website with web components"
slug: deep-dive-into-webcomponents/web-assembly
tags:
- webdev
- web-components
- programming
---

# A Deep Dive into Web Components - The Web Assembly

Ok, so this is not really to do with web compoents directly, but I have been
wanted to play around with rust and web assembly for some time now in a more
complete project and this was a good excuse to try it out as well as to see how
these two new bits of the web stack can play together.

Yes, you could just use the [qrcodejs] library and have it intergrated with a
few lines of code. But where is the fun in that?

[qrcodejs]: https://davidshimjs.github.io/qrcodejs/

For those that are not aware, web assembly is one of the newer and exciting
standards that most modern browsers now support nativly. In summary it allows
browsers to directly run a subset of the javascript bytecode without the
javascript runtime. What this means is that you can now compile other languages
to web assembly and run them in the browser without compiling them to
javascript first (which is what coffeescript and typescript currently do).

What this allows is languages like C/C++, or any other language to run with
near native preformance. In this post I will be looking at [rust] which has 1st
class support for web assembly and a growing number of tooling that makes
working with it much simpler - as well as bing a very nice language overall.

I am not going to be introducing rust in this post, instead you can learn more
about it in the offical [rust book] but overall I hope the use of it here will
be minimal enough for those less familar with it to be able to read along
anyway.

And while we are mentioning books, the [rust wasm book] is also well worth a
read while being what most of this post will be going through (in a lot less
detail).

[rust]: https://www.rust-lang.org/en-US/
[rust book]: https://doc.rust-lang.org/book/
[rust wasm book]:https://rustwasm.github.io/book/

## Setup

First, [setup the rust tool chain] (aka `rustup` and `cargo`) and download the
beta version of rust\*

```bash
rustup toolchain install stable beta
```

Note, I am using beta rust here as I also want to explore the new 2018 edition
that is due to be released in a week or so and thus likely to be the defacto
standard very shortly. It is quite likely that by the time you are reading this
it will have already been released and you can stick with stable. The key thing
is that we need rust version 1.31 or greater.

[setup the rust tool chain]: https://www.rust-lang.org/en-US/install.html

Then, download, build and install `wasm-pack` - a tool which makes building and
packaging npm packages from rust source very simple.

```bash
cargo install wasm-pack
```

And create a new project.

```bash
mkdir src/lib/qr-code && cd $_
rustup override set beta # so that only this directory uses beta
cargo init --lib --edition 2018
```

Rust uses a config file `Cargo.toml` in much the same way that npm uses
`package.json` with the major difference it being toml rather than json.
To be able to compile to webassembly we must change some setting in it, adding
the `cdynlib` to the crate type which tells rust it can compile it to a c
dynamic library and adding the `wasm-bindgen` dependency which is used to help
generate some boiler plate code reuqired to be able to take from javascript to
rust and vice versa.

```toml
...

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2.28"
```

## Alert! Hello, World

Let's start with a simple example first to check everything is working as expected.

Inside `src/lib.rs` (in side the rust crate rather than our project):

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello() {
    alert("Hello, World!");
}
```

We create a single function `hello` that calls `alert` with the iconic message.
This alert function is defined as an external "C" function and is how rust
communicated with other languages over the c ffi api, we talk to javascript
over this same interface (since like most things, web assembly is a c like
interface).

The `#[wasm_bindgen]` annotation tells the wasm_bindgen package to produce a
bunch of boiler place for talking to webassembly. This could be written by hand
but it is quite a bit of code and has to do some very tedious things like
converting between the web assembly types (i32, i63, f32 and f64) to wide
variaty of rust types. Note that list is a esaustive list, wasm only has four
number types - that is it and any other value much be converted to one of these
types for javascript to talk to rust. This makes sending data between the two
languages quite a bit of work and so should generally be kept to a minimum.

But wasm_bindgen takes care of the actualy translations for us so we don't have
to really think too much about it.

To build the package run

```bash
wasm-pack build
```

And it will handle everything for us and create a nice npm package in ./pkg
(including some nice javascript wrappers) ready for importing into javascript.
