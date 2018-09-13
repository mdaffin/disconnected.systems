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

Unlike Windows, Linux (and Unix systems overall) use a single directory structure which starts at the root aka `/`. Everything is located somewhere inside this directory tree.

::: note:::
This is distint from `/root` (aka "slash root") which is the root users home directoy inside the root of the directory structure.
:::

The hole structure is virtual, it does not exist in its entireity anywhere. Instead it is backed by multiple different locations, partitions on hard drives, in memory data structures, on remote macheines accessed over the network etc.

However, typically on most systems the majority of the tree is backed by a single partition on a single disk The kernel makes no aumption of this and you

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
