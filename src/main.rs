extern crate pigrust;
//extern crate runas;
extern crate chrono;
extern crate rscam;

// GPIO pin for PIR motion detector
const PIR_INPUT: u32 = 24;


use pigrust::board_control::*;
//use runas::Command;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;
use chrono::prelude::*;


fn process_trigger() {
  let now: DateTime<Local> = Local::now();
  let time_str = now.format("%Y%m%d_%H%M%S-cap.jpg").to_string();
  let fname = time_str.clone();

  let mut camera = rscam::new("/dev/video0").unwrap();
  camera.start(&rscam::Config {
    interval: (1, 30),      // 30 fps.
    resolution: (3280, 2464), // old: (1024, 768),
    format: b"MJPG",
    ..Default::default()
  }).unwrap();

  let frame = camera.capture().unwrap();
  let mut file = std::fs::File::create(time_str).unwrap();
  file.write_all(&frame[..]).unwrap();
  println!("wrote {}", fname);

  //raspistill -n -v -o test.jpg
  //let status = Command::new("raspistill").arg("-n").arg("-v").arg("-o").arg(time_str).status().expect("failed to snap!");
  //println!("raspistill exited with: {}", status);
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
  bc.set_gpio_mode(PIR_INPUT, GpioMode::Input);
  bc.set_pull_up_down(PIR_INPUT, GpioPullOption::Down);

  bc.add_edge_detector(PIR_INPUT,  GpioEdgeDetect::RisingEdge, gpio_trigger_fn);
  loop { 
    sleep(Duration::from_secs(5));
  }
}
