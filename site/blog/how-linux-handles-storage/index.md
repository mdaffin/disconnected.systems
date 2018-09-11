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

## Block Devices

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
