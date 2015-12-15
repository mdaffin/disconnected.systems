---
layout: post
title: Bare Metal Rust on the Teensy 3.1
---

Now that we have a good understanding of bare metal programming using the
triditional languages I wanted to look at rust. In this post I will port the
[bare metal c](bare-metal-c-on-the-teensy-3.1/) example to rust with cargo,
rusts dependency manager and build manager.


<!--more-->

The final source can be found in [this github
repository](https://github.com/james147/embedded-examples/tree/master/teensy-3-rust) with the following contents:

```
├── build.rs
├── .cargo
│   └── config
├── Cargo.toml
├── layout.ld
├── src
│   └── main.rs
└── thumbv7em-none-eabi.json
```

Now, this is a few more files needed then the c example, however most of them are for cargo and just make our life abit easier. 

The linker script
[`layout.ld`](https://github.com/james147/embedded-examples/blob/master/teensy-3-rust/layout.ld)
is identical to the c version so we will skip over it, see my [previous
post](bare-metal-c-on-the-teensy-3.1/) for more details about it.

# The Rust Code: [`src/main.rs`](https://github.com/james147/embedded-examples/blob/master/teensy-3-rust/src/main.rs)

This is a simple port of blink.c from the c example . We have moved it to src as
this is where cargo looks for our code and have renamed it to main.rs to tell
cargo we want to build a binary.

Rust makes use of feature guards to stop accdental use of unsable/experimental features. Unfortinutly bare metal programming still requires a few of these features so we must state which ones we want to use.

* [lang_items](https://doc.rust-lang.org/book/lang-items.html): so we can define some functions that are needed by rust to work (these are normally defined in the standard libraries)
* [no_std](https://doc.rust-lang.org/book/no-stdlib.html): to disable the standard libraries as they require an operating system to work
* [core_intrinsics](https://doc.rust-lang.org/core/intrinsics/): to make use of the `core::intrinsics::volatile_store` which is normally wrapped by the standard libraries.
* [asm](https://doc.rust-lang.org/book/inline-assembly.html): to allow us to call inline assembly directly
* [start](https://gist.github.com/luqmana/fa40eb63ff653fdfb3cf): to allow us to override the entry point to our program

We then disable the standard library with `#![no_std]`, tell rust we want a staticly linked executable `#![crate_type="staticlib"]` and decalre we want to use `volatile_store` from `core::intrinsics`.

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-rust/src/main.rs#L1-L5">src/main.rs</a></div>

~~~
#![feature(lang_items,no_std,core_intrinsics,asm,start)]
#![no_std]
#![crate_type="staticlib"]

use core::intrinsics::{volatile_store};
~~~

And now some required language functions which just cause the code to halt if we enconter an error.

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-rust/src/main.rs#L7-L26">src/main.rs</a></div>

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

The rest of the code is ported from the c example, first the macros so we can refer to specific memory locations by name rather then their address and the decleration of the variables from our linker script

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-rust/src/main.rs#L28-L43">src/main.rs</a></div>

~~~
macro_rules! GPIOC_PDOR   {() => (0x400FF080 as *mut u32);} // GPIOC_PDOR - page 1334,1335
macro_rules! WDOG_UNLOCK  {() => (0x4005200E as *mut u16);} // Watchdog Unlock register
macro_rules! WDOG_STCTRLH {() => (0x40052000 as *mut u16);} // Watchdog Status and Control Register High
macro_rules! GPIO_CONFIG  {() => (0x40048038 as *mut u32);}
macro_rules! PORTC_PCR5   {() => (0x4004B014 as *mut u32);} // PORTC_PCR5 - page 223/227
macro_rules! GPIOC_PDDR   {() => (0x400FF094 as *mut u32);} // GPIOC_PDDR - page 1334,1337
macro_rules! GPIOC_PDOR   {() => (0x400FF080 as *mut u32);} // GPIOC_PDOR - page 1334,1335

extern {
    static mut _sflashdata: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sbss: u32;
    static mut _ebss: u32;
    fn _estack();
}
~~~

# Compile and upload

To compile and upload we swap out the assembler for the c compiler and add the `-nostdlib` and `-c` flags to stop gcc including the std libraries and to tell it to compile without linking.

~~~bash
arm-none-eabi-gcc -mcpu=cortex-m4 -mthumb -nostdlib -c -o crt0.o crt0.c
arm-none-eabi-ld -T layout.ld -o crt0.elf crt0.o
arm-none-eabi-objcopy -O ihex -R .eeprom crt0.elf crt0.hex
echo "Reset teensy now"
teensy-loader-cli -w --mcu=mk20dx256 crt0.hex
~~~

# Conclusion

