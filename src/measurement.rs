use crate::config::ISAConfig;
use crate::Endianness;
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
    pub overcurrent_flag: bool,
    pub precision_error: bool,
    pub sensor_alive: bool,
    pub sensor_alive_counter: u8,
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
            overcurrent_flag: false,
            precision_error: false,
            sensor_alive: false,
            sensor_alive_counter: 0,
            settings: settings,
            system_error: false,
        }
    }
    pub fn set_error_status(&mut self, mut error_value: u8) {
        /// Lower nibble: Message Counter
        /// Upper nibble: Error bits
        /// bit 0: set if OCS is true
        /// bit 1: set if
        /// - this result is out of (spec-) range,
        /// - this result has reduced precision
        /// - this result has a measurement-error
        /// bit 2: set if
        /// - any result has a measurement-error
        /// bit 3: set if
        /// - system-error, sensor functionality is not ensured!


        // Test alive first. Counter is 0-15
        // Not sure if there's a better way.
        if ((error & 0x0F) % 16) != (sensor_alive_counter % 16) {
            self.sensor_alive = true;
            self.sensor_alive_counter = (error & 0x0F);
        } else {
            self.sensor_alive = false;
            self.sensor_alive_counter = (error & 0x0F);
        }
        error_value = error_value >> 4;
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
        if (error_value & 0x02) == 0x02 {
            self.precision_error = true;
        } else {
            self.precision_error = false;
        }
        if (error_value & 0x01) == 0x01 {
            self.overcurrent_flag = true;
        } else {
            self.overcurrent_flag = false;
        }
    }
    fn decode_data(&mut self, data: &[u8; 6], data_format: Endianness, divisor: f32) -> f32 {
        if data_format == Endianness::Big {
            // 0x01_05_00_00_88_b8 == 35,000 mV or 35V
            // Value / 1000 = Volts
            let word: i32 = i32::from_be_bytes([data[2], data[3], data[4], data[5]]);
            return word as f32 / divisor;
        } else {
            // 0x01_05_b8_88_00_00 == 35,000 mV or 35V
            // Value / 1000 = Volts
            let word: i32 = i32::from_le_bytes([data[2], data[3], data[4], data[5]]);
            return word as f32 / divisor;
        }
    }
    pub fn process_config_response(&mut self, data: &[u8; 8]) {
        // Respnse from config ID (0x511)
        self.update_config();
    }

    pub fn process_data(&mut self, id: u16, data: &[u8; 6]) {
        if id == self.settings.current.can_id
            || id == self.settings.voltage_1.can_id
            || id == self.settings.voltage_2.can_id
            || id == self.settings.voltage_3.can_id
            || id == self.settings.temperature.can_id
            || id == self.settings.power.can_id
            || id == self.settings.current_counter.can_id
            || id == self.settings.energy_counter.can_id
        {
            // Check the error states
            self.set_error_status(data[1]);
            // Figure out where it belongs
            self.update_data(&data);
        }
    }
    fn update_config(&mut self) {
        self.settings.current.can_id = 0x999;
    }
    fn update_data(&mut self, data: &[u8; 6]) {
        /// MuxID Item              Unit   Divisor
        /// 0x00  IVT_Msg_Result_I  1 mA   1000
        /// 0x01  IVT_Msg_Result_U1 1 mV   1000
        /// 0x02  IVT_Msg_Result_U2 1 mV   1000
        /// 0x03  IVT_Msg_Result_U3 1 mV   1000
        /// 0x04  IVT_Msg_Result_T  0.1 °C 10
        /// 0x05  IVT_Msg_Result_W  1 W    1
        /// 0x06  IVT_Msg_Result_As 1 As   1
        /// 0x07  IVT_Msg_Result_Wh 1 W    1


        // data[0] is the aforementioned MuxID
        match data[0] {
            0 => {
                self.current = self.decode_data(&data, self.settings.current.data_format, 1_000.0);
            }
            1 => {
                self.voltage_1 =
                    self.decode_data(&data, self.settings.voltage_1.data_format, 1_000.0);
            }
            2 => {
                self.voltage_2 =
                    self.decode_data(&data, self.settings.voltage_2.data_format, 1_000.0);
            }
            3 => {
                self.voltage_3 =
                    self.decode_data(&data, self.settings.voltage_3.data_format, 1_000.0);
            }
            4 => {
                self.temperature =
                    self.decode_data(&data, self.settings.temperature.data_format, 10.0);
            }
            5 => {
                self.power = self.decode_data(&data, self.settings.power.data_format, 1.0);
            }
            6 => {
                self.current_counter =
                    self.decode_data(&data, self.settings.current_counter.data_format, 1.0);
            }
            7 => {
                self.energy_counter =
                    self.decode_data(&data, self.settings.energy_counter.data_format, 1.0);
            }
            _ => {}
        }
    }
}
