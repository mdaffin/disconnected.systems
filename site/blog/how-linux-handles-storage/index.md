---
date: '2018-04-06T00:00:00Z'
description: A look at how the Linux kernel handles storage of data from low level block devices to the file system hairachy.
tags:
- linux
- kernel
- filesystems
---

# How Linux Handles Files

In Linux *almost* everything is a file, configuration, 


How disks are handled in Linux is based on some fairly simple building blocks
that can be assembled into some very complex layouts. In this post we will take
a look at what is going on under the hood at how the Linux Filesystem works and
how your files and directories are stored.

## Physcial Media

Fundemeatally, all storage media is designed to store a seqence of 1s and 0s.
To the device these have no meaning, its only job is to store them when asked
and retreve them what asked. To do this the disk and the OS need a way to addess every single
bit so that when the OS asks for a specific bit the disk knows which one to fetch. 

It is impracticle to address each individual bit however, so instead 

But disks are huge, modern disks can be hundreads of terrabytes in size.

[disk-sector]: https://en.wikipedia.org/wiki/Disk_sector
[hard-drive-knowledge-blocks-vs-sectors]: http://www.alphaurax-computer.com/computer-tips/hard-drive-knowledge-blocks-vs-sectors

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
