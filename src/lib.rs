//! Use the API Documentation below to learn about the various things you can do with this crate.
//!
//! **If this is your first time using Rust or using this crate, read the Guide on
//! [turtle.rs](http://turtle.rs) to learn how to start.**
//!
//! * The [`Turtle` struct](struct.Turtle.html) - lists of all the various drawing commands that the
//!   turtle supports
//! * The [`Drawing` struct](struct.Drawing.html) - allows you to manipulate the title, size,
//!   background and more of the drawing that you are creating
//! * The [`color` module](color/index.html) - describes the different ways to create colors and
//!   includes a list of the hundreds of predefined color names that you can use to easily set the
//!   pen, fill, and background color of your drawings
//!
//! Note: Call [`turtle::start()`](fn.start.html) if you do not create a turtle with
//! [`Turtle::new()`](struct.Turtle.html#method.new) right at the beginning of your program. Most
//! programs will never need to call this function as it is called for you in
//! [`Turtle::new()`](struct.Turtle.html#method.new).
//!
//! # Random Values
//!
//! See the [`rand` module](rand/index.html) for information about generating random colors, speeds,
//! angles, and more which can be used in your programs to produce some interesting results!
//!
//! # Event Handling
//!
//! The [`Event` enum](event/enum.Event.html) documentation provides information about how you can
//! create an event loop. This allows you to draw things in response to certain events like the
//! mouse moving, keys being pressed, and more.
//!
//! The `Turtle` struct contains a few convenience methods so you can do some common event-related
//! things without creating the entire event loop. For example, use
//! [`wait_for_click()`](struct.Turtle.html#method.wait_for_click) to wait for the user to click
//! anywhere on the screen before proceeding.

#[cfg(all(test, not(feature = "test")))]
compile_error!("Make sure you run tests with `cargo test --features test`");

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate piston_window;
extern crate interpolation;
extern crate rand as rand_crate;

mod turtle_window;

mod app;
mod turtle;
mod drawing;
mod speed;
mod point;
mod radians;
mod animation;
mod extensions;
mod renderer;
mod state;
mod query;
mod server;
mod renderer_process;
mod messenger;

pub mod color;
pub mod event;
pub mod rand;

pub use server::start;
pub use point::Point;
pub use turtle::{Turtle, Distance, Angle};
pub use drawing::{Drawing, Size};
pub use speed::{Speed};
pub use color::{Color};
pub use event::Event;
pub use rand::{random, random_range};
