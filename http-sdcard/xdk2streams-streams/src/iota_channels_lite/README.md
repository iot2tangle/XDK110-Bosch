# About

This repository contains sample code for you to get started with the Channels application in IOTA Streams.

You can find documentation for these examples on the [IOTA documentation portal](https://docs.iota.org/docs/iota-streams/1.0/overview).

# Goal
The goal is to build a narrow high-level API that can reduce development complexity when working with IOTA-Streams.
The two classes `channel_author` and `channel_subscriber` fix the Tangle as transport medium for the stream, removing the need for the developer to interact or even understand the complexities of IOTA and the IOTA Streams. 

# How It Works

Use `channel_author.open()` to open the channel and get the announcement verifier <br />
Use `channel_author.add_subscriber()` to add a subscriber to the channel <br />
Use `channel_author.write_signed()` to write a signed message(public or masked) into the channel <br />
Use `channel_author.write_tagged()` to write a tagged message(public or masked) into the channel <br />
<br />
Use `channel_subscriber.connect()` to connect to a channel<br />
Use `channel_subscriber.update_keyload()` to update the session key<br />
Use `channel_subscriber.read_signed()` to read a signed message from the channel<br />
Use `channel_subscriber.read_tagged()` to read a tagged message from the channel<br />

# Try it yourself
Clone the repo:<br />
`git clone https://github.com/AleBuser/iota-channels-lite`<br />
Enter into the folder:<br />
`cd iota-channels-lite`<br />
Run the example code:<br />
`cargo run --example example`<br />

# Use it yourself
Add the dependency to the `Cargo.toml` file: <br />
`channels_lite = { git= "https://github.com/AleBuser/iota-channels-lite"}`<br />
Import the channels into you code:<br />
`use channels_lite::channels::channel_author;`<br />
`use channels_lite::channels::channel_subscriber;`<br />


