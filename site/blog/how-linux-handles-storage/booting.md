---
date: '2018-04-06T00:00:00Z'
description: A look at what happens to Linux as it boots
tags:
- linux
- kernel
- booting
---

# How Linux Boots

You press the power button, a rush of current floods in and the computer roars
to life. The hardware is in its reset state and begins their initlisation
process. The CPU has no instructions to execute, the RAM is empty, the disks
are idle and waiting to be read and the fans are wering away in a despritate
attempt to keep everything cool while being completlty unware of the systems
state.

At this point everything is in disarray, the main coordinator of the system,
the CPU gets its instructions from the RAM, which sits empty waiting for
instructions on what to store or retreve. But the preciouse instructions the
CPU craves so much are stored on somewhere on one of the systems disks. Worst
yet, the instructions the CPU needs to load more instructions from the disk are
also stored on the disk. A catch 22.

## The BIOS or UEFI chip

To save your computer from being a giant, expensive and power hungry paper
weight the UEFI chip steps and and storts out this mess. Like its bigger
brother the CPU, the UEFI chip is able to executre instructions and corrdinate
the rest of the hardware into some form of order. While much less powerful it
has one key advantage - inbuilt read only memory, or ROM.

This ROM contains the instructions the UEFI chip needs to do a number of
chores. The first of which is to bring some order to the chaios of uninitlised
devices now at its finger tips. It sends its probes out, questioning each
device to see what is there and what is not.

It is looking for a few things, a display, so it can tell the outside world it
is alive and well. A keyboard and mouse, to allow the outside world a small
influence in what happens next. Some disks, that will hopefully contain the
instructions its bigger brother hungers for.

It also needs to check the CPU, RAM and any other devices for glearing faults
that would result in everything catching fire and exploding, or at the very
least stop the system from booting. We call this the POST tests and it checks
to see if anything is misbehaving or missing. We can halt here if anything
critical is wrong, or ask the user to press the F1 key if anything useful is
missing, you know like the keyboard and they would like to try to boot
reguardless.

---

If everything is good, the human is given a eternal 3-5 seconds to affect the
path taken by hammering specific keys. Should they want to enter the setup and
mess with the default settings or change which disk to attempt to boot from.

After the user fails to hit the right keys, or was too distracted to hit any at
all the process of finding the bootloader code begins. This starts with the
disk boot order, which is saved to the UEFI's flash storage (the read/write
part of the ship used to store configurate and settings). Any new disks that
are not in this list are added to the end and any that are missing are skipped.

Loading the first avaiable disk,

---

- Look at the first avaiable disk
- Detemin if it is MBR or GPT
- If it is MBR then start executing the first 512bits of the disk

---

- Power on
- CPU initlisation
- CPU starts executing instruction from the UEFI chip
- UEFI firmware initlises any require hardware such as keyboard, mouse, display, disks and runs basic

---

## The Bootloader (GRUB?)

## The Kernel

## Initramfs

## PID 1 and systemd

## Networking

## Login manager

## X11 Desktop environment or Window manager

- https://www.centos.org/docs/rhel-rg-en-3/s1-grub-whatis.html
