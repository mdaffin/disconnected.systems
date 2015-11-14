---
layout: post
title: Arduino Powered Rover
description: A simple arduino powered robot
---

Over the past couple of weeks I have been developing an arduino powered autonomous rover.

<!--more-->

## The Hardware

The rover consists of the following hardware all fairly cheap and readily available;

*   [Arduino uno](http://proto-pic.co.uk/arduino-uno/) or [Arduino pro mini 5v](http://proto-pic.co.uk/arduino-pro-mini-328-5v-16mhz/)&#42;
*   [Potentiometer](http://proto-pic.co.uk/trimpot-10k-with-knob/)
*   [2 bumper/latch switches](http://proto-pic.co.uk/omron-snap-action-switch/)
*   1 [Momentary switch](http://proto-pic.co.uk/momentary-push-button-switch-12mm-square/)
*   2 [Continuous Rotation Servo](http://www.hobbytronics.co.uk/springrc-sm-s4303r?keyword=servo)&#42;&#42;
*   [2 GP2Y0D805Z0F Digital Distance Sensor](http://www.hobbytronics.co.uk/sensors/sensors-proximity/sharp-distance-sensor-5cm)
*   4 AA batteries and [holder](http://proto-pic.co.uk/battery-holder-4xaa-square-terminated/)&#42;&#42;&#42;
*   3 [1k resistor](http://proto-pic.co.uk/1k-ohm-1-4-watt-resistor-pack-of-20/)
*   A proto board or bread board
*   Some wires and connectors
*   Something to mount everything on&#42;&#42;&#42;&#42;

&#42; I started out with the uno but later switched to the pro mini as it is easier to power. Most other arduinos should also work.  
&#42;&#42;You can buy continuous rotation servos, or you can modify normal servos for continuous use. There are lots of guides out there on how to do this so I will leave it at that for now.  
&#42;&#42;&#42;Or any ~5v power supply  
&#42;&#42;&#42;&#42; I used a sheet of expanded pvc but any rigid workable material should do.

## The Circuits

The circuits are very basic, below is a breakdown of each section. I found the Arduino UNO fairly
annoying to power with the servos, since it is recommended to have 7v+ to power the arduino on the
Vin pin and the servos are rated at 6v max. I solved this by connecting the battery directly to the
usb port on the arduino uno and later switch to a 5V pro mini.

### The Power Supply

I had some trouble with the power supply on the arduino uno in that the arduino uno is recommended
to be powered by 7-12v via the Vin pin or the 5.5in power plug. But the servos are rated at 4-6v and
cannot be powered by the arduino Vout pins as they draw too much current.

I solved this on the arduino by hacking up a an old USB B cable and using that to power the arduino
uno from a 5-6v power supply (4 AA batteries). Later on I switched to the arduino pro mini 5v, which
supports 5-12v on the RAW pin.

### The Servos

The servos control the rovers movement. They should be powered directly from the power source (4-6v)
rather the from the Arduino as they can draw more current then the Arduino can provide which can
cause it to randomly reset. The ground pins should be connected to the power source as well as the
Arduinos ground pins. The left servo to pin 5 on the Arduinos digital pins and the right servo to
pin 6.

[![Rover Servo layout]({{site.url}}/images/arduino-powered-rover/Rover-servos-288x300.png
"Rover Servos")]({{site.url}}/index.php/arduino-powered-rover/rover-servos/)

### The Bumpers and Switch

The Switch is used to start/stop the rover and the bumpers are used in object avoidance. One end of
the switch and bumpers should be connected to the ardunio's +5v pin and the other end to the
Arduinos digital pins 2 (switch), 3 (left bumper) or 4 (right bumper) as well as through a 1k
resistor to ground.

[![The bumpers and switch
circuits]({{site.url}}/images/arduino-powered-rover/Rover-bumpers-198x300.png
"Rover-Bumpers")]({{site.url}}/index.php/arduino-powered-rover/rover-bumpers/)

### The Range Finders

The range finders are used to detect table edges to avoid falling off surfaces and should be mounted
facing down at the front of the rover. I used an uncommon ir range finders in that they do not
report the distance back, but instead drive the input pins high if there is an object within 5cm or
low otherwise. This makes the code much simpler since we only really care about this cutoff point.
You could replace these with normal range finders, but you would have to modify the code to make use
of them (I recommend the [NewPing](http://code.google.com/p/arduino-new-ping/) library if you do
decide to use them).

The power pins should be connected to the Arduinos +5v pins and the ground to the ground pins. The
left range finder to the digital pin 7 and the right one to digital pin 8.

[![Range finders]({{site.url}}/images/arduino-powered-rover/Rover-Range-300x295.png
"Rover-Range")]({{site.url}}/index.php/arduino-powered-rover/rover-range/)

### The Potentiometer

The potentiometer is used to control the speed of the over. Its outer pins can be directly connected
to the arduinos ground and +5v power supply and the central pin to the Arduinos analog pin 0.

[![Potentiometer
circuit]({{site.url}}/images/arduino-powered-rover/Rover-Potentiometer-296x300.png
"Rover-Potentiometer")]({{site.url}}/index.php/arduino-powered-rover/rover-potentiometer/)

### The LEDs

You can optionally connect two LEDs from pins 12 and 13 to ground, or use the LED built into the
arduinos 13th pin which shows when the rover is running or not. The 12th pin is mostly used for
debugging so is not needed.

## The Software

The code for this project is available on [my github
account](https://github.com/james147/ArduinoRover). And can easily be built with

    mkdir build
    cd build
    cmake ..
    make
    make rover-upload # This step requires the arduino to be plugged into the computer

The code should be built then uploaded to the arduino and the rover should start moving when you hit
the switch (and the LED on pin 13 should light up when it is moving), press it again to stop it. If
the led lights up but it doesn't start moving, make sure the potentiometer isn't set to its minimum
position.

## Conclusion

You should now have a fully functional rover capable of avoiding obstacles and table edges that can
easily be expanded upon. Some ideas for expansions:

*   Add a [line sensor](http://proto-pic.co.uk/qre1113-line-sensor-breakout-digital/) to make the rover follow pre-layed paths
*   Add ranger finders as bumpers
*   Add some LRDs to make the rover follow lights
*   Create a remote control using IR emitter/detector
*   Create a remote control using XBee modules

If you would like to see me attempt any of these or any other idea you have or just want help
creating your own feel free to contact me and I will see what I can do.
