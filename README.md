# What does this application?

This application is a mock for the vcgencmd command of the Raspberry Pi. It helps you to develop applications, that rely on this Raspberry Pi specific command within an non-Raspberry environment. 


## Build

    cargo build --release

    ./target/release/vcgencmd measure_temp

