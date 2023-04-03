use sysinfo::{System, SystemExt, CpuExt};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use termion::clear;
use termion::cursor;

fn main() {
    let mut sys = System::new_all();
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let mut count = 0;
    
    write!(stdout, "{}", clear::All).unwrap();
    
    loop {
        sys.refresh_cpu();
        let column_width = 8;
        let spacing = 4;

        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

        for (i, cpu) in sys.cpus().iter().enumerate() {
            if count % 3 == 0 {
                writeln!(stdout).unwrap();
            }

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

        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(2000));
    }
}
