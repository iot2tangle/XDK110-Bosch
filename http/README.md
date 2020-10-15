# XDK110 with WiFI

HTTP is a common protocol to transfer data and files over the network. The XDK supports HTTP natively and offers two modules to make HTTP requests. This guide will provide an introduction to both of them and will demonstrate how to use them to make GET and POST request.

# Instructions

## Requirements
In order to be able to run the code on this repo you will to [download XDK Workbench](https://developer.bosch.com/web/xdk/downloads), and have a XDK 110.

## Flashing your XDK: wifi and sensors configuration

Open XDK Workbench and go to File -> Import. Choose General > Projects from Folder or Archive and select the ***XDK110-Bosch/http*** folder. Accept to import project. Navigate to the source folder and edit the following lines at ***iot2tangle.h***


```
#define DEVICE_NAME      "XDK-HTTP"
```

```
#define WLAN_SSID        "enter-your-wifi-ssid"
```

```
#define WLAN_PSK         "enter-your-wifi-password"
```
```
#define DEST_SERVER_HOST         "192.168.7.181"
```

```
#define INTER_REQUEST_INTERVAL   UINT32_C(30000)
```
By default this code will stream data for every sensor built in in the XDK 110. If you want to only use some sensors, edit the file ***iot2tangle.c*** and switch to false on the ones you don't want to use. For instance, following edit will not send data for  Light and Magnometer sensors

```
// Global array of all sensors => true : enable -- false : disable
bool typesSensors[6] = {
						true, //ENVIROMENTAL
						true, //ACCELEROMETER
						true, //GYROSCOPE
						true, //INERTIAL
						false, //LIGHT
						false  //MAGNETOMETER
					};
```

### Clear, Build and Flash
Once changes are to this files are saved, right click on ***XDK110-Bosch*** folder in your Workbench Project Explorer and select ***Clean project***. Once this is done, repat and select ***Build Project***. This process can take some minutes depending on your hardware and you should see any problems at the Workbench Console.

Finally, once the project has been built, connect your XDK110 via USB and click the ***Flash*** button to install the software on the board. If everything went fine the XDK110 should now be sending its sensors data to the given destination server. 

### Dealing with the Invalid application error
Some XDK110 using the 1.1.0 bootloader version produce an invalid application output when the flash process finishes. If you get this case, try clicking on the **Boot** button (it should reboot and give the error again) and then click again on **Flash**. If you get the error again, repeat the process. We don't know why this error is produced and have already informed the XDK110 team at Bosch about it.


# Setting up the Streams Gateway

**Note:** you can run the Gateway on a Raspberry Pi, a local Node in your Network or a VPS


## Preparation

Install Rust if you don't have it already, find the instructions here https://www.rust-lang.org/tools/install

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Make sure you also have the build dependencies installed, if not run:  

`sudo apt install build-essential`  
`sudo apt install pkg-config`  
`sudo apt install libssl-dev`  
`sudo apt update`  

## Installing the IOTA Stream Gateway

Get the IOTA Streams WiFi Gateway. 

`git clone https://github.com/iot2tangle/Streams-http-gateway`

Navigate to the **Streams-wifi-gateway** directory and edit the **config.json** file to define your device name (it must match what you set on the Sense Hat config).
There you can also change ports and the IOTA Full Node used.  
  
```
{
    "device_name": "XDK-HTTP", 
    "port": 8080, 
    "node": "https://nodes.iota.cafe:443", 
    "mwm": 14,    
    "local_pow": false     
}
```

**Set the *device_name* to the value specified in the XDK110 configuration file as DEVICE_NAME**  
Change *port, node, mwm, local_pow* if needed. 

## Start the Streams Server

### Sending messages to the Tangle

Run the Streams Gateway:  

`cargo run --release`  

This will compile and start the Streams Gateway. Note that the compilation process may take from 3 to 30 minutes (Pi3 took us around 30 mins, Pi4 8 mins and VPS or desktop machines will generally compile under the 5 mins) depending on the device you are using as host.

You will only go through the compilation once and any restart done later will take a few seconds to have the Gateway working.

![Streams Gateway receiving SenseHat data](https://iot2tangle.io/assets/screenshots/PiSenseHatSend.png)
*The Gateway starts by giving us the channel id that will allow subscribers to access the channel data.*

### Reading messages from the Tangle

In a separate console start a subscriber using the Channel Id printed by the Gateway (see example above):  

`cargo run --release --example subscriber <your_channel_root> `  

![Streams Gateway receiving SenseHat data](https://iot2tangle.io/assets/screenshots/PiSenseHatGet.png)


### Testing the Gateway without sensors

To send data to the server you can use Postman, or like in this case cURL, make sure the port is the same as in the config.json file:  
`  
curl --location --request POST '127.0.0.1:8080/sensor_data'   
--header 'Content-Type: application/json'   
--data-raw '{
    "iot2tangle": [
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
IMPORTANT: The device will be authenticated through the "device" field in the request (in this case XDK-HTTP), this has to match what was set as device_name in the config.json on the Gateway (see Configuration section above)!  
  
After a few seconds you should now see the data beeing recieved by the Subscriber!


