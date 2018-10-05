extern crate pigrust;
extern crate chrono;
extern crate rscam;
extern crate runas;
extern crate rand;

// GPIO pin for PIR motion detector
const PIR_INPUT: u32 = 24;

use pigrust::board_control::*;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;
use chrono::prelude::*;
use rscam::*;
use rscam::{Camera, CtrlData};
use rscam::FLAG_DISABLED;
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

fn dump_settings() {
    let camera = Camera::new("/dev/video0").unwrap();

    for wctrl in camera.controls() {
        let ctrl = wctrl.unwrap();

        if let CtrlData::CtrlClass = ctrl.data {
            println!("\n[{}]\n", ctrl.name);
            continue;
        }

        print!("{:>32} ", ctrl.name);

        if ctrl.flags & FLAG_DISABLED != 0 {
            println!("(disabled)");
            continue;
        }

        match ctrl.data {
            CtrlData::Integer { value, default, minimum, maximum, step } =>
                println!("(int)     min={} max={} step={} default={} value={}",
                         minimum, maximum, step, default, value),
            CtrlData::Boolean { value, default } =>
                println!("(bool)    default={} value={}", default, value),
            CtrlData::Menu { value, default, ref items, .. } => {
                println!("(menu)    default={} value={}", default, value);
                //for item in items {
                 //   println!("{:42} {}: {}", "", item.index, item.name);
                //}
            },
            CtrlData::IntegerMenu { value, default, ref items, .. } => {
                println!("(intmenu) default={} value={}", default, value);
                //for item in items {
                 //   println!("{:42} {}: {} ({:#x})", "", item.index, item.value, item.value);
                //}
            },
            CtrlData::Bitmask { value, default, maximum } =>
                println!("(bitmask) max={:x} default={:x} value={:x}", maximum, default, value),
            CtrlData::Integer64 { value, default, minimum, maximum, step } =>
                println!("(int64)   min={} max={} step={} default={} value={}",
                         minimum, maximum, step, default, value),
            CtrlData::String { ref value, minimum, maximum, step } =>
                println!("(str)     min={} max={} step={} value={}",
                         minimum, maximum, step, value),
            CtrlData::Button => println!("(button)"),
            _ => {}
        }
    }
}

/**
This capture method uses the canned `raspistill` command to
capture still images using the best available settings.
This works slightly better for the Pi camera than using the
rscam abstraction layers.

*/
fn capture_raspistill(filename: &str) {
  //let now: DateTime<Local> = Local::now();
  //let time_str = now.format("%Y%m%d_%H%M%S-cap.jpg").to_string();
  //let fname = time_str.clone();

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



}

/**
This capture method uses rscam.
This is a bit more abstracted than the raspberry pi camera,
but might be useful for capture snapshots from a non-Pi board.

*/
fn capture_rscam() {
  let now: DateTime<Local> = Local::now();
  let time_str = now.format("%Y%m%d_%H%M%S-cap.jpg").to_string();
  let fname = time_str.clone();

  let mut camera = rscam::new("/dev/video0").unwrap();
  // v4l2-ctl --set-ctrl=auto_exposure=1
  // v4l2-ctl --set-ctrl=rotate=180
/*
  camera.set_control(CID_BRIGHTNESS, 50).unwrap();
  camera.set_control(CID_ISO_SENSITIVITY_AUTO, ISO_SENSITIVITY_AUTO).unwrap();

  camera.set_control(CID_EXPOSURE_AUTO, EXPOSURE_AUTO).unwrap();
  camera.set_control(CID_AUTO_N_PRESET_WHITE_BALANCE, WHITE_BALANCE_AUTO).unwrap();

  camera.set_control(CID_EXPOSURE_METERING, EXPOSURE_METERING_AVERAGE ).unwrap();

  camera.set_control(CID_ROTATE, 180).unwrap();
*/

  println!("starting camera...");
  camera.start(&rscam::Config {
    interval: (1, 1),      //  fps.
    resolution: (2592 , 1944), //(1280, 720), //(1920, 1080), //(1024, 768),
    format: b"JPEG", //b"MJPG",
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

  //dump_settings();
  //capture_rscam();
  //capture_raspistill();

  bc.set_gpio_mode(PIR_INPUT, GpioMode::Input);
  bc.set_pull_up_down(PIR_INPUT, GpioPullOption::Down);

  // start listening for a rising edge on our PIR sensor input pin 
  bc.add_edge_detector(PIR_INPUT,  GpioEdgeDetect::RisingEdge, gpio_trigger_fn);
  loop { 
    sleep(Duration::from_secs(5));
  }
}
