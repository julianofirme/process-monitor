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
cpu0 => [18.0%]       cpu1 => [11.1%]       cpu2 => [10.1%]   
cpu3 => [9.3%]        cpu4 => [9.6%]        cpu5 => [12.1%]   
cpu6 => [10.2%]       cpu7 => [10.4%]       cpu8 => [8.2%]    
cpu9 => [7.7%]        cpu10 => [8.7%]       cpu11 => [9.1%]   
```
