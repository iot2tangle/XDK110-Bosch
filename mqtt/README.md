# XDK110 MQTT

MQTT is an open OASIS and ISO standard lightweight, publish-subscribe network protocol that transports messages between devices. The protocol usually runs over TCP/IP; however, any network protocol that provides ordered, lossless, bi-directional connections can support MQTT

The following repository has C code to have the Bosch XDK 110 sending its data to a MQTT Broker. 

# Instructions for the XDK110

## Requirements
In order to be able to run the code on this repo you will to [download XDK Workbench](https://developer.bosch.com/web/xdk/downloads), and have a Bosch XDK 110.

## Flashing your XDK: Wifi, Broker and sensors configuration

Open XDK Workbench and go to File -> Import. Choose General > Projects from Folder or Archive and select the folder **XDK110-Bosch/mqtt-sdcard**. Accept to import project.
Navigate to the source folder and edit the following lines at **iot2tangle-mqtt.h**


```
#define WIFI_SSID 			   "YourWifiNetwork"
```

```
#define WIFI_PW				      "YourWifiPassword"
```

```
#define MQTT_BROKER_HOST	  "MQTT_IP_BROKER_HOST"
```

```
#define MQTT_BROKER_PORT	UINT16_C(11075) //<-- MQTT_PORT_BROKER
```

```
#define MQTT_USERNAME	      "YourMqttUsername"
```

```
#define MQTT_PASSWORD	       "YourMqttPassowrd"
```

```
#define DEVICE_NAME			"XDK-DEVICE-NAME"	
```
```
#define TOPIC				"YourTopicName"
```

```
#define PUBLISHTIMER_PERIOD_IN_MS 30000
```

By default this code will stream data for every sensor built in in the XDK 110. If you want to only use some sensors, edit the file ***iot2tangle-mqtt.c*** and switch to false on the ones you don't want to use. For instance, following edit will not send data for  Light and Magnometer sensors

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
Open XDK Workbench and go to File -> Import. Choose General > Projects from Folder or Archive and select the folder **XDK110-Bosch/mqtt**. Accept to import project. Once project is imported, right click on **iot2tangle-mqtt** folder in your Workbench Project Explorer and select **Clean project**. When the clean is done, repeat and select **Build Project**. This process can take some minutes depending on your hardware and you should see any problems at the Workbench Console.

Finally, once the project has been built, connect your XDK 110 via USB and click the ***Flash*** button to install the software on the board. If everything went fine, you should be able to see the sensor data on your console.

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


**IMPORTANT:** Devices will be authenticated by the Gateway through the "whitelisted_device_ids" parameter of the config.json. Be sure to use the same Device Id on both config files (the one used for the XDK110 SD Card and the one used by the Gateway).
  
