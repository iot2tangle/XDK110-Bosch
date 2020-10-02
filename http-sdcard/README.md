# XDK2Streams HTTP Protocol WITH SD CARD

HTTP is a common protocol to transfer data and files over the network. The XDK supports HTTP natively and offers two modules to make HTTP requests. This guide will provide an introduction to both of them and will demonstrate how to use them to make GET and POST request.

The following repository has either files for the Bosch XDK 110 and for the data receiver in Rust where the attach to Tangle via Streams happens. 

**This package is a variation of the HTTP one that allows to use WLAN SSID, Password, Host and other needed values from a config file on a micro sd card, which makes possible to use the XDK in diferent networks without need to recompile (you just change values in the config file and you are ready to go)**

- xdk2streams-c (C Code to build and flash to your XDK)
- xdk2mam-rust (Rust code to start a listener server)

# Instructions for the XDK110

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
DEST_POST_PATH=/sensor_data
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


# Instructions for the Streams Gateway


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

Download XDK2Streams and navigate to the xdk2streams-rust folder:  

`git clone https://github.com/iot2tangle/xdk2streams`  
`cd http-sdcard/xdk2streams-rust`  
  
Configure the Streams Gateway on the ***config.json*** file   

```
{
    "device_name": "XDK_HTTP", 
    "port": 8080, 
    "node": "https://nodes.iota.cafe:443", 
    "mwm": 14,    
    "local_pow": false     
}
```

**Set the *device_name* to the value specified in the XDK110 configuration file as DEVICE_NAME**  
Change *port, node, mwm, local_pow* if needed. 

## Runnig the Examples:  

### Sending messages to the Tangle

Run the Streams Gateway:  

`cargo run --release`  

This starts the server which will forward messages from the XDK to the Tangle  
  
The Output will be something like this:  
`>> Starting.... `  
`>> Channel root: "ab3de895ec41c88bd917e8a47d54f76d52794d61ff4c4eb3569c31f619ee623d0000000000000000"`  
`>> To Start the Subscriber run: `  
  
`>> cargo run --release --example subscriber "ab3de895ec41c88bd917e8a47d54f76d52794d61ff4c4eb3569c31f619ee623d0000000000000000" `  
  
`>> Listening on http://0.0.0.0:8080`  

### Reading messages from the Tangle

In a separate window start a subscriber using the Channle Root printed by the Gateway (see example above):  
`cargo run --release --example subscriber <your_channel_root> `  


### Testing 

To send data to the server you can use Postman, or like in this case cURL, make sure the port is the same as in the config.json file:  
`  
curl --location --request POST '127.0.0.1:8080/sensor_data'   
--header 'Content-Type: application/json'   
--data-raw '{
    "xdk2mam": [
        {
            "sensor": "Gyroscope",
            "data": [
                {
                    "x": "4514"
                },
                {
                    "y": "244"
                },
                {
                    "z": "-1830"
                }
            ]
        },
        {
            "sensor": "Acoustic",
            "data": [
                {
                    "mp": "1"
                }
            ]
        }
    ],  
    "device": "XDK_HTTP",  
    "timestamp": "1558511111"  
}'  
`   
IMPORTANT: The device will be authenticated through the "device" field in the request (in this case XDK_HTTP), this has to match what was set as device_name in the config.json on the Gateway (see Configuration section above)!  
  
After a few seconds you should now see the data beeing recieved by the Subscriber!
