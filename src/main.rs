use sysinfo::{System, SystemExt, CpuExt};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use termion::clear;
use termion::cursor;

fn main() {
    let mut sys = System::new_all();
    
    // Initialize termion configs
    let stdout = stdout();
    let mut stdout = stdout.lock();

    // Variable to limit the number of nucle per column
    let mut count = 0;
    
    // Clean terminal before printing
    write!(stdout, "{}", clear::All).unwrap();
    
    loop {
        // Update data
        sys.refresh_all();

        // Delay to update
        thread::sleep(Duration::from_millis(2000));
        
        // Variebles to show the nucle usage in column format
        let column_width = 8;
        let spacing = 4;

        // Print information on top of terminal
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

        // Show CPU usage per nucle
        for (i, cpu) in sys.cpus().iter().enumerate() {
            if count % 3 == 0 {
                writeln!(stdout).unwrap();
            }
            
            // Show cpu usage
            let cpu_usage = format!("{:.1}%", cpu.cpu_usage());
            let num_spaces = column_width - cpu_usage.len();

            write!(
                stdout,
                "cpu{} => [{}]{}",
                i,
                cpu_usage,
                " ".repeat(num_spaces)
            )
            .unwrap();
            
            write!(stdout, "{}", cursor::Right(spacing as u16)).unwrap();

            count += 1;
        }

        writeln!(stdout).unwrap();

        // Show ram usage
        let ram_usage = format!("{}mb", sys.used_memory() / 1000000);
        let ram_total = format!("{}mb", sys.total_memory() / 1000000);

        write!(
            stdout,
            "ram => [{}/{}]",
            ram_usage,
            ram_total
        )
        .unwrap();


        stdout.flush().unwrap();
    }
}
