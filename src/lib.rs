#![no_std]

/*!
When an error happens, [`core::result::Result::unwrap`] prints the error using the [`core::fmt::Debug`] instead of the [`core::fmt::Display`] trait.
That's ok as the result of `unwrap` is not meant to be seen by end users  anyway.

Some error types are also incomprehensible for developers when being formatted using `Debug` instead of `Display`.
In this case formatting the error with `Display` can increase productivity.
A typical usecase would be errors from parsers inside unittests.

This crate adds the
[`unwrap_display`](UnwrapDisplay::unwrap_display)
and
[`expect_display_code`](UnwrapDisplay::unwrap_display)
methods to [`core::result::Result`].

Now you can write `do_something_interesting().unwrap_display()` in your tests and get the error formatted with `Display` instead of `Debug.`
*/

/** A type implementing this trait provides the `unwrap_display` method 
*/
pub trait UnwrapDisplay
{
  /** Type of the value to return if there was no error.
  */
  type Inner;

  /** Returns the inner value when there was no error.

  When there was an error it panics displaying the error value.
  */
  #[track_caller]
  fn unwrap_display(self) -> Self::Inner;

  /** Returns the inner value when there was no error.

  When there was an error it panics displaying the passed code.
  */
  #[track_caller]
  fn expect_display_code(self, code: &str) -> Self::Inner;
}

impl<T, E: core::fmt::Display> UnwrapDisplay for Result<T, E>
{
  type Inner = T;

  fn unwrap_display(self) -> Self::Inner
  {
    match self
    {
      Ok(x) => x,
      Err(e) => panic!("{e}"),
    }
  }

  fn expect_display_code(self, code: &str) -> Self::Inner
  {
    match self
    {
      Ok(x) => x,
      Err(e) => panic!("{e}\ncode:\n```\n{code}\n```"),
    }
  }
}
