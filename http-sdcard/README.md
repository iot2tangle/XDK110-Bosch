# XDK2MAM

The followong repository provides with code in Clang to flash the Bosch XDK110 and in Rust to get the XDK110 datasets and publish them to the IOTA Tangle through Streams. 
This code has been tested on Raspberry PI 3 and 4 and in Debian based Virtual Private Servers.  

## Preparation
Install rust if you don't have it already, find the instructions here https://www.rust-lang.org/tools/install

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Make sure you also have the build dependencies installed, if not run:  

`sudo apt install build-essential`  
`sudo apt install pkg-config`  
`sudo apt install libssl-dev`  
`sudo apt update`  

## Installing XDK2Streams
Download XDK2Streams:  
`git clone https://github.com/iot2tangle/xdk2streams`  
`cd http-sdcard/xdk2streams-streams`  
  
Configure the Streams Gateway:  
`nano config.json`  

Run the Streams Gateway:  
`cargo run --release [device_key] [subscriber_key]`  
This starts the server which will forward messages from the XDK to the Tangle as well as manage the subscribers

  
## Runnig the Examples:  
  
Start the Streams Gateway with two example keys:  
`cargo run --release EXAMPLE_KEY SUB_KEY`  

In a separate window start a public-only subscriber:  
`cargo run --release --example subscriber_public_only`  

Or a subscriber that can read both public and masked data:  
`cargo run --release --example subscriber`  

