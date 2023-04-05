# Process Monitor (in progress)

This is a simple process monitor that displays information about the system's CPU and RAM usage for each process. It uses the **sysinfo** to get system information and the **termion** crate to format the output.

## TODO
- [ ] Implement RAM usage
- [ ] Format the output to a beautiful table

## Usage
To run the process monitor, simply run the following command:

```
cargo run
```
The monitor will refresh the CPU usage every 2 seconds and display the results in the console.

## Example output (for now)
Here's an example output of the process monitor:

```
cpu0 => [5.3%]        cpu1 => [4.3%]        cpu2 => [2.0%]    
cpu3 => [1.0%]        cpu4 => [7.7%]        cpu5 => [5.4%]    
cpu6 => [1.0%]        cpu7 => [1.5%]        cpu8 => [3.4%]    
cpu9 => [2.0%]        cpu10 => [2.0%]       cpu11 => [2.5%]    
ram => [6145mb/16124mb] 
```
