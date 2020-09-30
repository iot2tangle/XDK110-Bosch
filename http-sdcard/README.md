# XDK2Streams HTTP Protocol WITH SD CARD

HTTP is a common protocol to transfer data and files over the network. The XDK supports HTTP natively and offers two modules to make HTTP requests. This guide will provide an introduction to both of them and will demonstrate how to use them to make GET and POST request.

The following repository has either files for the Bosch XDK 110 and for the data receiver in Rust where the attach to Tangle via Streams happens. 

**This package is a variation of the HTTP one that allows to use WLAN SSID, Password, Host and other needed values from a config file on a micro sd card, which makes possible to use the XDK in diferent networks without need to recompile (you just change values in the config file and you are ready to go)**

- xdk2streams-c (C Code to build and flash to your XDK)
- xdk2mam-streams (Rust code to start a listener server)

# Instructions

## Requirements
In order to be able to run the code on this repo you will to [download XDK Workbench](https://xdk.bosch-connectivity.com/software-downloads), have a XDK 110 and insall Node on the computer you are going to use as listener server.

## Flashing your XDK: wifi and sensors configuration
Open XDK Workbench and go to File -> Import. Choose General > Projects from Folder or Archive and select the folder ***xdk2mam-c***. Accept to import project. 

### Clear, Build and Flash
Open XDK Workbench and go to File -> Import. Choose General > Projects from Folder or Archive and select the folder **xdk2mam-c**. Accept to import project. Once project is imported, right click on **xdk2mam** folder in your Workbench Project Explorer and select **Clean project**. When the clean is done, repat and select **Build Project**. This process can take some minutes depending on your hardware and you should see any problems at the Workbench Console.

Finally, once the project has been built, connect your XDK 110 via USB and click the ***Flash*** button to install the software on the board. If everything went fine, you should be able to see the sensor data on your console.

### Editing config data

Open the **config.cfg** file on your computer and change the values to match your WLAN data, host, port and the sensors you want to use.

```
DEVICE_NAME=enter-your-device-id
WLAN_SSDI=enter-your-wifi-ssid
WLAN_PSK=enter-your-wifi-password
DEST_SERVER_HOST=192.168.0.4
DEST_SERVER_PORT=8080
INTER_REQUEST_INTERVAL=3000
DEST_POST_PATH=/sensors
ENVIROMENTAL=YES
ACCELEROMETER=YES
GYROSCOPE=YES
INERTIAL=YES
LIGHT=YES
MAGNETOMETER=YES
ACOUSTIC=YES
```

Save the values, extract the micro SD card and carefully insert it into the XDK SD slot (contacts up). 
Turn on the XDK and you are good to go! 
If everything went fine the XDK110 should now be sending its sensors data to the given destination server. 


## Setting up Rust Server for Streams

### Streams: Setting up your Rust publisher


The followong repository provides with code in Clang to flash the Bosch XDK110 and in Rust to get the XDK110 datasets and publish them to the IOTA Tangle through Streams. 
This code has been tested on Raspberry PI 3 and 4 and in Debian based Virtual Private Servers.  

## Preparation

Clone this repo and navigate to the http-sdcard/xdk2streams-streams where the Rust code is

`git clone https://github.com/iot2tangle/xdk2streams.git`

Install Rust if you don't have it already, find the instructions here https://www.rust-lang.org/tools/install

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

