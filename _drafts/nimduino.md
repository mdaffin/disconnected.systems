---
layout: post
title: Nimduino
---

In the embedded world there are a shamefully small number of languages you can choose from. Namly C or C++. There are a few boards out there that support javascript as well, like the Esprino, but they are not hugly common.

There are a number of new languages that are comming out that are very intresting. I have been playing around allot with Go but unfortinatly it would be allot of work to port it to an embeded system. However, there are two very intresting alternitives, rust and nim that are more suited to the embeded world both of which I keep checking up on. Rust now has a fork desiened to run on avr and nim has builtin support for both avr and arm. So I have decided to take a deeper look into nim. After some reasurch it looks like people have been using nim on avr chips, but there is very little documentation around this except how to compile the code. In this series of posts I will be looking at it in more detail to see if nim will finally answer my question of is there a better language out there for embeded systems.

Brief look at nim
=================

So the first step is to try out nim. After installing nim

```bash
sudo pacman -S nim
```

it was trival to get an example program up and running:

```bash
cat <<EOF >example.nim
# This is a comment
echo("What's your name? ")
var name: string = readLine(stdin)
echo("Hi, ", name, "!")
EOF
nim compile --run example.nim
```

There are quite a few good tutorials on how to program in nim, so I will skip on to the more intresting parts

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

Then to compile and upload to the Arduino UNO simply:

```bash
avr-gcc -Os -DF_CPU=16000000UL -mmcu=atmega328p -c -o led.o led.c
avr-gcc -mmcu=atmega328p led.o -o led
avr-objcopy -O ihex -R .eeprom led led.hex
avrdude -F -V -c arduino -p ATMEGA328P -P /dev/ttyACM0 -b 115200 -U flash:w:led.hex
```

And thats it, we now have a blinking led on our Arduino UNO.

Compile and example nim program for avr
=======================================

The only working example I could find of how to compile a nim program for avr was from this  [Github issue](https://github.com/nim-lang/Nim/issues/1964).

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

Nim c libraries
===============

So, the next step is include the avr c libraries and use them from a nim program
