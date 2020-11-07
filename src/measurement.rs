use crate::config::ISAConfig;
pub struct ISAMeasurements {
    pub current: f32,
    pub voltage_1: f32,
    pub voltage_2: f32,
    pub voltage_3: f32,
    pub temperature: f32,
    pub power: f32,
    pub current_counter: f32,
    pub energy_counter: f32,
    pub settings: ISAConfig,
}

impl ISAMeasurements {
    pub fn new(settings: ISAConfig) -> Self {
        // return self
        Self {
            current: 0.0,
            voltage_1: 0.0,
            voltage_2: 0.0,
            voltage_3: 0.0,
            temperature: 0.0,
            power: 0.0,
            current_counter: 0.0,
            energy_counter: 0.0,
            settings: settings,
        }
    }
    pub fn process_data(&mut self, id: u16) {
        if id == 0x511 {
            // Respnse from config
            self.update_config();
        } else if id == self.settings.current.can_id {
            self.update_current();
            println!("Got current!");
        } else if id == self.settings.voltage_1.can_id {
            self.update_voltage_1();
            println!("Got vol 1!");
        } else if id == self.settings.voltage_2.can_id {
            self.update_voltage_2();
            println!("Got vol 2!");
        } else if id == self.settings.voltage_3.can_id {
            self.update_voltage_3();
            println!("Got vol 3!");
        }
    }
    fn update_config(&mut self) {
        self.settings.current.can_id = 0x999;
    }
    fn update_current(&mut self) {
        self.current = 99.9;
    }
    fn update_voltage_1(&mut self) {
        self.voltage_1 = 120.9;
    }

    fn update_voltage_2(&mut self) {
        self.voltage_2 = 240.9;
    }
    fn update_voltage_3(&mut self) {
        self.voltage_3 = 280.9;
    }
}
