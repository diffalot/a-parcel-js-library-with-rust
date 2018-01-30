#[no_mangle]
pub fn log() { println!("Hello, Parcel!"); }

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
  return a + b
}
