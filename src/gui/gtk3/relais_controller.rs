use errors::*;
use gtk;
use gtk::prelude::*;
use shift_register::*;

pub fn all(button: &gtk::ToggleButton) -> Result<()> {
    let mut relais = ShiftRegister::new(ShiftRegisterType::RELAIS);

    match button.get_active() {
        true => relais.all()?,
        false => relais.reset()?,
    }

    Ok(())
}

pub fn random(button: &gtk::ToggleButton) -> Result<()> {

    println!("Random Relais {}!", button.get_active());

    Ok(())
}

pub fn one_after_one(button: &gtk::ToggleButton) -> Result<()> {

    println!("One after one Relais {}!", button.get_active());

    Ok(())
}

pub fn set(button: &gtk::ToggleButton, num: u64) -> Result<()> {
    let mut relais = ShiftRegister::new(ShiftRegisterType::RELAIS);

    match button.get_active() {
        true => relais.set(num)?,
        false => relais.clear(num)?,
    }

    Ok(())
}
