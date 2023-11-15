use std::env;

fn main() {
    println!("cargo:rustc-link-search=native=C:\\Program Files\\Npcap\\"); // Your path to the "wpcap.lib" file
}
