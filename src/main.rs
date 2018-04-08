extern crate pigrust;
extern crate chrono;
extern crate rscam;

// GPIO pin for PIR motion detector
const PIR_INPUT: u32 = 24;


use pigrust::board_control::*;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;
use chrono::prelude::*;
use rscam::*;
use rscam::{Camera, Control, CtrlData};



fn process_trigger() {
  capture_one();
}

fn capture_one() {
  let now: DateTime<Local> = Local::now();
  let time_str = now.format("%Y%m%d_%H%M%S-cap.jpg").to_string();
  let fname = time_str.clone();

  let mut camera = rscam::new("/dev/video0").unwrap();
  camera.set_control(CID_EXPOSURE_AUTO, EXPOSURE_AUTO).unwrap();

  println!("starting camera...");
  camera.start(&rscam::Config {
    interval: (1, 10),      // 10 fps.
    resolution:  (1280, 720), //(1920, 1080), //(1024, 768),
    format: b"MJPG",
    ..Default::default()
  }).unwrap();

  let frame = camera.capture().unwrap();
  let mut file = std::fs::File::create(time_str).unwrap();
  file.write_all(&frame[..]).unwrap();
  println!("wrote {}", fname);

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

  capture_one();

  bc.set_gpio_mode(PIR_INPUT, GpioMode::Input);
  bc.set_pull_up_down(PIR_INPUT, GpioPullOption::Down);

  bc.add_edge_detector(PIR_INPUT,  GpioEdgeDetect::RisingEdge, gpio_trigger_fn);
  loop { 
    sleep(Duration::from_secs(5));
  }
}
