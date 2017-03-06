+++
date = "2017-03-05T23:00:35Z"
title = "2017 03 05 automating custom pi images"
draft = true
description = "Takes creating custom pi images on step further and completely automates the process using docker github and travis."
slug = "automating-pi-image-creation"
tags = ["linux", "automation", "raspberry-pi", "github", "travis"]
aliases = [
    "/automating-pi-image-creation/",
    "/posts/automating-pi-image-creatio/",
]
+++

```shell
sudo docker run --privileged --rm -it -v $PWD:/code -w /code ubuntu bash -c "apt update -y && \
apt install -y qemu qemu-user-static binfmt-support parted wget dosfstools && \
./create-image" 
```
```shell
-rw-r--r-- 1 root    root    2.0G Mar  5 23:36 rpizw-rover.img
-rw-r--r-- 1 mdaffin mdaffin 303M Mar  5 23:38 rpizw-rover.img.bz2
-rw-r--r-- 1 mdaffin mdaffin 330M Mar  5 23:37 rpizw-rover.img.gz
-rw-r--r-- 1 mdaffin mdaffin  94M Mar  5 23:38 rpizw-rover.img.xz
```