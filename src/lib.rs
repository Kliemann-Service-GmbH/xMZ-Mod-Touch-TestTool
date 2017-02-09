// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
extern crate gdk;
extern crate gtk;
extern crate libmodbus_rs;

#[macro_use] mod macros;
mod gui {
    pub mod gtk3;
}
pub mod errors;
