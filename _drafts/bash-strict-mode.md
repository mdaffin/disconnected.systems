---
layout: post
title: Unofficial Bash Strict Mode
description: Cause bash scripts to fail fast and fail loudly to aid debugging.
tags: [linux, bash, shell]
---

I have been using [Aaron Maxwell's Unofficial Bash Strict
Mode](http://redsymbol.net/articles/unofficial-bash-strict-mode/) for many years
now and it has saved my loads of time if finding and fixing buggy bash scripts.
My main problem now is scripts that fail silently (or far from the last command
that output anything). To solve this I have started to use the following
variant.

{% highlight shell %}
#!/bin/bash
set -uo pipefail
trap 's=$?; echo "$0: Error on line "$LINENO": $BASH_COMMAND"; exit $s' ERR
IFS=$'\n\t'
{% endhighlight %}

<!-- more -->

The major improvement over Aaron Maxwell's version is that scripts can not fail
silently. Instead they will tell you where and what failed, although not always
why. The problem with `set -e` is it produces no output forcing you to entirely
rely on the failing command to print what went wrong. This has two problems,
first not all commands are nice enough to print why they failed and second
commands won't tell you where your script failed.

Error traps can give you more information - anything that is available to the
bash shell at the time they where triggered.

This is why I replaced the `set -e` with and error trap. Now I can print out any
information that could be useful to identifying the failing command, most
notably the line number and the command that was run. With this information you
do not need to guess at or worst plaster your script in debug echo statements to
figure out which command failed. It just tell you.

Aaron Maxwell's post also contains allot of useful tips on working in a more
strict bash. I am not going to cover what he has already said but am going to
expand on it with some useful tricks I have found over the years.
