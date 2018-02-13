+++
title = "Another Bash Strict Mode"
description = "Cause bash scripts to fail fast and loud to aid debugging."
slug = "another-bash-strict-mode"
date = 2016-04-14
tags = ["linux", "bash", "shell"]
aliases = [
    "/another-bash-strict-mode/",
    "/posts/another-bash-strict-mode/",
]
+++

I have been using [Aaron Maxwell's Unofficial Bash Strict
Mode](http://redsymbol.net/articles/unofficial-bash-strict-mode/) for many years
now and it has saved my loads of time if finding and fixing buggy bash scripts.
The main problem I now encounter is scripts that fail silently (or far from the
last command that output anything). To solve this I have started to use the
following variant.

```sh
#!/bin/bash
set -uo pipefail
trap 's=$?; echo "$0: Error on line "$LINENO": $BASH_COMMAND"; exit $s' ERR
IFS=$'\n\t'
```

The major problem with `set -e` and `set -o pipefail` is that they are silent so
you have to rely the output of the failed command to debug your script. But not
all commands fail loudly and when they do they don't tell you where in your
script they failed.

Error traps can give you more information, anything that is available to the
bash shell at the time they where triggered. Most usefully the command that
failed `$BASH_COMMAND` and the line number `$LINENO` that command was on.
