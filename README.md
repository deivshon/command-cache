# command-cache

command-cache is a wrapper around other commands that stores their output for later use, without having to re-execute them

## Usage
The following command line arguments are available

| Argument | Long argument | Description                                                                              | Optional | Default            |
|----------|---------------|------------------------------------------------------------------------------------------|----------|--------------------|
| -c       | --command     | Command to be run                                                                        | No       | N/A                |
| -p       | --period      | Threshold beyond which the command needs to be re-run instead of using the cached output | No       | N/A                |
| -d       | --dir-cache   | Directory where to look for and store the cache file for the current run                 | Yes      | /tmp/command-cache |

The period is to be specified in milliseconds
## Why?
At the time of writing (29/03/2023) [Waybar](https://github.com/Alexays/Waybar) seems to launch custom modules' commands indipendently per monitor, with only instants between executions. This causes the modules to have different outputs on different monitors, and it also makes the modules whose commands' outputs are based on the time elapsed between executions misbehave

With command-cache, synchronization between monitors can be achieved: the first to launch will execute the command, and the other(s) will use the cached output, instead of re-executing the command

## Notes
+ The cached commands' outputs are stored **without any form of security or encryption**
+ Only stdout is cached and printed
+ By default cache files are stored in `/tmp/command-cache`, you should make your own considerations based on how `/tmp` is mounted in your system
