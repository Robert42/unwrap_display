#![no_std]

pub trait UnwrapDisplay
{
  type Inner;

  #[track_caller]
  fn unwrap_display(self) -> Self::Inner;

  #[track_caller]
  fn expect_display_code(self, code: &str) -> Self::Inner;
}

impl<T, E: core::fmt::Display> UnwrapDisplay for Result<T, E>
{
  type Inner = T;

  fn unwrap_display(self) -> T
  {
    match self
    {
      Ok(x) => x,
      Err(e) =>
      {
        panic!("{e}");
      }
    }
  }

  #[track_caller]
  fn expect_display_code(self, code: &str) -> Self::Inner
  {
    match self
    {
      Ok(x) => x,
      Err(e) =>
      {
        panic!("{e}\ncode:\n```\n{code}\n```");
      }
    }
  }
}
