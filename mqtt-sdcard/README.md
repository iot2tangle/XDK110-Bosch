# XDK110 MQTT with SD CARD

MQTT is an open OASIS and ISO standard lightweight, publish-subscribe network protocol that transports messages between devices. The protocol usually runs over TCP/IP; however, any network protocol that provides ordered, lossless, bi-directional connections can support MQTT

The following repository has C code to have the Bosch XDK 110 sending its data to a MQTT Broker. 

**This package is a variation of the MQTT one that allows to use WLAN SSID, Password, Broker address, Port, and other needed values from a config file on a micro sd card, which makes possible to use the XDK in diferent networks without need to recompile (you just change values in the config file and you are ready to go)**

# Instructions for the XDK110

## Requirements
In order to be able to run the code on this repo you will to [download XDK Workbench](https://developer.bosch.com/web/xdk/downloads), and have a Bosch XDK 110.

## Flashing your XDK: wifi and sensors configuration

### Clear, Build and Flash
Open XDK Workbench and go to File -> Import. Choose General > Projects from Folder or Archive and select the folder **XDK110-Bosch/mqtt-sdcard**. Accept to import project. Once project is imported, right click on **iot2tangle-mqtt** folder in your Workbench Project Explorer and select **Clean project**. When the clean is done, repeat and select **Build Project**. This process can take some minutes depending on your hardware and you should see any problems at the Workbench Console.

Finally, once the project has been built, connect your XDK 110 via USB and click the ***Flash*** button to install the software on the board. If everything went fine, you should be able to see the sensor data on your console.

### Editing config data

Open the **config.cfg** file on your computer and change the values to match your WLAN data, MQTT Host, Port, auth data, and the sensors you want to use.

```
DEVICE_NAME=enter-your-device-id
WLAN_SSDI=enter-your-wifi-ssid
WLAN_PSK=enter-your-wifi-password
MQTT_BROKER_HOST=enter-ip-mqtt-host
MQTT_BROKER_PORT=enter-port-mqtt
PUBLISHTIMER_PERIOD_IN_MS=30000
MQTT_USERNAME=enter-username
MQTT_PASSWORD=enter-password
TOPIC=enter-topic-name
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
If everything went fine the XDK110 should now be sending its sensors data to the MQTT Broker defined in the config. 

### Dealing with the Invalid application error
Some XDK110 using the 1.1.0 bootloader version produce an invalid application output when the flash process finishes. If you get this error, try clicking on the **Boot** button (it should reboot and give the error again) and then click again on **Flash**. If you get the error again, repeat the process. We don't know why this error is produced and have already informed the XDK110 team at Bosch about it.


# Setting up the Streams Gateway

**Note:** you can run the Gateway on a Raspberry Pi, a local Node in your Network or a VPS

## Installing the MQTT broker

The Gateway will require a MQTT Broker to get the data sent by the XDK110. You can run one locally with Mosquitto:  

`sudo apt install mosquitto`  

If you don't want to set authentication for users run:  

`mosquitto`  


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

`git clone https://github.com/iot2tangle/Streams-mqtt-gateway.git`

Navigate to the **Streams-mqtt-gateway** directory and edit the **config.json** file to define your device name (it must match what you set on the Sense Hat config).
There you can also set the Broker, Ports, Credentials, Topic, and the IOTA Full Node used.  
  
```
{
    "whitelisted_device_ids": [
        "enter-your-device-id",
        "DEVICE_ID_2"
    ],
    "username": "",
    "password": "",
    "broker_ip": "localhost",
    "broker_port": 1883,
    "topic": "iot2tangle",
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

![Streams Gateway receiving XDK110 data](https://iot2tangle.io/assets/screenshots/XDK110MQTTGW.png)

*The Gateway starts by giving us the channel id that will allow subscribers to access the channel data.*

### Reading messages from the Tangle

You can now head to the I2T Streams Explorer and search for the published datasets using the channel Id provided by the Gateway

![Streams Gateway receiving SenseHat data](https://iot2tangle.io/assets/screenshots/XDKMQTTEXPLORER.png)


IMPORTANT: The device will be authenticated through the "device" field in the request (in this case XDK-HTTP), this has to match what was set as device_name in the config.json on the Gateway (see Configuration section above)!  
  
After a few seconds you should now see the data beeing recieved by the Subscriber!
