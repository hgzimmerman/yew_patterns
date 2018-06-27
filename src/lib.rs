extern crate yew;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod directional;
pub use directional::{Sender, Receiver};
