use errors::*;
use gtk;
use gtk::prelude::*;
use shift_register::*;


pub fn launch(builder: &gtk::Builder, label: &gtk::Label) {
    println!("Relais, index, Builder: {:?}, Label: {:?}", builder, label);
}
