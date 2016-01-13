---
layout: post
title: Bare Metal Assembly on the Teensy 3.1
description: A look at bare metal programming in assembly on the teensy 3.1 with out any external dependencies.
---

I started to look at bare metal programming on the Teensy 3.1 and found quite a
few examples mainly based off the work of [Karl
Lunt](http://www.seanet.com/~karllunt/bareteensy31.html). All of these examples
include several files and do not explain what they are for or where they are
obtained. I started to dig a bit deeper and found an nice guide to low level arm
programming [here](http://bravegnu.org/gnu-eprog/) which explained what some of
them where for. Then I found a minimal working example in pure assembly for the
Teensy 3.0
[here](https://forum.pjrc.com/threads/25762-Turn-the-LED-on-with-assembler-code-\(-Teensy-3-1-\)?p=47739&viewfull=1#post47739).
I also found the [programmers
manual](https://www.pjrc.com/teensy/K20P64M72SF1RM.pdf) for the MK20DX256VLH7
very useful.

I took the minimal assembly example above with what I learned from other
articles around the topic to give a more complete, but still minimal, example.
The final source can be found in [this github
repository](https://github.com/james147/embedded-examples/tree/master/teensy-3-assembler)
and only contains two files: the assembly source and the linker script, which I
will explain in this post.

<!--more-->

## Requirements

This post is about what is needed to get the Teensy up and running rather then a
guide to assembly programming so I assume you have a basic knowledge of
assembly. You will also require the arm-none-eabi toolkit, explicitly the
assembler `arm-none-eabi-as`, linker `arm-none-eabi-ld` and objcopy
`arm-none-eabi-objcopy` binaries. These can be obtained from most Linux
distribution's package managers or from inside a Arduino SDK's tools directory:
`$ARDUINO_SDK/hardware/tools/arm/bin`.

## The Linker script: [`layout.ld`](https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/layout.ld)

This file tells linker where the various bits of memory are located and tells it
where to put different bits of the code. There are two main blocks to the linker
script, the `MEMORY` block and the `SECTIONS` bock.

The `MEMORY` block tells the linker where sections of storage on chip are
located. The flash is located at the start of the chip `0x00000000` and on the
MK20DX256VLH7 it is 256K long. Where as on the MK20DX256VLH7 the ram starts at
0x1FFF8000 and is 64K long:

<code data-gist-id="d6fb7e91aa21d6943ef4" data-gist-file="layout.ld" data-gist-line="29-32"></code>

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/layout.ld#L29-L32">layout.ld</a></div>

~~~
MEMORY {
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
    RAM  (rwx) : ORIGIN = 0x1FFF8000, LENGTH = 64K
}
...
~~~

The values for these location can be found in the programmers manual on pages 63
and 90. Note that you can split up the flash and ram even more to give greater
control over the layout of code and give each section different permissions. For
example, you could make the second part of the flash read only to store data
without the worry about it being executed:

~~~
MEMORY {
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 128K
    RODATA (ro) : ORIGIN = 0x00020000, LENGTH = 128K
    RAM  (rwx) : ORIGIN = 0x1FFF8000, LENGTH = 64K
}
~~~

The SECTIONS block tells the linker where to place the various parts of
the program:

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/layout.ld#L34-L45">layout.ld</a></div>

~~~
...
SECTIONS {
    . = 0x00000000;
    .text : {
        KEEP(*(.vectors)) /* vectors must be placed first - page 63*/
        . = 0x400;
        KEEP(*(.flashconfig*)) /* flash configuration starts at 0x400 - page 569 */
        *(.startup)
        *(.text)
    } > FLASH

    _estack = ORIGIN(RAM) + LENGTH(RAM); /* stack pointer start */
}
~~~

`. = 0x000000000;` sets the current location to the start of the block.

`.text : {...} > FLASH` matches all the text (aka code) and tells it to place it
in the FLASH section defined above.

The first part of all arm chips is where the exception vectors are placed. These
hold locations that the arm chip will jump to an events occurs, such as an
interrupt firing or a memory fault occurs. For a full list of them see the table
on page 63 of the programmers manual. We tell the linker to place the vectors
first with `KEEP(*(.vectors))`. To break this down further:

* `KEEP(...)` tells the linker to not remove any
dead/duplicate code as we do not want it moving or skipping various vectors.
* `*(...)` matches any file, you could specify a file name to only include code
from within that file however you generally don't need to make use of this
feature.
* `.vectors` is the part of our code that we want to place here, we will
look at how to label the code when we look at the assembly file below.

Next `. = 0x400` causes us to skip to address `0x400` and tells the linker to
place the `.flashconfig` section here. This address and the values in this
section allow you to configure the protection settings of the flash, you can
read more about the values on page 569 of the programmers manual.

After the flashconfig the startup code is placed with `*(.startup)` and finally the
rest of the code with `*(.text)`.

Finally we set a variable `_estack` to point to the end of the ram which will be
used to set the stack pointer.

## The assembly code: [`blink.s`](https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/blink.s)

Arm assembly comes in two flavors, the 16bit thumb instruction set and the
full 32bit arm instruction set. With the first line of code `.syntax unified`
we well the assembler we are using a mix of the instruction sets.

As we discussed above, we need to define the exception vectors:

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/blink.s#L32-L40">blink.s</a></div>

~~~ assembly
...
    .section ".vectors"
    .long _estack  //  0 ARM: Initial Stack Pointer
    .long _startup //  1 ARM: Initial Program Counter
    .long _halt    //  2 ARM: Non-maskable Interrupt (NMI)
    .long _halt    //  3 ARM: Hard Fault
    .long _halt    //  4 ARM: MemManage Fault
    .long _halt    //  5 ARM: Bus Fault
    .long _halt    //  6 ARM: Usage Fault
...
~~~

The `.section ".vectors"` tells the assembler to place this bit of code in the
`.vectors` section described in the linker script above, which we placed at the
start of the flash section. Due to this it does not matter where in the file
this code is placed, it will always be placed at the start of the flash by the
linker script.

In this example we only really make use of the *Inital Program Counter* to tell
the chip where to start executing from a reset, here we tell it to jump to the
\_startup label which is defined below.

The *Inital Stack Pointer* tells the arm chip where to start the stack, which
we defined at the end of the ram in the linker script. However we do not
properly initialize or make use of the stack in this example.

The rest of the vectors defined just jump to an infinite loop to halt execution
on the chip. We have also skipped a whole bunch of other vectors that are
described on page 63 of the programmers manual as they will not be needed in
this example.

Next we place the `.flashconfig` section, which will be placed at `0x400` due
to our linker script described in the last section. This address and the values
are described in the programmers manual on page 569 but we are not making any
real use of these features in this example.

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/blink.s#L42-L47">blink.s</a></div>

~~~ assembly
    .section ".flashconfig"
    .long   0xFFFFFFFF
    .long   0xFFFFFFFF
    .long   0xFFFFFFFF
    .long   0xFFFFFFFE
~~~

Now we move on to the setup code. This will be placed after the `.flashconfig`
as we defined in the linker script. `_startup:` is the label that the arm chip
will jump to when it resets as we defined in the exception vectors above.

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/blink.s#L50-L53">blink.s</a></div>

~~~ assembly
    .section ".startup","x",%progbits
    .thumb_func
    .global _startup
_startup:
...
~~~

There are a few things we need to do to setup the arm chip, first we reset all
the registers to 0.

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/blink.s#L55-L67">blink.s</a></div>

~~~ assembly
...
    // Zero all the registers
    mov     r0,#0
    mov     r1,#0
    mov     r2,#0
    mov     r3,#0
    mov     r4,#0
    mov     r5,#0
    mov     r6,#0
    mov     r7,#0
    mov     r8,#0
    mov     r9,#0
    mov     r10,#0
    mov     r11,#0
    mov     r12,#0
...
~~~

The Teensy 3 has a watchdog, which is enabled by default. This will cause the
chip to reset if the watchdog is not reset frequently. We do not want to worry
about the watchdog in this example so we are going to disable it. This involves
disabling interrupts, unlocking the watchdog (so it can be configured) then
disable it before enabling interrupts again. You can read more about how to
configure the watchdog on page 463 of the programmers manual.

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/blink.s#L69-L83">blink.s</a></div>

~~~ assembly
...
    cpsid i // Disable interrupts

    // Unlock watchdog - page 478
    ldr r6, = 0x4005200E // address from page 473
    ldr r0, = 0xC520
    strh r0, [r6]
    ldr r0, = 0xD928
    strh r0, [r6]

    // Disable watchdog - page 468
    ldr r6, = 0x40052000 // address from page 473
    ldr r0, = 0x01D2
    strh r0, [r6]

    cpsie i // Enable interrupts
~~~

With that the general configuration of the chip is done. We can now configure
the parts of the chip we want to use and start running our application loop. In
this example that means to enable and set as an `OUTPUT` the GPIO pin the led
is connected to.

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/blink.s#L85-L98">blink.s</a></div>

~~~ assembly

    // Enable system clock on all GPIO ports - page 254
    ldr r6, = 0x40048038
    ldr r0, = 0x00043F82 // 0b1000011111110000010
    str r0, [r6]

    // Configure the led pin
    ldr r6, = 0x4004B014 // PORTC_PCR5 - page 223/227
    ldr r0, = 0x00000143 // Enables GPIO | DSE | PULL_ENABLE | PULL_SELECT - page 227
    str r0, [r6]

    // Set the led pin to output
    ldr r6, = 0x400FF094 // GPIOC_PDDR - page 1334,1337
    ldr r0, = 0x20 // pin 5 on port c
    str r0, [r6]
~~~

Our logic is very simple:

* Turn on the led
* Busy wait
* Turn off the led
* Busy wait
* Repeat

Which is done by the following loop.

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/blink.s#L100-L106">blink.s</a></div>

~~~ assembly
    // Main loop
loop:
    bl led_on
    bl delay
    bl led_off
    bl delay
    b loop
~~~

Rather then embedding logic in the loop above we have moved it into separate
functions to mimic an actual application closer. The two functions to turn the
led on and off are as follows.

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/blink.s#L108-L124">blink.s</a></div>

~~~ assembly
    // Function to turn the led off
    .thumb_func
    .global led_off
led_off:
    ldr r6, = 0x400FF080 // GPIOC_PDOR - page 1334,1335
    ldr r0, = 0x0
    str r0, [r6]
    mov pc, r14

    // Function to turn the led on
    .thumb_func
    .global led_on
led_on:
    ldr r6, = 0x400FF080 // GPIOC_PDOR - page 1334,1335
    ldr r0, = 0x20
    str r0, [r6]
    mov pc, r14
~~~

And the last function just causes the processor to busy wait for a reasonable
amount of time by counting down from a fairly large number.

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/blink.s#L126-L135">blink.s</a></div>

~~~ assembly
    // Uncalibrated busy wait
    .thumb_func
    .global delay
delay:
    ldr r1, = 0x2625A0
delay_loop:
    sub r1, r1, #1
    cmp r1, #0
    bne delay_loop
    mov pc, r14
~~~

Finally we have the busy wait which will cause the chip to lockup in cause any
of the interrupts we defined at the start trigger.

<div class="code-header"><a href="https://github.com/james147/embedded-examples/blob/master/teensy-3-assembly/blink.s#L137-L138">blink.s</a></div>

~~~ assembly
_halt: b _halt
    .end
~~~

## Compile and Upload

To compile and upload to the Teensy run:

~~~ bash
arm-none-eabi-as -g -mcpu=cortex-m4 -mthumb -o blink.o blink.s
arm-none-eabi-ld -T layout.ld -o blink.elf blink.o
arm-none-eabi-objcopy -O ihex -R .eeprom blink.elf blink.hex
echo "Reset teensy now"
teensy-loader-cli -w --mcu=mk20dx256 blink.hex
~~~

## Summary

This was a very informative experience for me, having never touched assembly or
done any bare metal programming on the arm before. There are still some bits
missing that are required by higher level languages or more complete programs
but is nice start to understanding what happens on the arm ship at the lowest
level. I hope to expand on this in the future and see what it takes to convert
the assembler to a higher level language such as C.

## References
1. [Karl Lunt - Bare-metal Teensy 3.x Development](http://www.seanet.com/~karllunt/bareteensy31.html)
2. [Vijay Kumar B. - Embedded Programming with the GNU Toolchain](http://bravegnu.org/gnu-eprog/)
3. [glock45 - Turn the LED on with assembler code ( Teensy 3.1 )](https://forum.pjrc.com/threads/25762-Turn-the-LED-on-with-assembler-code-\(-Teensy-3-1-\)?p=47739&viewfull=1#post47739)
