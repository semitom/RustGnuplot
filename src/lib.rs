// Copyright (c) 2013-2014 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

#![crate_name="gnuplot"]

#![crate_type = "lib"]

#![feature(globs)]
#![feature(macro_rules)]
#![allow(unused_must_use)]

/*!
A simple gnuplot controller.

# Example

~~~ {.rust}
use gnuplot::*;

let x = [0, 1, 2];
let y = [3, 4, 5];
let mut fg = Figure::new();
{
   fg.axes2d()
   .lines(x.iter(), y.iter(), [Caption("A line"), Color("black")]);
}
fg.show();
~~~
*/

pub use coordinates::*;
pub use datatype::*;
pub use figure::*;
pub use internal::axes2d::Axes2D;
pub use internal::axes3d::Axes3D;
pub use options::*;
pub use axes_common::AxesCommon;

#[macro_escape]
mod util;
mod axes_common;
mod writer;
pub mod figure;
pub mod options; 
pub mod datatype;
pub mod coordinates;

mod internal
{
	pub mod axes2d;
	pub mod axes3d;
}
