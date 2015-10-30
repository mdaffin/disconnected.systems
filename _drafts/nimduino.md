---
layout: post
title: Nimduino
---

There has been a few intresting projects over the past few years that have tried to bring laternitive languages to the embeded world. The [espruino](http://www.espruino.com/) and [micropython](https://micropython.org/) are two very interesting project that allow you to code in javascript and python directly on a microcontroller. However they have one large drawback, they only support their own boards and thus can only run them on a limited number of microcontrollers. However there has also been development in a couple of new languages designed to challange C/C++, rust and nim. These are two very intresting project for the microcontrollers as they will be able to run bare metal on any platform their underlying compiler supports (llvm and gcc respectivly).

This post I will look at nim and attempt to get it running on an Arduino UNO.

Brief look at nim
=================

So the first step is to try out nim and write a simple hello world program to check I have everything installed correctly.

After installing the latest version of [nim](http://nim-lang.org/download.html) (0.12.0 at the time of writing) it was trivial to get an example program up and running:

```bash
cat <<EOF >example.nim
# This is a comment
echo("What's your name? ")
var name: string = readLine(stdin)
echo("Hi, ", name, "!")
EOF
nim compile --run example.nim
```

There are quite a few tutorials on how to program in nim, so I will skip on to the more interesting parts.

Programming AVR without arduino
===============================

Before we start to look at how to compile and upload a nim program to and avr chip we first need to see how this works without the Arduino SDK. Fortunately this process is quite easy and the example below where adapted from [Balau's blog](https://balau82.wordpress.com/2011/03/29/programming-arduino-uno-in-pure-c/) on the subject. I recommend reading his blog post for more details about the process.

<div class="code-header">led.c</div>

```bash
#include <avr/io.h>
#include <util/delay.h>

#define BLINK_DELAY_MS 1000

int main (void)
{
 /* set pin 5 of PORTB for output*/
 DDRB |= _BV(DDB5);

 while(1) {
  /* set pin 5 high to turn led on */
  PORTB |= _BV(PORTB5);
  _delay_ms(BLINK_DELAY_MS);

  /* set pin 5 low to turn led off */
  PORTB &= ~_BV(PORTB5);
  _delay_ms(BLINK_DELAY_MS);
 }
}
```

Then to compile and upload to the Arduino UNO simply;

```bash
avr-gcc -Os -DF_CPU=16000000UL -mmcu=atmega328p -c -o led.o led.c
avr-gcc -mmcu=atmega328p led.o -o led
avr-objcopy -O ihex -R .eeprom led led.hex
# Change /dev/ttyACM0 to the serial port of your arduino
avrdude -F -V -c arduino -p ATMEGA328P -P /dev/ttyACM0 -b 115200 -U flash:w:led.hex
```

And thats it, the on board LED should now be slowly blinking away.

Compile and example nim program for avr
=======================================

There is currently a lack of documentation around writing nim for embedded devices despite it being one of listed features of the language. The only working example I could find of how to compile a nim program for avr was from this  [Github issue](https://github.com/nim-lang/Nim/issues/1964).

So let us try it out

<div class="code-header">hello.nim</div>

```nim
echo "Hello, world!"
```

<div class="code-header">panicoverride.nim</div>

```nim
proc printf(frmt: cstring) {.varargs, importc, header: "<stdio.h>", cdecl.}
proc exit(code: int) {.importc, header: "<stdlib.h>", cdecl.}

{.push stack_trace: off, profiler:off.}

proc rawoutput(s: string) =
  printf("%s\n", s)

proc panic(s: string) =
  rawoutput(s)
  exit(1)

{.pop.}
```

The above files can be converted to c with the following:

```bash
nim c -c --cpu:avr --os:standalone --deadCodeElim:on hello.nim
```

This will give you a directoy named `nimcache` with two c file inside, these can be compiled and uploaded to the Arduino UNO using the commands from our previous step

```bash
avr-gcc -Os -DF_CPU=16000000UL -mmcu=atmega328p -c -o nimcache/hello.o nimcache/hello.c
avr-gcc -Os -DF_CPU=16000000UL -mmcu=atmega328p -c -o nimcache/system.o nimcache/system.c
avr-gcc -mmcu=atmega328p nimcache/hello.o nimcache/system.o -o nimcache/hello
avr-objcopy -O ihex -R .eeprom nimcache/hello nimcache/hello.hex
avrdude -F -V -c arduino -p ATMEGA328P -P /dev/ttyACM0 -b 115200 -U flash:w:nimcache/hello.hex
```

And the led stops blinking - progress, but nim is able to directly compile to avr, so we should be able to skip the avr-gcc steps. In order to do this we need to specify a few options via the nim.cfg. I found these options by using the `--parallelBuild:1 --verbosity:2` flags to see how nim was compiling the program.

First I noticed it was using `gcc` not `gcc-avr`. This was fixed by adding the following

<div class="code-header">nim.cfg</div>

```
avr.standalone.gcc.path = "/usr/bin"
avr.standalone.gcc.exe = "avr-gcc"
avr.standalone.gcc.linkerexe = "avr-gcc"
```

I then noticed some of the flags where missing from the compiler and linker. This was fixed by adding the following

<div class="code-header">nim.cfg</div>

```
passC = "-Os"
passC = "-DF_CPU=16000000UL"
passC = "-mmcu=atmega328p"
passL = "-mmcu=atmega328p"
```

Finally I added a couple more options

<div class="code-header">nim.cfg</div>

```
cpu = "avr"
define = "release"
deadCodeElim = "on"
```

I could not get the os flag to work in the config, so that is the only one that needs to be passed on the command line. You can now compile and upload the program with

```bash
nim c --os:standalone hello.nim
avr-objcopy -O ihex -R .eeprom hello hello.hex
avrdude -F -V -c arduino -p ATMEGA328P -P /dev/ttyACM0 -b 115200 -U flash:w:hello.hex
```

Blink in nim
===============

Now its time to get nim to blink the led. We will need the `nim.cfg` and `panicoverride.nim` files from the previous steps. Then we need to create a small c library to talk to the Arduino that we can wrap with nim. This is just the c example above split into separate functions.

<div class="code-header">led.c</div>

```c
#include <avr/io.h>
#include <util/delay.h>

void led_setup(void) {
  DDRB |= _BV(DDB5);
}

void led_on() {
  PORTB |= _BV(PORTB5);
}

void led_off() {
  PORTB &= ~_BV(PORTB5);
}

void delay(int ms) {
  // Not the best way to do this, but it does not matter for this example
  for (int i = 0; i < ms; i++) {
    _delay_ms(1);
  }
}
```

And now for the nim version of blink.

<div class="code-header">blink.nim</div>

```nim
{.compile: "led.c".}
proc led_setup(): void {.importc.}
proc led_on(): void {.importc.}
proc led_off(): void {.importc.}
proc delay(ms: int): void {.importc.}

when isMainModule:
  led_setup();
  while true:
    led_on();
    delay(1000);
    led_off();
    delay(1000);
```

Finally the steps to compile and upload it, these are basically the same as above. Note that we led.c is compiled for us due to the  `{.compile: "led.c".}` line in the blink program.

```bash
nim c --os:standalone blink.nim
avr-objcopy -O ihex -R .eeprom blink blink.hex
avrdude -F -V -c arduino -p ATMEGA328P -P /dev/ttyACM0 -b 115200 -U flash:w:blink.hex
```
