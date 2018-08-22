extern crate enigo;
extern crate image;
extern crate screenshot;
extern crate bmp;
extern crate rand;

mod GeneticAlgorithm;
mod SystemFunctions;

use std::process::{Stdio, Command};

const W: [u32; 9] = [1, 0, 0, 0, 0, 0, 0, 0, 0];
const A: [u32; 9] = [0, 1, 0, 0, 0, 0, 0, 0, 0];
const S: [u32; 9] = [0, 0, 1, 0, 0, 0, 0, 0, 0];
const D: [u32; 9] = [0, 0, 0, 1, 0, 0, 0, 0, 0];
const L_CLICK: [u32; 9] = [0, 0, 0, 0, 1, 0, 0, 0, 0];
const R_CLICK: [u32; 9] = [0, 0, 0, 0, 0, 1, 0, 0, 0];
const S_BAR: [u32; 9] = [0, 0, 0, 0, 0, 0, 1, 0, 0];
const X_AIM: [u32; 9] = [0, 0, 0, 0, 0, 0, 0, 1, 0];
const Y_AIM: [u32; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 1];

enum Actions {
  W,
  A,
  S,
  D,
  Lclick,
  Rclick,
  Sbar,
  Xaim(i32),
  Yaim(i32)
}

fn create_enviroment(num_frames: i32) {
  let output = if cfg!(target_os = "windows") {
    Command::new(r#"\Users\samue\Documents\projects\NuclearThrone\nuclearthrone.exe"#)
      .stdin(Stdio::piped())
            .output()
            .expect("failed to execute process")
  } else {
    Command::new("sh")
            .arg("-c")
            //.arg("../../Games/nuclear-throne/runner")
            .arg("lutris lutris:rungameid/45")
            .output()
            .expect("failed to execute process")
  };
  println!("{:?}", output);
  
  let mut input_manager = SystemFunctions::inputs::InputManager::new();
  
  for i in 0..num_frames as usize {
    let screen_data = SystemFunctions::screenshot::take_screenshot();
    input_manager.press_w();
  }
}

fn main() {
  SystemFunctions::screenshot::test_screenshot();
  
  let pop_size = 10;
  let mut the_brain = GeneticAlgorithm::brain::Population::new(pop_size);
  the_brain.print_best_fitness_weights();
  
  let num_frames = 10;
  create_enviroment(num_frames);
}

