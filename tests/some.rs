// Copyright 2018 Val Markovic
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate hamcrest2;

mod some {
  use hamcrest2::prelude::*;

  #[test]
  fn some_no_explicit_type() {
    let var: Option<i8> = Some(5);
    assert_that!(var, some());
  }

  #[test]
  fn some_is_some() {
    assert_that!(Some(5), some::<i8>());
  }

  #[test]
  fn none_is_not_some() {
    let var: Option<i8> = None;
    assert_that!(var, not(some()));
  }
}
