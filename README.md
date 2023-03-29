# command-cache

command-cache is a wrapper around other commands that stores their output for later use, without having to re-execute them

## Usage

The argument format is static
```
$ command-cache {time-limit} {command} {command-arg-1} {command-arg-2} ...
```

The time limit is expressed in milliseconds. If the last time the command was executed (provided it was executed using `command-cache`) was less than `time-limit` milliseconds back, the command will not be re-executed, instead, the cached output will be printed

If the cache needs to be cleared, run
```
$ command-cache --purge
```

## Why?
At the time of writing (29/03/2023) [Waybar](https://github.com/Alexays/Waybar) seems to launch custom modules' commands indipendently per monitor, with only instants between executions. This causes the modules to have different outputs on different monitors, and it also makes the modules whose commands' outputs are based on the time elapsed between executions misbehave

With command-cache, synchronization between monitors can be achieved: the first to launch will execute the command, and the other(s) will use the cached output, instead of re-executing the command

## Notes
+ The cached commands' outputs are stored **without any form of security or encryption**
+ Only stdout is cached and printed
+ The cache files are stored in `/tmp/command-cache`, you should make your own considerations based on how `/tmp` is mounted in your system
