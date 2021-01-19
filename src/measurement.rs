use crate::config::ISAConfig;
pub struct ISAMeasurements {
    pub any_measurement_error: bool,
    pub current: f32,
    pub voltage_1: f32,
    pub voltage_2: f32,
    pub voltage_3: f32,
    pub temperature: f32,
    pub power: f32,
    pub current_counter: f32,
    pub energy_counter: f32,
    pub precision_error: bool,
    pub settings: ISAConfig,
    pub system_error: bool,
}

impl ISAMeasurements {
    pub fn new(settings: ISAConfig) -> Self {
        // return self
        Self {
            any_measurement_error: false,
            current: 0.0,
            voltage_1: 0.0,
            voltage_2: 0.0,
            voltage_3: 0.0,
            temperature: 0.0,
            power: 0.0,
            current_counter: 0.0,
            energy_counter: 0.0,
            precision_error: false,
            settings: settings,
            system_error: false,
        }
    }
    pub fn set_error_status(&mut self, mut error_value: u8) {
        error_value = (error_value >> 4);
        if (error_value & 0x08) == 0x08 {
            self.system_error = true;
            return;
        } else {
            self.system_error = false;
        }
        if (error_value & 0x04) == 0x04 {
            self.any_measurement_error = true;
        } else {
            self.any_measurement_error = false;
        }
        if (error_value & 0x04) == 0x04 {
            self.any_measurement_error = true;
        } else {
            self.any_measurement_error = false;
        }
    }
    pub fn process_data(&mut self, id: u16, data: &[u8; 6]) {
        if id == 0x511 {
            // Respnse from config
            self.update_config();
        } else if id == self.settings.current.can_id {
            // Check the error states
            self.set_error_status(data[1]);
            self.update_current();
            println!("Got current!");
        } else if id == self.settings.voltage_1.can_id {
            // Check the error states
            self.set_error_status(data[1]);
            self.update_voltage_1(&data);
            println!("Got vol 1!");
        } else if id == self.settings.voltage_2.can_id {
            // Check the error states
            self.set_error_status(data[1]);
            self.update_voltage_2();
            println!("Got vol 2!");
        } else if id == self.settings.voltage_3.can_id {
            // Check the error states
            self.set_error_status(data[1]);
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
    fn update_voltage_1(&mut self, data: &[u8; 6]) {
        self.voltage_1 = 120.9; // dummy value if no data given.
        if data[0] == 0x01 {
            // Mux ID 1 is for Voltage 1.
            if self.settings.voltage_1.data_format == byteordered::Endianness::Big {
                // 0x01_05_00_00_88_b8 == 35 mV --- 500 mV -> 0x7A120 = 500,000
                // Value / 1000000 = Volts
                let mut word: u32 = (data[2] as u32) << 24
                    | (data[3] as u32) << 16
                    | (data[4] as u32) << 8
                    | data[5] as u32;
                self.voltage_1 = word as f32 / 1_000_000.0;
            }
        }
    }

    fn update_voltage_2(&mut self) {
        self.voltage_2 = 240.9;
    }
    fn update_voltage_3(&mut self) {
        self.voltage_3 = 280.9;
    }
}
