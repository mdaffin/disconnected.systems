---
date: '2018-04-06T00:00:00Z'
description: A look at how the Linux kernel handles storage of data from low level block devices to the file system hairachy.
tags:
- linux
- kernel
- filesystems
---

# How Linux Handles Files

One of the fundermental parts of Linux is the filesystem heirarchy. There are a lot of guides out there explaining the purpose of each directory in it, but none as to how it actually works, how are directories mounted, how are files stored, how to partitions and filesystems function
 In this article we are going to explore these details and look at how linux manages files, folders and stores these on disks.

## The Filesystem Hierarchy

Unlike Windows, Linux (and Unix systrms overall) use a single directory structure which starts at the root aka `/`. Everything is located somewhere inside this directory tree.

::: note:::
This is distint from `/root` (aka "slash root") which is the root users home directoy inside the root of the directory structure.
:::

One of the key concepts in Linux is that almost everything is a file, or more accruatly can be interfaced with via a file handle. this not only includes actual files, but also the partition inwhich thse files live, the disk the partitions are on, you keyboard, mouse and display, even your printer as well as internal kernel datastructures and data.

 So how does linux talk to all of thse ddifferent devives throught the same interface?

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
