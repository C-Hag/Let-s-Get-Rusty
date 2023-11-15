use pcap::{Capture, Device, Error}; // pcap related imports
use std::env; // for accessing command line arguments
use std::fs; // for file system operations
use std::path::PathBuf; // for handling file paths

// Function to set up directory for saving captures
fn setup_capture_directory() -> std::io::Result<PathBuf> {
    let mut dir = env::current_dir()?;
    dir.push("captured_packets"); // Directory name

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

fn main() -> Result<(), Error> {
    // Parsing command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: <program> <number_of_packets>");
        std::process::exit(1);
    }

    let number_of_packets: usize = args[1].parse().expect("Invalid number for packets");

    // Setting up the directory for saving the capture file
    let capture_dir = setup_capture_directory()?;
    let file_name = "capture.pcap"; // Capture file name

    let mut file_path = PathBuf::from(capture_dir);
    file_path.push(file_name);

    // Capturing packets
    let default_device = Device::lookup()?;
    let mut cap = Capture::from_device(default_device)?
        .promisc(true)
        .snaplen(5000)
        .open()?;
    let mut savefile = cap.savefile(file_path.as_path())?;

    for _ in 0..number_of_packets {
        if let Ok(packet) = cap.next() {
            savefile.write(&packet);
        }
    }

    Ok(())
}
