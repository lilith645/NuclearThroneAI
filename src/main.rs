extern crate enigo;
extern crate image;

#[macro_use]
extern crate syscall;

fn main() {
  println!("Hello, world!");
  unsafe {
    syscall!(_SYSCTL, "gnome_screenshot".as_ptr());
  }
}
