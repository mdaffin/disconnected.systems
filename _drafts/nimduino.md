---
layout: post
title: Nimduino
---

It has been a while since I last posted and wanted to try and get back into it. 

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

The arduino framework hides allot of the details about how to program avr/arm chips away from the user. This is greate for beginers but also means we first need to find out what it is doing for us under the hood. For the first step we will look at programming an arduino uno bare bones - without the aid of the arduino sdk or libraries.

For this first part we will write a simple blink program using raw avr and upload it to the arduino uno

https://balau82.wordpress.com/2011/03/29/programming-arduino-uno-in-pure-c/

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

This is one of the simplest program we can write that we can actually observe working. Our aim will be to port it to nim. But first lets compile and upload it

```bash
avr-gcc -Os -DF_CPU=16000000UL -mmcu=atmega328p -c -o led.o led.c
avr-gcc -mmcu=atmega328p led.o -o led
avr-objcopy -O ihex -R .eeprom led led.hex
avrdude -F -V -c arduino -p ATMEGA328P -P /dev/ttyACM0 -b 115200 -U flash:w:led.hex
``` 

Compile and example nim program for avr
=======================================

The only example I could find of how to compile a nim program for avr was from this minimal issue https://github.com/nim-lang/Nim/issues/1964

So I took copied the code into a directory and ran

nim c -c --cpu:avr --os:standalone --deadCodeElim:on hello.nim

This gave me a directory with two c files inside it. I then used the commands from the avr tutorial in the last step to compile the c files and upload it. with some minor adjustments it worked! the program uploaded and the led stopped blinking!

Now to make it blink again.

Nim c libraries
===============

So, the next step is include the avr c libraries and use them from a nim program
