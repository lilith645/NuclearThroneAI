extern crate enigo;
extern crate image;
extern crate screenshot;
extern crate bmp;
extern crate rand;

use std::thread;

mod GeneticAlgorithm;
mod SystemFunctions;

use std::process::{Stdio, Command};

const _W: [u32; 9] = [1, 0, 0, 0, 0, 0, 0, 0, 0];
const _A: [u32; 9] = [0, 1, 0, 0, 0, 0, 0, 0, 0];
const _S: [u32; 9] = [0, 0, 1, 0, 0, 0, 0, 0, 0];
const _D: [u32; 9] = [0, 0, 0, 1, 0, 0, 0, 0, 0];
const _L_CLICK: [u32; 9] = [0, 0, 0, 0, 1, 0, 0, 0, 0];
const _R_CLICK: [u32; 9] = [0, 0, 0, 0, 0, 1, 0, 0, 0];
const _S_BAR: [u32; 9] = [0, 0, 0, 0, 0, 0, 1, 0, 0];
const _X_AIM: [u32; 9] = [0, 0, 0, 0, 0, 0, 0, 1, 0];
const _Y_AIM: [u32; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 1];

enum _Actions {
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

fn _create_enviroment(num_frames: i32) {
  let thread = thread::spawn(move || {
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
  });
  
  let mut input_manager = SystemFunctions::inputs::InputManager::new();
  
  for i in 0..num_frames as usize {
    let screen_data = SystemFunctions::screenshot::take_screenshot();
    input_manager.press_w();
  }
}

fn main() {
 // SystemFunctions::screenshot::test_screenshot();
  
  let pop_size = 100;
  let goal = vec!(0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1);
  let input = vec!(0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1);
  let mut brain = GeneticAlgorithm::brain::Population::new(pop_size, input.len(), goal.len(), goal);
  
  for i in 0..3 {
    brain.run_generation(input.clone());
    brain.calculate_fitness();
    brain.print_best_fitness_info();
    //the_brain.print_best_fitness_weights();
    
    brain.next_generation();
  }
  
  println!("Final Best");
  brain.calculate_fitness();
  brain.print_best_fitness_info();
  brain.print_best_fitness_weights();
 // let num_frames = 1000;
//  create_enviroment(num_frames);
}

