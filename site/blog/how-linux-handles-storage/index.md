---
date: '2018-04-06T00:00:00Z'
description: A look at how the Linux kernel handles storage of data from low level block devices to the file system hairachy.
tags:
- linux
- kernel
- filesystems
---

# How Linux Handles Storage

How disks are handled in Linux is based on some fairly simple building blocks
that can be assembled into some very complex layouts. In this post we will take
a look at what is going on under the hood at how the Linux Filesystem works and
how your files and directories are stored.

## Physcial Media

Fundemeatally, all storage media is designed to store a seqence of 1s and 0s.
To the device yhese have no meaning, its only job is to store them when asked
and retreve them what asked. To do this we need a way to addess every single
bit so that when the CPU asks for a specifix bit we know which one it means.
But disks are huge, modern disks can be multiple terrabytes in size.

## Partition Tables

### Master Boot Eecord (MBR)

### GNU??? Parition Table (GPT)

## Filesystems

## Kernel Filesystem Drivers

### Block Devices

#### Loopback Devices

### Tmpfs

### Proc/Sysfs/Configfs

### Fuser

### Network Shares

## Filesystem Hierarchy

### The Filesystem Root and Kernel loading

### Mounting

### Bind Mounting
