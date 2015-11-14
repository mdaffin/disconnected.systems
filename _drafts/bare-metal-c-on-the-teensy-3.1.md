---
layout: post
title: Bare Metal C on the Teensy 3.1
---

As a follow on from my previous post about writing [bare metal assembly on the
teensy 3.1](bare-metal-assembly-on-the-teensy-3.1/) I wanted to see what it
would take to port it to C. There where a few bits missing from the assembly
example that are more important in the C port, which I will cover in this post.



# The Linker Script: `layout.ld`

The linker script is very similar to the assembly example, just with a couple of
additions. The `MEMORY` block is identical to the assembly example so we will
skip over it.

We have added three new lines to the `.text` part of the `SECTIONS` block.

<div class="code-header">layout.ld</div>

~~~
        ...
        *(.rodata*)
        . = ALIGN(4);
        _etext = .;
        ...
~~~

The `.rodata` section is where C places all `const` variables, we simply put these
after our code. We then align to the byte boundary and set a variable `_etext`
that points to the end of our code and `const` data.

Next we define the `.data` section. This is where C will place all initialized
global variables, which can be modified so must be placed in `RAM`. This is done
with `> RAM` at the end of the block, however we have a problem, RAM is
volatile and will be empty when the chip first powers on.

Instead we want to store the values inside the `FLASH` section and copy them to
the `RAM` when we boot up. We can tell the linker to store the values with `AT >
FLASH`. Which causes the linker to reserve the section in `RAM` but to store the
values in `FLASH`. We will deal with copying the values when the chip boots in
C.

We store the start and end addresses of this block in the `_sdata` and `_edata`
variables. These point to the locations in RAM, but where are the values located
in FLASH? That is what the `_etext` variable holds, since the values are
appended to FLASH, they will start from the end of the last block, which we
stored the variable `_etext`.

<div class="code-header">layout.ld</div>

~~~
    ...
    .data : {
        . = ALIGN(4);
        _sdata = .;
        *(.fastrun*)
        *(.data*)
        . = ALIGN(4);
        _edata = .;
    } > RAM AT > FLASH
    ...
~~~

Now we have dealt with the initialized variables its time for the uninitialized
variables which C store in a section called `.bss`. So we create that next, again storing the start and end in `_sbss` and `_ebss`.

<div class="code-header">layout.ld</div>

~~~
    ...
    .bss : {
        . = ALIGN(4);
        _sbss = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;
    } > RAM
    ...
~~~

# The C code: `crt0.c`

We start with some definitions, some macros, the values from our linker script
and the forward decelerations of the functions we are going to write.

The macros are simply to put a name to the various memory location we are going
to be using and are the same ones we played with in the assembly example.

<div class="code-header">layout.ld</div>

~~~
#define WDOG_UNLOCK  (*(volatile unsigned short *)0x4005200E) // Watchdog Unlock register
#define WDOG_STCTRLH (*(volatile unsigned short *)0x40052000) // Watchdog Status and Control Register High
#define GPIO_CONFIG  (*(volatile unsigned short *)0x40048038)
#define PORTC_PCR5   (*(volatile unsigned short *)0x4004B014) // PORTC_PCR5 - page 223/227
#define GPIOC_PDDR   (*(volatile unsigned short *)0x400FF094) // GPIOC_PDDR - page 1334,1337
#define GPIOC_PDOR   (*(volatile unsigned short *)0x400FF080) // GPIOC_PDOR - page 1334,1335

extern unsigned long _etext;
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
...
~~~

Like in the assembly example we need to define the exception vectors. We define
them as an array of const function pointers and assign the function we want to
handle each interrupt. Like in the assembly example we need to tell gcc that
this code should be placed in the `.vectors` section which is done with the
attribute flag. The `used` attribute flag is also used to tell gcc not to remove
the code as it normally would unused code.

<div class="code-header">layout.ld</div>

~~~
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
~~~

We then do something similar for the `.flashconfig` section but with an array on
unsigned chars.

<div class="code-header">layout.ld</div>

~~~
__attribute__ ((section(".flashconfig"), used))
const unsigned char flashconfigbytes[16] = {
	0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF
};
~~~

The startup code should also look similar to the assembly example, we define a
function then unlock and disable the watchdog. Todo this we make use of the
macros we defined at the start but is functionally equivalent to the assembly
example.

<div class="code-header">layout.ld</div>

~~~
__attribute__ ((section(".startup")))
void startup() {
  WDOG_UNLOCK  = ((unsigned short)0xC520);
  WDOG_UNLOCK  = ((unsigned short)0xD928);
  WDOG_STCTRLH = ((unsigned short)0x01D2);
  ...
~~~

This next section is new, this is where we copy the initialized variables from
the `FLASH` section to `.data` in `RAM` and then initialize the `.bss` section
to all `0`.

<div class="code-header">layout.ld</div>

~~~
  ...
	unsigned long *src = &_etext;
	unsigned long *dest = &_sdata;

	while (dest < &_edata) *dest++ = *src++;
	dest = &_sbss;
	while (dest < &_ebss) *dest++ = 0;
  ...
~~~

And the rest of startup is from the assembly example and simply configures the
gpio pins so that we can blink the led before jumping into our infinite loop.

<div class="code-header">layout.ld</div>

~~~
  // Enable system clock on all GPIO ports - page 254
  GPIO_CONFIG = ((unsigned short)0x00043F82); // 0b1000011111110000010
  // Configure the led pin
  PORTC_PCR5 = ((unsigned short)0x00000143); // Enables GPIO | DSE | PULL_ENABLE | PULL_SELECT - page 227
  // Set the led pin to output
  GPIOC_PDDR = ((unsigned short)0x20); // pin 5 on port c

  loop();
}
~~~

Our loop is also very similar to the assembly example, the major difference is
we initialize a variable to pass to delay. This is done simply to verify that
the `.data` section is initialize correctly by our startup code.

<div class="code-header">layout.ld</div>

~~~
int n = 1000; // Used to test if the data section is copied correctly
void loop() {
  while (1) {
    led_on();
    delay(n);
    led_off();
    delay(n);
  }
}
~~~

We turn the led on and off in the same way as the assembly code and the delay was
changed to accept an argument but is otherwise the same.

<div class="code-header">layout.ld</div>

~~~
void led_on() {
  GPIOC_PDOR = ((unsigned short)0x20);
}

void led_off() {
  GPIOC_PDOR = ((unsigned short)0x0);
}

void delay(int ms) {
  for (unsigned int i = 0; i <= ms * 2500; i++) {;}
}
~~~

Finally all of the exception handlers are defined to simply lockup the cpu by busy looping.
<div class="code-header">layout.ld</div>

~~~
void nim_handler() { while (1); }
void hard_fault_handler() { while (1); }
void mem_fault_handler() { while (1); }
void bus_fault_handler() { while (1); }
void usage_fault_handler() { while (1); }
~~~

# Compile and upload

To compile and upload we only change the assembler to the c compiler and add the `-nostdlib` and `-c` flags to stop gcc including the std libraries and to tell it to compile without linking.

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
allocate space. I would still recommend using a more complete base for any real
work such as form the teensy project
([mk29dx128.c](https://github.com/PaulStoffregen/cores/blob/master/teensy3/mk20dx128.c),
[mk20dx256.ld](https://github.com/PaulStoffregen/cores/blob/master/teensy3/mk20dx256.ld))
which you will see share many similar parts as explained in this post.
