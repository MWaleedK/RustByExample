pub struct Converter {
    days: u32,
    years: u16,
}

impl Converter {
    pub fn new() -> Converter {
        Converter { days: 0, years: 0 }
    }
    pub fn set_value_in_years(&mut self, years: u16) {
        if years < 1 {
            panic!("Age cannot be less than 1 years");
        } else {
            self.years = years;
        }
    }
    pub fn get_value_in_days(&mut self) -> u32 {
        self.count();
        self.days
    }
    fn count(&mut self) {
        self.days = 365 * self.years as u32;
    }
    pub fn get_value_in_years(&mut self) -> u16 {
        self.years
    }
}
