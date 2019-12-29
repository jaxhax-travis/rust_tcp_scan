# rust_tcp_scan
A simple multi-threaded TCP port scanner written in Rust. This was written mostly as an exercise for me to learn a little bit about threading in Rust but I am releasing it in hopes that it is useful for someone else looking for a code example. This code will create a thread for each port to be scanned.

# Getting started

## Installation

### Build From Source
The application can be built using `cargo build --release`. The resulting binary will be found in target/release/. This has been tested in Linux and Windows.

# Screenshots
![Help Screen](https://gist.githubusercontent.com/jaxhax-travis/2add94577250c42bdcb6093a8df6f09d/raw/af4d24ba1e8535db0cd71c5da0a1afcaebcf2cce/rust_tcp_scan_help.png?raw=true "tcp_scan Help Screen")
tcp_scan help screen


![Scanning Select Ports on Metasploitable2](https://gist.githubusercontent.com/jaxhax-travis/2add94577250c42bdcb6093a8df6f09d/raw/af4d24ba1e8535db0cd71c5da0a1afcaebcf2cce/rust_tcp_scan_metasploitable2_select_ports.png?raw=true "Scanning Select Ports on Metasploitable2")
Scanning ports 80, 443, and 3000-4000 on Metaploitable2 VM.


![Scanning All Ports on Metasploitable2](https://gist.githubusercontent.com/jaxhax-travis/2add94577250c42bdcb6093a8df6f09d/raw/af4d24ba1e8535db0cd71c5da0a1afcaebcf2cce/rust_tcp_scan_metasploitable2_all_ports.png?raw=true "Scanning All Ports on Metasploitable2")
Scanning all ports on Metaploitable2 VM.
