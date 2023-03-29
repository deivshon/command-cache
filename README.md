# command-cache

command-cache is a wrapper around other commands that stores their results for later use, without having to re-execute them

## Usage

The argument format is static
```
$ command-cache {time-limit} {command} {command arguments}
```

The time limit is expressed in milliseconds. If the last time the command was executed was less than `time-limit` milliseconds back, the command will not be re-executed, instead, the cached result will be printed

## Why?

At the time of writing (29/03/2023) Waybar seems to launch custom modules' commands indipendently per monitor. This causes the modules to have different outputs on different monitors, and it also makes the modules whose commands' outputs are based on the time elapsed between executions misbehave

With command-cache, synchronization between monitors can be achieved: the first to launch will execute the command, and the other(s) will use the cached result, instead of re-executing the command
