use pcap::{Capture, Device, Error};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn setup_capture_directory() -> std::io::Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    dir.push("captured_packets");
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

fn print_intro() {
    println!(
    // ... ASCII art ...
"
████   ██  ███████╗██╗    ██╗      ██████╗  ██████╗ █████╗ ██████╗ ████████╗██╗   ██╗██████╗ ███████╗
████╗  ██║╚══██╔══╝██║    ██║      ██╔══██╗██╔════╝██╔══██╗██╔══██╗╚══██╔══╝██║   ██║██╔══██╗██╔════╝
██╔██╗ ██║   ██║   ██║ █╗ ██║█████╗██████╔╝██║     ███████║██████╔╝   ██║   ██║   ██║██████╔╝█████╗  
██║╚██╗██║   ██║   ██║███╗██║╚════╝██╔═══╝ ██║     ██╔══██║██╔═══╝    ██║   ██║   ██║██╔══██╗██╔══╝  
██║ ╚████║   ██║   ╚███╔███╔╝      ██║     ╚██████╗██║  ██║██║        ██║   ╚██████╔╝██║  ██║███████╗
╚═╝  ╚═══╝   ╚═╝    ╚══╝╚══╝       ╚═╝      ╚═════╝╚═╝  ╚═╝╚═╝        ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚══════╝
"

    );
                                                                                                     
}

fn ask_for_packet_count() -> usize {
    let mut input = String::new();
    print!("Enter the number of packets to capture: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().expect("Please enter a valid number")
}

fn choose_network_interface() -> Result<Device, pcap::Error> {
    let devices = Device::list()?;
    println!("Available network interfaces:");
    for (index, device) in devices.iter().enumerate() {
        println!("{}: {} ({})", index + 1, device.name, device.desc.as_deref().unwrap_or("No description"));
    }

    let mut choice = String::new();
    println!("Enter the number of the interface to capture:");
    io::stdin().read_line(&mut choice).unwrap();
    let choice: usize = choice.trim().parse().expect("Invalid input. Please enter a number.");

    if choice == 0 || choice > devices.len() {
        eprintln!("Invalid choice. Please enter a number between 1 and {}.", devices.len());
        std::process::exit(1);
    }

    Ok(devices[choice - 1].clone())
}

fn ask_to_open_in_wireshark() -> bool {
    let mut input = String::new();
    println!("Would you like to open the capture in Wireshark? [y/N]");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().eq_ignore_ascii_case("y")
}

fn main() -> Result<(), Error> {
    print_intro();

    let number_of_packets = ask_for_packet_count();
    let chosen_device = choose_network_interface()?;
    let capture_dir = setup_capture_directory()?;
    let file_name = "capture.pcap";
    let mut file_path = PathBuf::from(capture_dir);
    file_path.push(file_name);

    let mut cap = Capture::from_device(chosen_device)?
        .promisc(true)
        .snaplen(5000)
        .open()?;
    let mut savefile = cap.savefile(file_path.as_path())?;

    let progress_bar = ProgressBar::new(number_of_packets as u64);
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap_or_else(|e: indicatif::style::TemplateError| {
            eprintln!("Error setting progress bar template: {}", e);
            std::process::exit(1);
        });

    progress_bar.set_style(style.progress_chars("#>-"));

    for _ in 0..number_of_packets {
        if let Ok(packet) = cap.next() {
            savefile.write(&packet);
            progress_bar.inc(1);
        }
    }

    progress_bar.finish_with_message("Capture complete");

    if ask_to_open_in_wireshark() {
        if let Err(e) = Command::new("wireshark")
            .arg(file_path.to_str().unwrap())
            .spawn() 
        {
            eprintln!("Failed to open Wireshark: {}", e);
        }
    }

    Ok(())
}
