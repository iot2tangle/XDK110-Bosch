# XDK2MAM

## Preparation
Install rust if you don't have it already, find the instructions here https://www.rust-lang.org/tools/install

Make sure you also have the build dependencies installed, if not run:  
`sudo apt install build-essential`  
`sudo apt install pkg-config`  
`sudo apt install libssl-dev`  
`sudo apt update`  

## Installing XDK2Streams
Download XDK2Streams:  
`git clone https://github.com/AleBuser/xdk2mam`  
`cd xdk2mam/http-streams`  
  
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

To send data to the server you can use Postman, or like in this case cURL, make sure the port is the same as in the config.json file:  
`  
curl --location --request POST '127.0.0.1:8080/sensor_data_public'   
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
    "device": "EXAMPLE_KEY",  
    "timestamp": "1558511111"  
}'  
`  

Same thing for masked data but with the _sensor_data_masked_ endpoint (Note, this only works if a subscriber has subscribed to the encrypted channel already):  
`curl --location --request POST '127.0.0.1:8080/sensor_data_masked'
--header 'Content-Type: application/json' 
--data-raw '{
    "xdk2mam": [
        {
            "sensor": "Magnetometer",
            "data": [
                {
                    "x": "36"
                },
                {
                    "y": "51"
                },
                {
                    "z": "28"
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
    "device": "EXAMPLE_KEY",
    "timestamp": "1558511111"
}'  
`  

After a few seconds you should now see the data beeing recieved by the Subscriber!
