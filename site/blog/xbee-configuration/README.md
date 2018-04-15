---
aliases:
- /xbee-configuration/
- /posts/xbee-configuration/
date: '2012-11-17T00:00:00Z'
description: A beginners guide on configuring and communicating over two xbees.
slug: xbee-configuration
tags:
- xbee
title: XBee Configuration
---

For my third year project I will be using a bunch of XBee connected to various
devices, this post will talk about how to set up the XBee and what the various
config options do. To follow this post you will need the following equipment:

* [XBee](https://www.sparkfun.com/products/8665)\*
* [XBee Explorer USB]( https://www.sparkfun.com/products/8687)\*\*

\* I have tested this with the series 1 modules, but it should mostly work for the series 2 modules although some steps might be different.  
\*\* Or any equivalent way of connecting the XBee to a computer.

---

By default the XBee modules are configured in transparent mode where any data
sent on the Din line is transmitted over the radio and any data received on the
radio is put on the Dout line. This makes it very easy to send a receive data
without any special libraries, you just open a serial connection and send your
data.

## Notes

**\<CR\>** in this article (as well as other places on the internet) means 
_Carriage Return_, and you should hit the _Enter_ or _Return_ key in place of
it.

## Setup

Once you have your XBee plugged into the computer via the explorer you should
get a new serial port appear, normally /dev/ttyUSB0 on Linux or COM1 on Windows.
Open up your favourite serial program, in this post I will be using picocom, but
minicom or screen are just as good (Windows users can use putty) and connect to
the XBee serial port (/dev/ttyUSB0 or what ever it shows up as) at the default
baud rate of 9200:

```bash
picocom -lc /dev/ttyUSB0
```

Now anything you type should be transmitted by the radio and anything received
on the radio should be printed on the terminal. At the moment there wont be much
happening as the XBee modules are probably not configured to connect to each
other (if you have a second one plugged in that is).

## Command mode

In order to configure the XBee you need to switch to command mode, to do this
type +++ in the serial program and wait 1-2 seconds. Note: DO NOT hit enter
after the +++ or you will not enter the command mode and you will have to type
it again. You are now in command mode, anything you type now will not be sent
over the radios but will be used to configure the XBee. To exit command mode
either don't type anything for a few seconds or type "ATCN&lt;CR&gt;". While in
command mode you can read the current settings by typing the associated command
such as:

```bash
ATMY<CR>
0
```

or set it by adding a value before you hit return

```bash
ATMY1<CR>
OK
ATMY<CR>
1
```

if you miss type a command you get the ERROR message and can retype it again. If
you don't get a response then it likely means the command session has timed out
Note: the characters are sent over as you type them in picocom so you cannot use
backspace to correct typos and have to instead start the command again. You can
also chain together commands by dropping the AT after the first command and
separating them by commas like this,

```bash
ATMY1,WR,CN<CR>
```

However I find this less useful when in a terminal program that sends the
characters as you type them as you will get a response after ever comma or
return key which makes it harder to read the returned values.

Note that the changes will not be applied until the CN or AC command is issued
or the module is rebooted.

## Configuring the XBee

### Re-settings the XBee

The first thing you should do when configuring a new XBee module is reset the
device to its factory defaults. This will help prevent any configurations made
previously from silently messing up your project. Enter command mode and type

```bash
ATRE<CR>
```

Your devices settings should now have reset to their defaults.

### Saving Changes

The next thing you want to do is write these changes to the non-volatile part of
the XBee memory which can be done with the **WR** command. You should issue this
command when ever you have finished editing the commands to ensure that the
changes you made will be preserved after a reboot or power up.

```bash
ATWR<CR>
```

### Restarting the XBee

Should you wish to restart the device (without unplugging/powering it down that
it) you can issue the FR command to tell the device to do a software reset. This
is useful to do after you have configured your device (and saved the settings!)
to ensure the XBee is using the correct settings.

```bash
ATFR<CR>
```

### Useful Commands

Now the device is in a nice and clean state you can start to configure it to
your liking, the commands that are most interesting to begin with are the MY,
DH, DL, BD and ID commands as explained below.

**ID** - The pan ID of the network, it does not matter what value this is set to
as long as all devices in the same network use the same ID as XBee modules will
only talk to other XBee modules that have the same ID. If there are no other
XBees active near by you can leave this at the default value (3332) or you can
set it to what ever value you want to avoid conflicts with other XBee networks
near by.

**BD** - The baud rate to communicate at. Takes a value of 0-7 (1200-115200bps
or a custom amount) and defaults to 3 (9200). All XBees on the network should
have the same value and the default should be good enough for now. Note the
higher the value, the faster data can be sent, but the more likely you are to
lose packets. Also remember to restart the terminal application and reconnect
with the new baud rate once you apply this setting.

**MY** - The 16 bit address of the XBee module, each device in the network
should be given a unique address as this is what other XBees will refer to when
sending data.

**DH** and **DL** - The high and low bit of the destination address, set DH to 0
and DL to the MY value of the XBee you want to connect to. DH is used for 64bit
addressing and is not needed in this tutorial, for simple setups 16bit
addressing will probably be enough so you should make sure it is set to 0 and
then forget about it for now.

## Connecting two XBees

Now that we have seen some commands lets look at connecting two XBees to talk to
one another, so plug-in the first XBee and open up picocom and send the
following (remember to not press enter (or any other key) after the +++ until
you receive the OK response);

```bash
+++
OK
ATRE<CR>
OK
ATMY0,DH0,DL1,WR,CN
OKOKOKOKOK
```

Then connect the second  XBee and send the following;

```bash
+++
OK
ATRE<CR>
OK
ATMY1,DH0,DL0,WR,CN
OKOKOKOKOK
```

These will give the first XBee an address of 0 and tell it to send to 1 and the
second XBee an address of 1 and tell it to send to 0. If you now connect up both
XBees any message you send on one should be received on the other.

## Conclusion

You have now seen how easy it is to connect two XBee modules together and how
the transparent mode works, now this is fine for a single pair of XBees but it
gets hectic if you want to add more XBees to the network (for one you will have
to keep changing the DL address to talk to different XBee modules). In a future
post I will talk about API mode and how to set up a network coordinator which
will solve some of the problems with this simple set-up.
