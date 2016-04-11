---
layout: post
title: Unofficial Bash Strict Mode
description: Cause bash scripts to fail fast and fail loudly to aid debugging.
tags: [linux, bash, shell]
---

I have been using [Aaron Maxwell's Unofficial Bash Strict Mode](http://redsymbol.net/articles/unofficial-bash-strict-mode/) for many years now and it has saved my loads of time if finding and fixing buggy bash scripts.

{% highlight shell %}
#!/bin/bash
set -uo pipefail
trap 's=$?; echo "${0}: Error on line "${LINENO}": $BASH_COMMAND"; exit $s' ERR
IFS=$'\n\t'
{% endhighlight %}

The major improvement over Aaron Maxwell's version is that script now fail loudly. The problem with `set -e` is it produces no output but relies on the failing command to print what went wrong. This has two problems, first not all commands are nice enough to print why they failed and second they won't tell you where your script failed. This means when a command fails you have to figure ypou where in your script it was and quite often which command actually caused the problem.

This is why I replaced the `set -e` with and error trap. Now I can print out any information that could be useful to identifying the failing command, most notably the line number and the command that was run. With this information you do not need to guess at or worst plaster your script in debug echo statements to figure out which command failed. It just tell you.

Aaron Maxwell's post also contains allot of useful tips on working in a more strict bash. I am not going to cover what he has already said but am going to expand on it with some useful tricks I have found over the years.

## Handling arguments

With `set -u` you get a nice little error message when you try to use an undefined variable.

{% highlight shell %}
./s: line 8: $1: unbound variable
{% endhighlight %}

Which is great to catch variables you have forgotten to define, but when checking script arguments it gives a rather unhelpful message to your user. 

{% highlight shell %}
./s: line 8: 1: Missing ARG1: Usage: ./s <ARG1> [ARG2] [ARG3]
{% endhighlight %}

{% highlight shell %}
#!/bin/bash
set -uo pipefail
trap 's=$?; echo "${0}: line "${LINENO}": Error running $BASH_COMMAND"; exit $s' ERR
IFS=$'\n\t'

usage="Usage: $0 <ARG1> [ARG2] [ARG3]"

arg1=${1?"Missing ARG1: $usage"}
arg2=${2-b}
arg3=${3-}

echo "'${arg1}' '${arg2}' '${arg3}'"
{% endhighlight %}
