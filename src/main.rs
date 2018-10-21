extern crate pigrust;
extern crate chrono;
extern crate runas;

// GPIO pin for PIR motion detector
const PIR_INPUT: u32 = 22;
// Illumination LED outputs
const LED1: u32 = 23;
const LED2: u32 = 24;

use pigrust::board_control::*;
use std::thread::sleep;
use std::time::Duration;
use chrono::prelude::*;
use runas::Command;



fn set_illumination(bc: &BoardController, enable: bool) {
  if enable {
    bc.gpio_write(LED1, 1);
    bc.gpio_write(LED2, 1);
  }
  else {
    bc.gpio_write(LED1, 0);
    bc.gpio_write(LED2, 0);
  }
}


// Capture one still image 
fn capture_one_snapshot() {

  let now: DateTime<Local> = Local::now();
  let time_str = now.format("%Y%m%d_%H%M%S-cap.jpg").to_string();
  let fname = time_str.clone();

  let bc = BoardController::new();
  set_illumination(&bc, true);
  capture_raspistill(&fname);
  set_illumination(&bc, false);

}

/**
This capture method uses the canned `raspistill` command to
capture still images using the best available settings.
This works slightly better for the Pi camera than using the
rscam abstraction layers.

*/
fn capture_raspistill(filename: &str) {

  //raspistill -v -n -rot 180 -o
  let status = Command::new("raspistill")
	.arg("-n")
	.arg("-rot").arg("180")
	.arg("-t").arg("250")
	.arg("-o").arg(filename)
	.status().expect("cmd failed!");

  if !status.success()  {
    println!("cmd failed {}", status);
  }
  else {
    println!("{}", filename);
  }

}

fn safe_shutdown() {
  //shutdown -h now
  let status = Command::new("shutdown")
	.arg("-h").arg("now")
	.status().expect("cmd failed!");

  if !status.success()  {
    println!("shutdown failed {}", status);
  }
  else {
    println!("shutdown");
  }

}


fn main() {
  let now: DateTime<Local> = Local::now();
  let time_str = now.format("### Restart at %Y%m%d_%H%M%S").to_string();
  println!("{}", time_str);

  let bc  = BoardController::new();
  // enable LEDs for illumination
  bc.set_gpio_mode(LED1, GpioMode::Output);
  bc.set_gpio_mode(LED2, GpioMode::Output);
  //PIR input will switch high wnen movement is detected
  bc.set_gpio_mode(PIR_INPUT, GpioMode::Input);
  bc.set_pull_up_down(PIR_INPUT, GpioPullOption::Down);

  // when this process is first launched, we capture an image as soon as we can
  capture_one_snapshot();

  // start listening for a falling edge on our (inverted) PIR sensor input pin
  bc.add_edge_detector_closure(PIR_INPUT, GpioEdgeDetect::RisingEdge,
     |_gpio, _level| {
        capture_one_snapshot();
      });

  //wait around for a while to see if we detect any more motion
  for _watch_time in 0..12 { 
    sleep(Duration::from_secs(5));
  }
  
  //enter HALT state
  safe_shutdown();
}
