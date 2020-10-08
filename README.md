# Bosch XDK 110

The [Bosch XDK110](https://xdk.bosch-connectivity.com) is a programmable sensor device & a prototyping platform for many IoT use cases, being used in the field of Internet of Production, mainly to measure the performance of industrial machinery.

On the other side, [IOTA Streams](https://blog.iota.org/iota-streams-alpha-7e91ee326ac0) is a second layer cryptographic framework to securely distribute data over the Tangle.

[IOT2TANGLE](https://iot2tangle.io) main goal is to provide open source software to allow interaction between this powerful hardware and the IOTA Tangle.

The following repository has the needed **C code** for the Bosch XDK110 to send its sensors data to the Streams Gateway

## This branch code has been tested in Workbench 3.6.1

While our goal is to always be up to the latest release of the [XDK Workbench](https://xdk.bosch-connectivity.com/software-downloads) (an Eclipse based IDE that comes with XDK to build software and flash it to the hardware), changes made by the Bosch team from release to release tend to leave our code with some compilation errors. 

This is an issue **we are reviewing actively with Bosch XDK team**. Until we sync we recommend to use our so called **CSVC** (current stable version to compile). This will allow you to build your project without errors so you can start working with your XDK inmmediatly after.

## Available connectivity

- **[HTTP](https://github.com/iot2tangle/XDK110-Bosch/tree/dev/http)** (WiFi on XDK will just post a request with the data to a given server)
- **[HTTP-SD Card](https://github.com/iot2tangle/XDK110-Bosch/tree/master/http-sdcard)** (same as HTTP but with a config file placed on the microSD card to allow portability)

