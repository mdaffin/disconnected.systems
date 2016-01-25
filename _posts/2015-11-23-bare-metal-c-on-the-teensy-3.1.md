---
layout: post
title: Bare Metal C on the Teensy 3.1
---

As a follow on from my previous post about writing [bare metal assembly on the
teensy 3.1](/bare-metal-assembly-on-the-teensy-3.1/) I wanted to see what it
would take to port it to C. There where a few bits missing from the assembly
example that are more important in the C port, which I will cover in this post.

The final source can be found in [this github
repository](https://github.com/james147/embedded-examples/tree/master/teensy-3-c)
and only contains two files: the c source and the linker script.

<!--more-->

# The Linker Script: [`layout.ld`](https://github.com/james147/embedded-examples/blob/master/teensy-3-c/layout.ld)

The linker script is very similar to the assembly example, just with a couple of
additions. The `MEMORY` block is identical to the assembly example so we will
skip over it.

C places all `const` variables inside a section called `.rodata`, which we place
after the code section with by adding the following to the end of the `.text`
section in the `SECTIONS` block.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="layout.ld" data-gist-line="44-45"></code>

Next we define the `.data` section. This is where C will place all initialized
global variables, which can be modified so should be placed in `RAM`.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="layout.ld" data-gist-line="48-55"></code>

`RAM` is volatile so we cannot store the initial values of variables there
directly. Instead we want to reserve space in `RAM` for them, but actually store
them in the `FLASH` section. This will allow us to copy them from `FLASH` to
`RAM` at runtime when the chip resets. We tell the linker to do this with `> RAM
AT > FLASH`.

To copy the data at runtime we need to know the start and end address of the
data in `RAM`, which we store in the variables `_sdata` and `_edata`. We also
need to know where the data starts in `FLASH`, which we obtain using `LOADADDR`
and store in `_sflashdata`. This references the whole data block so much be
located outside of it, we just place it at the top for convenience.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="layout.ld" data-gist-line="29"></code>

Note that we place two bits in the `.data` section. `.data` which contains the
uninitialized variables and `.fastrun` which can contain any code that we want
copied to `RAM` so it can be loaded faster when executed.

The uninitialized variables are easier to deal with as we don't need to worry
about copying them from `FLASH`. C stores them in a section called `.bss`. So we
create that next, again storing the start and end in `_sbss` and `_ebss`.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="layout.ld" data-gist-line="57-64"></code>

# The C Code: [`blink.c`](https://github.com/james147/embedded-examples/blob/master/teensy-3-c/blink.c)

The C code is a port of the assembly code and contains all the major parts
including, the exception vectors, flash configuration, start up code, the main
loop and functions for turning the led on/off and a simple delay.

We start with some macros definitions that will allow us to write to various
memory locations by name rather then their actual address.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="blink.c" data-gist-line="30-35"></code>

You should recognize these values from the assembly example and can all be found
in the programmers manual. They have all been brought to the top so their
definitions can be reused and to make the code easier to read.

Then we declare the linker script variables and the functions we will use later.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="blink.c" data-gist-line="37-53"></code>

We define the exception vectors as an array of const function pointers and
assign the function we want to handle each interrupt. Like in the assembly
example we need to tell gcc that this code should be placed in the `.vectors`
section which is done with the attribute flag. The `used` attribute flag tells
gcc the the code is used and to not remove it during the optimization process.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="blink.c" data-gist-line="55-64"></code>

We then do something similar for the `.flashconfig` section using an array of
unsigned chars.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="blink.c" data-gist-line="66-70"></code>

The startup code is also similar to the assembly example but now also
initializes the global variables in ram. But first we unlock and
disable the watchdog.

The startup code expands upon the assembly example, it now also initializes the
global variables in ram for the rest of the program to use. But like in the
assembly we first need to unlock and disable the watchdog.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="blink.c" data-gist-line="72-76"></code>

Then we immediately setup the global variables before anything else attempts to
use them. This is simply done by copying the `.data` location in `FLASH` to its
location in `RAM`, then zeroing the `.bss` section in `RAM`.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="blink.c" data-gist-line="78-83"></code>

And the rest of startup simply configures the gpio pins as we did in the
assembly example before jumping into the loop.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="blink.c" data-gist-line="85-93"></code>

Our loop is also very similar to the assembly example, the major difference is
we initialize a variable to pass to delay. This is done simply to verify that
the `.data` section is initialize correctly by our startup code.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="blink.c" data-gist-line="95-103"></code>

The rest of the functions do the same thing as they did in the assembly example.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="blink.c" data-gist-line="105-116"></code>

Finally all of the exception handlers are defined to simply lockup the cpu by
busy looping.

<code data-gist-id="f9132c388fae9ef5f5fe" data-gist-file="blink.c" data-gist-line="118-122"></code>

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

Although more complete then the assembly example there are still some missing
bits. Most notably we have not setup the heap or malloc so cannot dynamically
allocate memory. I would still recommend using a more complete base for any real
project such as form the teensy project
([mk29dx128.c](https://github.com/PaulStoffregen/cores/blob/master/teensy3/mk20dx128.c),
[mk20dx256.ld](https://github.com/PaulStoffregen/cores/blob/master/teensy3/mk20dx256.ld))
which you will see share many similar parts as explained in this post.
