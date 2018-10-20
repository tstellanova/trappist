extern crate pigrust;
extern crate chrono;
extern crate runas;
extern crate rand;

// GPIO pin for PIR motion detector
const PIR_INPUT: u32 = 3;
// Illumination LED outputs
const LED1: u32 = 23;
const LED2: u32 = 24;

use pigrust::board_control::*;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;
use chrono::prelude::*;
use runas::Command;
#[macro_use]
extern crate lazy_static;

lazy_static! {
  static ref STREAM_BASE: u32 = rand::random();
}

static mut NUM_CAPS: i32 = 0;


fn process_trigger() {
  
  unsafe {
    let fname  = format!("{}-{}.jpg", STREAM_BASE.to_string(), NUM_CAPS.to_string() );
    NUM_CAPS += 1;
    capture_raspistill(&fname);
  }
}


/**
This capture method uses the canned `raspistill` command to
capture still images using the best available settings.
This works slightly better for the Pi camera than using the
rscam abstraction layers.

*/
fn capture_raspistill(filename: &str) {

  // turn on illuminators
  //bc.gpio_write(LED1, 1);
  //bc.gpio_write(LED2, 1);

  //raspistill -v -n -rot 180 -o
  let status = Command::new("raspistill")
	.arg("-n")
	.arg("-rot").arg("180")
	.arg("-t").arg("250")
	.arg("-o").arg(filename)
	.status().expect("cmd failed!");

  if !status.success()  {
    println!("status {}", status);
  }
  else {
    println!("wrote {}", filename);
  }

  // turn off illuminators
  //bc.gpio_write(LED1, 0);
  //bc.gpio_write(LED2, 0);
}



#[no_mangle]
pub extern fn gpio_trigger_fn(_daemon_id: i32, gpio: u32, _level: u32, _tick: u32, _userdata: u32 ) {
  if PIR_INPUT == gpio {
    println!("GPIO triggered!!");
    process_trigger();
  }
}

fn main() {
  let bc  = BoardController::new();
  println!("Initialized pigpiod. ");

  //dump_settings();

  let now: DateTime<Local> = Local::now();
  let time_str = now.format("%Y%m%d_%H%M%S-cap.jpg").to_string();
  let fname = time_str.clone();

  capture_raspistill(&fname);

  bc.set_gpio_mode(LED1, GpioMode::Output);
  bc.set_gpio_mode(LED2, GpioMode::Output);

  bc.gpio_write(LED1, 0);
  bc.gpio_write(LED2, 0);

  bc.set_gpio_mode(PIR_INPUT, GpioMode::Input);
  bc.set_pull_up_down(PIR_INPUT, GpioPullOption::Up);

  // start listening for a rising edge on our PIR sensor input pin 
  bc.add_edge_detector(PIR_INPUT,  GpioEdgeDetect::FallingEdge, gpio_trigger_fn);
  loop { 
    sleep(Duration::from_secs(5));
  }
}
