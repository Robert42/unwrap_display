extern crate unwrap_display;
use unwrap_display::*;

#[derive(Clone, Copy, Debug)]
struct ErrorCode(u32);

impl core::fmt::Display for ErrorCode
{
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result
  {
    write!(f, "<error code {:#X?}>", self.0)
  }
}

fn success(val: u32) -> Result<u32, ErrorCode> {Ok(val)}
fn failure(code: u32) -> Result<(), ErrorCode> {Err(ErrorCode(code))}

#[test]
fn no_effect_when_ok()
{
  assert_eq!(success(42).unwrap(), 42);
  assert_eq!(success(42).unwrap_display(), 42);
}

#[test]
#[should_panic(expected = "ErrorCode(42)")]
fn unwrap_uses_debug()
{
  failure(42).unwrap();
}

#[test]
#[should_panic(expected = "<error code 0x2A>")]
fn unwrap_display_uses_display()
{
  failure(42).unwrap_display();
}
