use errors::*;
use rand::Rng;
use std::thread;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};


#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub enum ShiftRegisterType {
    LED,
    RELAIS,
    Simulation,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShiftRegister {
    register_type: ShiftRegisterType,
    pub oe_pin: Option<u64>,
    pub ds_pin: Option<u64>,
    pub clock_pin: Option<u64>,
    pub latch_pin: Option<u64>,
    #[serde(default)]
    pub data: u64,
}

impl Default for ShiftRegister {
    fn default() -> Self {
        ShiftRegister {
            register_type: ShiftRegisterType::Simulation,
            oe_pin: None,
            ds_pin: None,
            clock_pin: None,
            latch_pin: None,
            data: 0,
        }
    }
}

impl ShiftRegister {
    /// Erzeugt ein neuen Shift Register
    ///
    /// # Arguments
    /// * `register_type`     - Art des Shift Registers
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// assert_eq!(sim.data, 0b0);
    /// ```
    pub fn new(register_type: ShiftRegisterType) -> Self {
        match register_type {
            ShiftRegisterType::LED => ShiftRegister {
                register_type: register_type,
                oe_pin: Some(276),
                ds_pin: Some(38),
                clock_pin: Some(44),
                latch_pin: Some(40),
                data: 0,
            },
            ShiftRegisterType::RELAIS => ShiftRegister {
                register_type: register_type,
                oe_pin: Some(277),
                ds_pin: Some(45),
                clock_pin: Some(39),
                latch_pin: Some(37),
                data: 0,
            },
            ShiftRegisterType::Simulation => ShiftRegister {
                register_type: register_type,
                oe_pin: None,
                ds_pin: None,
                clock_pin: None,
                latch_pin: None,
                data: 0,
            }
        }
    }

    /// Setzt das übergebene Bit im Shift Register `data` Buffer
    ///
    /// # Arguments
    /// * `num`     - Nummer des zu setzenden Bits **Diese Nummer ist Eins basiert!**
    ///
    /// Der Parameter ist nicht Null basiert. Das bedeutet `set(1)` setzt das erste Bit(0) im `data`
    /// Buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// assert_eq!(sim.data, 0b0);
    /// sim.set(3);
    /// assert_eq!(sim.data, 0b100);
    /// ```
    /// More info: http://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit-in-c-c
    pub fn set(&mut self, num: u64) -> Result<()> {
        self.data |= 1 << num -1;
        try!(self.shift_out());

        Ok(())
    }

    /// Abfrage ob ein Bit gesetzt ist, `true` wenn ja, `false` wenn das bit nicht gesetzt ist
    ///
    /// # Arguments
    /// * `num`     - Nummer des abzufragenden Bits **Diese Nummer ist Eins basiert!**
    ///
    /// Der Parameter ist nicht Null basiert. D.h. `get(1)` fragt das erste Bit(0) im `data`
    /// Buffer ab.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// sim.set(1);
    /// sim.set(3);
    /// assert_eq!(sim.get(1), true);
    /// assert_eq!(sim.get(2), false);
    /// assert_eq!(sim.get(3), true);
    /// ```
    /// More info: http://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit-in-c-c
    pub fn get(&self, num: u64) -> bool {
        match (self.data >> num -1) & 1 {
            0 => false,
            _ => true,
        }
    }

    /// Löscht das übergebene Bit
    ///
    /// # Arguments
    /// * `num`     - Nummer des zu löschenden Bits **Diese Nummer ist Eins basiert!**
    ///
    /// Der Parameter ist nicht Null basiert. D.h. `clear(1)` löscht das erste Bit(0) im `data`
    /// Buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// assert_eq!(sim.data, 0b0);
    ///
    /// sim.set(1);
    /// sim.set(3);
    /// assert_eq!(sim.get(1), true);
    /// assert_eq!(sim.get(3), true);
    ///
    /// sim.clear(3);
    /// assert_eq!(sim.get(1), true);
    /// assert_eq!(sim.get(3), false);
    /// ```
    pub fn clear(&mut self, num: u64) -> Result<()> {
        self.data &= !(1 << num - 1);
        try!(self.shift_out());

        Ok(())
    }

    /// Schaltet das übergebene Bit um, war es Null dann wird es Eins und umgekehrt
    ///
    /// # Arguments
    /// * `num`     - Nummer des zu wechselnden Bits **Diese Nummer ist Eins basiert!**
    ///
    /// Der Parameter ist nicht Null basiert. D.h. `toggle(1)` schaltet das erste Bit(0) im `data`
    /// Buffer um.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// assert_eq!(sim.data, 0b0);
    ///
    /// sim.toggle(3);
    /// assert_eq!(sim.get(3), true);
    /// sim.toggle(3);
    /// assert_eq!(sim.get(3), false);
    /// ```
    pub fn toggle(&mut self, num: u64) -> Result<()> {
        self.data ^= 1 << num -1;
        try!(self.shift_out());

        Ok(())
    }

    /// Reset nullt den Datenspeicher und gleicht ihn mit der Hardware ab.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// assert_eq!(sim.get(1), false);
    /// sim.set(1);
    /// assert_eq!(sim.get(1), true);
    /// sim.reset();
    /// assert_eq!(sim.get(1), false);
    /// ```
    pub fn reset(&mut self) -> Result<()> {
        self.data = 0;
        try!(self.shift_out());

        Ok(())
    }

    /// Alle Shift Register werden high gezogen
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    /// assert_eq!(sim.get(1), false);
    /// sim.all();
    /// assert_eq!(sim.get(1), true);
    /// ```
    pub fn all(&mut self) -> Result<()> {
        self.data = u64::max_value();
        try!(self.shift_out());

        Ok(())
    }

    /// Lampentest testet alle Outputs, Reset nach 1Sek
    ///
    /// Diese Funktion schaltet alle Ausgänge high, wartet eine Sekunde und schaltet danach alle
    /// Ausgänge wieder aus.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    ///
    /// sim.test();
    /// ```
    pub fn test(&mut self) -> Result<()> {
        // Alten Stand speichern
        let old_state = self.data;
        // Buffer komplett mit Einsen füllen
        self.data = u64::max_value();
        try!(self.shift_out());
        thread::sleep(Duration::new(1, 0));
        try!(self.reset());
        // alten Stand wieder herstellen
        self.data = old_state;
        try!(self.shift_out());

        Ok(())
    }

    /// Lampentest testet alle Outputs, Reset nach 1Sek
    ///
    /// Diese Funktion schaltet alle Ausgänge high, wartet eine Sekunde und schaltet danach alle
    /// Ausgänge wieder aus.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    ///
    /// sim.test_timed();
    /// ```
    pub fn test_timed(&mut self) -> Result<()> {
        // Alten Stand speichern
        let old_state = self.data;
        // Buffer komplett mit Einsen füllen
        self.data = u64::max_value();
        try!(self.shift_out());
        thread::sleep(Duration::new(1, 0));
        try!(self.reset());
        // alten Stand wieder herstellen
        self.data = old_state;
        try!(self.shift_out());

        Ok(())
    }

    /// Random Lampentest testet einige, Outputs, zufällig
    ///
    /// Diese Funktion setzt das ShiftRegister in einen zufälligen Zustand
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    ///
    /// sim.test_random();
    /// ```
    pub fn test_random(&mut self) -> Result<()> {
        // Buffer mit Zufallsdaten füllen
        self.data =  ::rand::thread_rng().gen_range(1, u64::max_value());

        try!(self.shift_out());

        Ok(())
    }

    /// Random Lampentest testet einige, Outputs, zufällig, Reset nach 1Sek
    ///
    /// Diese Funktion speichert den Zustand des Shift Registers, schaltet einige,
    /// zufällige Ausgänge high, wartet eine Sekunde und schaltet danach alle
    /// Ausgänge wieder auf den vorherigen Zustand.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_server::*;
    ///
    /// let mut sim = ShiftRegister::new(ShiftRegisterType::Simulation);
    ///
    /// sim.test_random_timed();
    /// ```
    pub fn test_random_timed(&mut self) -> Result<()> {
        // Alten Stand speichern
        let old_state = self.data;
        // Buffer mit Zufallsdaten füllen
        self.data =  ::rand::thread_rng().gen_range(1, u64::max_value());
        try!(self.shift_out());
        thread::sleep(Duration::new(1, 0));
        try!(self.reset());
        // alten Stand wieder herstellen
        self.data = old_state;
        try!(self.shift_out());

        Ok(())
    }


    /// Exportiert die Pins in das sysfs des Linux Kernels
    ///
    fn export_pins(&self) -> Result<()> {
        if let Some(oe_pin) = self.oe_pin { try!(Pin::new(oe_pin).export()) };
        if let Some(ds_pin) = self.ds_pin { try!(Pin::new(ds_pin).export()) };
        if let Some(clock_pin) = self.clock_pin { try!(Pin::new(clock_pin).export()) };
        if let Some(latch_pin) = self.latch_pin { try!(Pin::new(latch_pin).export()) };

        Ok(())
    }

    /// Schaltet die Pins in den OUTPUT Pin Modus
    ///
    fn set_pin_direction_output(&self) -> Result<()> {
        if let Some(oe_pin) = self.oe_pin { try!(Pin::new(oe_pin).set_direction(Direction::Out)) };
        if let Some(oe_pin) = self.oe_pin { try!(Pin::new(oe_pin).set_value(0)) }; // !OE pin low == Shift register enabled.
        if let Some(ds_pin) = self.ds_pin { try!(Pin::new(ds_pin).set_direction(Direction::Out)) };
        if let Some(ds_pin) = self.ds_pin { try!(Pin::new(ds_pin).set_value(0)) };
        if let Some(clock_pin) = self.clock_pin { try!(Pin::new(clock_pin).set_direction(Direction::Out)) };
        if let Some(clock_pin) = self.clock_pin { try!(Pin::new(clock_pin).set_value(0)) };
        if let Some(latch_pin) = self.latch_pin { try!(Pin::new(latch_pin).set_direction(Direction::Out)) };
        if let Some(latch_pin) = self.latch_pin { try!(Pin::new(latch_pin).set_value(0)) };

        Ok(())
    }


    /// Toogelt den Clock Pin high->low
    fn clock_in(&self) -> Result<()> {
        if let Some(clock_pin) = self.clock_pin { try!(Pin::new(clock_pin).set_value(1)) };
        if let Some(clock_pin) = self.clock_pin { try!(Pin::new(clock_pin).set_value(0)) };

        Ok(())
    }

    /// Toggelt den Latch Pin pin high->low,
    fn latch_out(&self) -> Result<()> {
        if let Some(latch_pin) = self.latch_pin { try!(Pin::new(latch_pin).set_value(1)) };
        if let Some(latch_pin) = self.latch_pin { try!(Pin::new(latch_pin).set_value(0)) };

        Ok(())
    }

    /// Schiebt die kompletten Daten in die Schiebe Register und schaltet die Ausgänge dieser
    /// Schiebe Register (latch out)
    fn shift_out(&self) -> Result<()> {
        // Wenn export_pins erfolgreich ist werden die Daten eingeclocked, ansonsten passiert nix
        try!(self.export_pins());
        try!(self.set_pin_direction_output());

        // Daten einclocken
        for i in (0..64).rev() {
            match (self.data >> i) & 1 {
                1 => { if let Some(ds_pin) = self.ds_pin { try!(Pin::new(ds_pin).set_value(1)) } },
                _ => { if let Some(ds_pin) = self.ds_pin { try!(Pin::new(ds_pin).set_value(0)) } },
            }
            try!(self.clock_in());
        }
        try!(self.latch_out());

        Ok(())
    }


}
