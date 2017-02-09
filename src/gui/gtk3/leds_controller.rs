use errors::*;
use gtk;
use gtk::prelude::*;
use shift_register::*;
use std::sync::{Arc, Mutex};



pub fn all(button: &gtk::ToggleButton, leds: &Arc<Mutex<ShiftRegister>>) -> Result<()> {
    let mut leds = leds.lock().unwrap();
    match button.get_active() {
        true => leds.all()?,
        false => leds.reset()?,
    }

    Ok(())
}

pub fn random(button: &gtk::ToggleButton, leds: &Arc<Mutex<ShiftRegister>>) -> Result<()> {
    let mut leds = leds.lock().unwrap();
    match button.get_active() {
        true => leds.test_random()?,
        false => leds.reset()?,
    }

    Ok(())
}

pub fn one_after_one(button: &gtk::ToggleButton, leds: &Arc<Mutex<ShiftRegister>>) -> Result<()> {
    let mut leds = leds.lock().unwrap();
    match button.get_active() {
        true => {
            for i in 1..21 {
                leds.set(i);

                ::std::thread::sleep(::std::time::Duration::from_millis(100));
            }
            leds.reset()?;
        },
        false => leds.reset()?,
    }

    Ok(())
}

pub fn set(button: &gtk::ToggleButton, leds: &Arc<Mutex<ShiftRegister>>, num: u64) -> Result<()> {
    let mut leds = leds.lock().unwrap();
    match button.get_active() {
        true => leds.set(num)?,
        false => leds.clear(num)?,
    }

    Ok(())
}
