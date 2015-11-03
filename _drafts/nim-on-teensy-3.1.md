---
layout: post
title: Nim on the teensy 3.1
---

After getting nim to work on the Arduino Uno I wanted to see what the process was like for an ARM based chip.

Bare Metal Programming on the Teensy 3.1
========================================
Like before we will first look at what it takes to program the teensy without the Arduino IDE. The examples here are a minimal example based of the more pragmatic posts on the subject by [Kevin Cuzner](http://kevincuzner.com/2014/04/28/teensy-3-1-bare-metal/), [Karl Lunt](http://www.seanet.com/~karllunt/bareteensy31.html) and [elegantcircuits](http://elegantcircuits.com/2015/02/03/bare-metal-programming-the-teensy-3-1-arm-development-board-without-the-arduino-ide/). 

~~~ bash
packer -S arm-none-eabi-gcc arm-none-eabi-binutils
~~~

<div class="code-header">blinky.c</div>

~~~ c
/*
 *  blinky.c for the Teensy 3.1 board (K20 MCU, 16 MHz crystal)
 *
 *  This code will blink the Teensy's LED.  Each "blink" is
 *  really a set of eight pulses.  These pulses give the actual
 *  system clock in Mhz, starting with the MSB.  A pulse is
 *  narrow for a 0-bit and wide for a 1-bit.
 *
 *  For a system clock of 72 MHz, blinks will read 0x48.
 *  For a system clock of 48 MHz, blinks will read 0x30.
 */

#include  "common.h"

#define  LED_ON        GPIOC_PSOR=(1<<5)
#define  LED_OFF    GPIOC_PCOR=(1<<5)


int  main(void)
{
    volatile uint32_t            n;
    uint32_t                    v;
    uint8_t                        mask;

    PORTC_PCR5 = PORT_PCR_MUX(0x1); // LED is on PC5 (pin 13), config as GPIO (alt = 1)
    GPIOC_PDDR = (1<<5);            // make this an output pin
    LED_OFF;                        // start with LED off

    v = (uint32_t)mcg_clk_hz;
    v = v / 1000000;

    while (1)
    {
        for (n=0; n<1000000; n++)  ;    // dumb delay
        mask = 0x80;
        while (mask != 0)
        {
            LED_ON;
            for (n=0; n<1000; n++)  ;        // base delay
            if ((v & mask) == 0)  LED_OFF;    // for 0 bit, all done
            for (n=0; n<2000; n++)  ;        // (for 1 bit, LED is still on)
            LED_OFF;
            for (n=0; n<1000; n++)  ;
            mask = mask >> 1;
        }
    }

    return  0;                        // should never get here!
}
~~~


