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

- Different protocals
  - IDE
  - SCSI
  - NVME
- Storage is broken up into block which are read/written as single chunks
- Can be treated as a single contiuous block of storage but there are some
  preformance considerations which affects the designs

## Partition Tables

Data on a disk is just 1s and 0s to the hardware, it enforces no meaning of
these bits and bytes. Partition tables are the first way give these bits some
meaning. In short it has two main responsibilities, to define a location where
the boot hardware and firmware (aka, BIOS/UEFI) to look for code to start
executing on powerup. And to split up the disk into smaller chunks that can be
used in different ways, known as partitions.

There are two main partition tables, the older MSDOS and the newer GPT. Both
are widly used these days but for newer devices there is a prefence towards
GPT.

### MSDOS

The legacy partitoin table, first created in ???? and still widly used today.
All devices understand this format making it a lot more portable than GPT but
it also has some limitations due to its aging design.

The data that desctibes this format is all stored at the start of the disk,
known as the header. The rest of the disk is untouched.

One of the biggest limitation of MSDOS is the restriction of 4 partitions, we
all know these are the primary partitions. This is due to lack for forsite when
the table was originally designed likely in an attempt to reduce the space used
by the header (you know, when disks where measured in KB and MB). This short
comming was relised too late and the format was in wide use by different
systems so could not be changed. Instead the idea of logical or extended
partitions was added and with this you could sub divide one of the primary
partitions into smaller segments known as logical partitions (numbered from 5).

The first 512 bytes??? of the disk are reserved for the master boot record
(MBR). This is the code that starts executing when the BIOS/UEFI firmware wants
to boot a disk. This contains the boot loader, or part of it, and is
responsible for understanding the contents of the partitions and figuring out
where on the disk to continue executing (either a second stage of the
bootloader or a kernel - but could be any code).

### GNU??? Parition Table (GPT)

The newish and well, only other kid on the block. Designed around the same time
as UEFI it is designed to fix some of the shortcomming with the MSDOS format.
Most notabilly it no longer has the four partition limit (increased to
128?????) and so the concept of extended and logical partitions does not apply
to it.

Devices that boot UEFI are also expected to have a greater understanding of the
format as well as a basic filesystem like vfat/exfat????? and as such does not
include a single location to start the boot from. Instead the firmware looks
for the partition marked as the EFI Partition and then searches this for EFI
applications (which can be bootloaders or it can directly boot kernels designed
for this).

Similarly to msdos the data for GPT is stored at the start of the disk, but
unlike MSDOS a copy is also stored at the end???? of the disk.

### Parition Alignment

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
