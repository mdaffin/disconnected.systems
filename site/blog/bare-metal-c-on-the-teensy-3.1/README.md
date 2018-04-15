---
aliases:
- /bare-metal-c-on-the-teensy-3.1/
- /posts/bare-metal-c-on-the-teensy-3.1/
date: '2015-11-23T00:00:00Z'
description: A look at bare metal programming in c on the teensy 3.1 with out external
  dependencies.
slug: bare-metal-c-on-the-teensy-3.1
tags:
- c
- teensy
- arm
title: Bare Metal C on the Teensy 3.1
---

As a follow on from my previous post about writing [bare metal assembly on the
teensy 3.1](/bare-metal-assembly-on-the-teensy-3.1/) I wanted to see what it
would take to port it to C. There where a few bits missing from the assembly
example that are more important in the C port, which I will cover in this post.

The final source can be found in [this github
repository](https://github.com/mdaffin/embedded-examples/tree/master/teensy-3-c)
and only contains two files: the c source and the linker script.

## The Linker Script

The linker script is very similar to the assembly example, just with a couple of
additions. The `MEMORY` block is identical to the assembly example so we will
skip over it.

C places all `const` variables inside a section called `.rodata`, which we place
after the code section with by adding the following to the end of the `.text`
section in the `SECTIONS` block.

###### [layout.ld](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/layout.ld#L36-L46)

```text
SECTIONS {
    . = 0x00000000;
    .text : {
        KEEP(*(.vectors)) /* vectors must be placed first - page 63*/
        . = 0x400;
        KEEP(*(.flashconfig*)) /* flash configuration starts at 0x400 - page 569 */
        *(.startup*)
        *(.text*)
        *(.rodata*)
        . = ALIGN(4);
    } > FLASH
```

Next we define the `.data` section. This is where C will place all initialized
global variables, which can be modified so should be placed in `RAM`.

###### [layout.ld](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/layout.ld#L48-L55)

```text
    .data : {
        . = ALIGN(4);
        _sdata = .;
        *(.fastrun*)
        *(.data*)
        . = ALIGN(4);
        _edata = .;
    } > RAM AT > FLASH
```

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

###### [layout.ld](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/layout.ld#L29)

```text
_sflashdata = LOADADDR(.data);
```

Note that we place two bits in the `.data` section. `.data` which contains the
uninitialized variables and `.fastrun` which can contain any code that we want
copied to `RAM` so it can be loaded faster when executed.

The uninitialized variables are easier to deal with as we don't need to worry
about copying them from `FLASH`. C stores them in a section called `.bss`. So we
create that next, again storing the start and end in `_sbss` and `_ebss`.

###### [layout.ld](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/layout.ld#L57-L64)

```text
    .bss : {
        . = ALIGN(4);
        _sbss = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;
    } > RAM
```

## The C Code

The C code is a port of the assembly code and contains all the major parts
including, the exception vectors, flash configuration, start up code, the main
loop and functions for turning the led on/off and a simple delay.

We start with some macros definitions that will allow us to write to various
memory locations by name rather then their actual address.

###### [blink.c](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/blink.c#L30-L35)

```c
#define WDOG_UNLOCK  (*(volatile unsigned short *)0x4005200E) // Watchdog Unlock register
#define WDOG_STCTRLH (*(volatile unsigned short *)0x40052000) // Watchdog Status and Control Register High
#define GPIO_CONFIG  (*(volatile unsigned short *)0x40048038)
#define PORTC_PCR5   (*(volatile unsigned short *)0x4004B014) // PORTC_PCR5 - page 223/227
#define GPIOC_PDDR   (*(volatile unsigned short *)0x400FF094) // GPIOC_PDDR - page 1334,1337
#define GPIOC_PDOR   (*(volatile unsigned short *)0x400FF080) // GPIOC_PDOR - page 1334,1335
```

You should recognize these values from the assembly example and can all be found
in the programmers manual. They have all been brought to the top so their
definitions can be reused and to make the code easier to read.

Then we declare the linker script variables and the functions we will use later.

###### [blink.c](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/blink.c#L37-L53)

```c
extern unsigned long _sflashdata;
extern unsigned long _sdata;
extern unsigned long _edata;
extern unsigned long _sbss;
extern unsigned long _ebss;
extern unsigned long _estack;

void startup();
void nim_handler();
void hard_fault_handler();
void mem_fault_handler();
void bus_fault_handler();
void usage_fault_handler();
void loop();
void led_on();
void led_off();
void delay(int ms);
```

We define the exception vectors as an array of const function pointers and
assign the function we want to handle each interrupt. Like in the assembly
example we need to tell gcc that this code should be placed in the `.vectors`
section which is done with the attribute flag. The `used` attribute flag tells
gcc the the code is used and to not remove it during the optimization process.

###### [blink.c](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/blink.c#L55-L64)

```c
__attribute__ ((section(".vectors"), used))
void (* const _vectors[7])(void) = {
  (void (*)(void))((unsigned long)&_estack),
  startup,
  nim_handler,
  hard_fault_handler,
  mem_fault_handler,
  bus_fault_handler,
  usage_fault_handler
};
```

We then do something similar for the `.flashconfig` section using an array of
unsigned chars.

###### [blink.c](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/blink.c#L66-L70)

```c
__attribute__ ((section(".flashconfig"), used))
const unsigned char flashconfigbytes[16] = {
  0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
  0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF
};
```

The startup code is also similar to the assembly example but now also
initializes the global variables in ram. But first we unlock and
disable the watchdog.

The startup code expands upon the assembly example, it now also initializes the
global variables in ram for the rest of the program to use. But like in the
assembly we first need to unlock and disable the watchdog.

###### [blink.c](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/blink.c#L72-L76)

```c
__attribute__ ((section(".startup")))
void startup() {
  WDOG_UNLOCK  = ((unsigned short)0xC520);
  WDOG_UNLOCK  = ((unsigned short)0xD928);
  WDOG_STCTRLH = ((unsigned short)0x01D2);
```

Then we immediately setup the global variables before anything else attempts to
use them. This is simply done by copying the `.data` location in `FLASH` to its
location in `RAM`, then zeroing the `.bss` section in `RAM`.

###### [blink.c](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/blink.c#L78-L83)


```c
  unsigned long *src = &_sflashdata;
  unsigned long *dest = &_sdata;

  while (dest < &_edata) *dest++ = *src++;
  dest = &_sbss;
  while (dest < &_ebss) *dest++ = 0;
```

And the rest of startup simply configures the gpio pins as we did in the
assembly example before jumping into the loop.

###### [blink.c](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/blink.c#L85-L93)

```c
  // Enable system clock on all GPIO ports - page 254
  GPIO_CONFIG = ((unsigned short)0x00043F82); // 0b1000011111110000010
  // Configure the led pin
  PORTC_PCR5 = ((unsigned short)0x00000143); // Enables GPIO | DSE | PULL_ENABLE | PULL_SELECT - page 227
  // Set the led pin to output
  GPIOC_PDDR = ((unsigned short)0x20); // pin 5 on port c

  loop();
}
```

Our loop is also very similar to the assembly example, the major difference is
we initialize a variable to pass to delay. This is done simply to verify that
the `.data` section is initialize correctly by our startup code.

###### [blink.c](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/blink.c#L95-L103)

```c
int n = 1000; // Used to test if the data section is copied correctly
void loop() {
  while (1) {
    led_on();
    delay(n);
    led_off();
    delay(n);
  }
}
```

The rest of the functions do the same thing as they did in the assembly example.

###### [blink.c](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/blink.c#L105-L116)

```c
void led_on() {
  GPIOC_PDOR = ((unsigned short)0x20);
}

void led_off() {
  GPIOC_PDOR = ((unsigned short)0x0);
}

void delay(int ms) {
  volatile unsigned int i;
  for (i = 0; i <= ms * 2500; i++) {;}
}
```

Finally all of the exception handlers are defined to simply lockup the cpu by
busy looping.

###### [blink.c](https://github.com/mdaffin/embedded-examples/blob/master/teensy-3-c/blink.c#L118-L122)

```c
void nim_handler() { while (1); }
void hard_fault_handler() { while (1); }
void mem_fault_handler() { while (1); }
void bus_fault_handler() { while (1); }
void usage_fault_handler() { while (1); }
```

## Compile and upload

To compile and upload we swap out the assembler for the c compiler and add the `-nostdlib` and `-c` flags to stop gcc including the std libraries and to tell it to compile without linking.

```sh
arm-none-eabi-gcc -mcpu=cortex-m4 -mthumb -nostdlib -c -o crt0.o crt0.c
arm-none-eabi-ld -T layout.ld -o crt0.elf crt0.o
arm-none-eabi-objcopy -O ihex -R .eeprom crt0.elf crt0.hex
echo "Reset teensy now"
teensy-loader-cli -w --mcu=mk20dx256 crt0.hex
```

## Conclusion

Although more complete then the assembly example there are still some missing
bits. Most notably we have not setup the heap or malloc so cannot dynamically
allocate memory. I would still recommend using a more complete base for any real
project such as form the teensy project
([mk29dx128.c](https://github.com/PaulStoffregen/cores/blob/master/teensy3/mk20dx128.c),
[mk20dx256.ld](https://github.com/PaulStoffregen/cores/blob/master/teensy3/mk20dx256.ld))
which you will see share many similar parts as explained in this post.
