// Copyright 2018 Val Markovic
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate hamcrest2;

mod ok {
  use hamcrest2::prelude::*;

  #[test]
  fn ok_no_explicit_type() {
    let var: Result<i8, ()> = Ok(5);
    assert_that!(var, ok());
  }

  #[test]
  fn ok_is_ok() {
    assert_that!(Ok(5), ok::<i8, ()>());
  }

  #[test]
  fn err_is_not_ok() {
    let var: Result<i8, ()> = Err(());
    assert_that!(var, not(ok()));
  }
}
