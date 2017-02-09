// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate gdk;
extern crate gtk;
extern crate libmodbus_rs;
extern crate rand;
extern crate serde_json;
extern crate serde;
extern crate sysfs_gpio;

#[macro_use] mod macros;
pub mod gui {
    pub mod gtk3;
}
pub mod errors;
pub mod shift_register;
