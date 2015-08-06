---
layout: post
title: Charlie Cube
---

So I have been wanted to build an LED Cube for quite some time and after researching how to make
them I came across two different techniques. The most common way is to
[multiplex](http://en.wikipedia.org/wiki/Multiplexing) the LEDs, but I found a interesting approach
by [Asher Glick and Kevin Baker](http://aglick.com/charliecube.html) that uses
[charlieplexing](http://en.wikipedia.org/wiki/Charlieplexing) instead. Both methods are techniques
used to drive a large number of leds via a smaller number of pins by taking advantage of the
persistence of vision effect meaning not all LEDs need to be on at the same time.

I decided to go for the charleplexing technique as it requires less components and looked simpler to
build.

The parts
---------

The charlie cube requires allot less parts then the multiplexing counterpart making it much cheaper
to make.

* 64x RGB LEDs (~£5-10 for 100 on ebay)
* 16x 100 Ohm resistors (<£1 on ebay)
* An Arduino UNO or similar (£5-20)
* Wire
* Veriboard (<£2 from any electronics shop)

The original design did not use any resistors, which is probably fine as the LEDs are never on 100%
of the time. But being my first cube I decided to play it safe. I recommend buying extra LEDs as
well as it is possible that some will not work or that you will accidentally kill a few during
assembly. Also, get frosted/diffused LEDs rather then the clear ones as they will look much better.

The micro controller can end up being the most expensive part of this build, an official Arduino UNO
costs over £20 but you can get cheaper variants to lower the cost. I just went for using a
[shrimping it](http://shrimping.it/blog/) kit which you can get for less then £5 as I already had a
bunch of them. The Arduino nano or micro are also a good alternatives.

Building the cube
-----------------

### Test the LEDs

Before you do anything else you should test the LEDs and you should do this at almost every stage of
the making as the last thing you want to is to discover a dead LED during your first test run. At
this stage it is easy, but very tedious. You just need to connect the longest ping (for common
cathode) to ground and the other three pins through a >220 Ohm resistor to +5V (just use your
Arduinos 5V pin). Though this is unlikely, if any LED does not give off a white light throw it away.
