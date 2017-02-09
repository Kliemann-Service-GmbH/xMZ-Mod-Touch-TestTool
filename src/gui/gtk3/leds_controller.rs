use errors::*;
use gtk;
use gtk::prelude::*;
use shift_register::*;


pub fn all(button: &gtk::ToggleButton) -> Result<()> {
    let mut leds = ShiftRegister::new(ShiftRegisterType::LED);

    match button.get_active() {
        true => leds.all()?,
        false => leds.reset()?,
    }

    Ok(())
}

pub fn random(button: &gtk::ToggleButton) {

    println!("Random LEDs {}!", button.get_active());
}

pub fn one_after_one(button: &gtk::ToggleButton) {

    println!("One after One LEDs {}!", button.get_active());
}
