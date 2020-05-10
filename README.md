# Bully algorithm impementation

## Description
Bully algorithm CLI programm for Task3 (exercise 6) of Distributed Systems.

## Program run
`
target/release/bully_alg -- <path to process file> 
`

After execution the code above, it displays interactive console running continiously. 

## Program commands
Commands allowed in interactive console mode:

* `help` - prints all possible commands
* `list` - returns list of alive processes in the system, coordinator is marked with '*'
* `kill <id>` - kills process with <id> id, <id> is an integer value
* `reload` - reloads initial file, initiates new election 
* `exit` - exits from commnd mode
