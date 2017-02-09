use errors::*;
use gtk;
use gtk::prelude::*;
use shift_register::*;


pub fn launch(builder: &gtk::Builder, label: &gtk::Label) {
    let button_led1: gtk::ToggleButton = builder.get_object("button_led1").unwrap();


    button_led1.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 1);
    }));

    println!("LEDs, index, Builder: {:?}, Label: {:?}", builder, label);
}
