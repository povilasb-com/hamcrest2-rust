// Copyright 2016 Urban Hafner
// Copyright 2018 Val Markovic
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate hamcrest2;

mod none {
  use hamcrest2::prelude::*;

  #[test]
  fn none_no_explicit_type() {
    let var: Option<i8> = None;
    assert_that!(var, none());
  }

  #[test]
  fn none_is_none() {
    assert_that!(None, is(none::<i8>()));
  }

  #[test]
  fn some_is_not_none() {
    assert_that!(Some(1), is_not(none()));
  }
}
