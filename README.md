# trappist
A motion detecting trapcam application written in Rust.

Uses an external hardware [passive infrared/PIR sensor](https://amzn.to/2IQGv8N) to detect the motion of animals 
(or other warm objects) and captures a photo using a `v4l2`-compatible camera, such as the Raspberry Pi camera module.

Originally built and tested on the [Raspberry Pi Zero W](https://amzn.to/2IQmaR1) with the [Raspberry Pi Camera V2](https://amzn.to/2pDIkhf)

## Dependencies

- Uses [`pigpiod`](http://abyz.me.uk/rpi/pigpio/pigpiod.html), which is now present by default in Raspbian Stretch, to detect the change in output state of the PIR detector
