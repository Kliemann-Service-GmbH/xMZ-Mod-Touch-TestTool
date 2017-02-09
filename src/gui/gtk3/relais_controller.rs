use errors::*;
use gtk;
use gtk::prelude::*;
use shift_register::*;
use std::sync::{Arc, Mutex};


pub fn all(button: &gtk::ToggleButton, relais: &Arc<Mutex<ShiftRegister>>) -> Result<()> {
    let mut relais = relais.lock().unwrap();
    match button.get_active() {
        true => relais.all()?,
        false => relais.reset()?,
    }

    Ok(())
}

pub fn random(button: &gtk::ToggleButton, relais: &Arc<Mutex<ShiftRegister>>) -> Result<()> {
    let mut relais = relais.lock().unwrap();
    match button.get_active() {
        true => relais.test_random()?,
        false => relais.reset()?,
    }

    Ok(())}

pub fn one_after_one(button: &gtk::ToggleButton, relais: &Arc<Mutex<ShiftRegister>>) -> Result<()> {
    let mut relais = relais.lock().unwrap();
    match button.get_active() {
        true => {
            for i in 1..10 {
                relais.set(i);

                ::std::thread::sleep(::std::time::Duration::from_millis(100));
            }
            relais.reset()?;
        },
        false => relais.reset()?,
    }

    Ok(())
}

pub fn set(button: &gtk::ToggleButton, relais: &Arc<Mutex<ShiftRegister>>, num: u64) -> Result<()> {
    let mut relais = relais.lock().unwrap();
    match button.get_active() {
        true => relais.set(num)?,
        false => relais.clear(num)?,
    }

    Ok(())
}
