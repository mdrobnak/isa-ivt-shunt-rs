extern crate byteordered;
pub struct ISAConfig {
    pub current: ISAMeasurementConfig,
    pub voltage_1: ISAMeasurementConfig,
    pub voltage_2: ISAMeasurementConfig,
    pub voltage_3: ISAMeasurementConfig,
    pub temperature: ISAMeasurementConfig,
    pub power: ISAMeasurementConfig,
    pub current_counter: ISAMeasurementConfig,
    pub energy_counter: ISAMeasurementConfig,
}

pub struct ISAMeasurementConfig {
    pub can_id: u16,
    pub data_format: byteordered::Endianness,
    pub sampling_mode: ISASampleModeEnum,
}

pub enum ISASampleModeEnum {
    Disabled,
    Triggered,
    Cyclical,
}

impl ISAConfig {
    pub fn new() -> Self {
        Self {
            current: ISAMeasurementConfig {
                can_id: 0x521,
                data_format: byteordered::Endianness::Big,
                sampling_mode: ISASampleModeEnum::Disabled,
            },
            voltage_1: ISAMeasurementConfig {
                can_id: 0x522,
                data_format: byteordered::Endianness::Big,
                sampling_mode: ISASampleModeEnum::Disabled,
            },
            voltage_2: ISAMeasurementConfig {
                can_id: 0x523,
                data_format: byteordered::Endianness::Big,
                sampling_mode: ISASampleModeEnum::Disabled,
            },
            voltage_3: ISAMeasurementConfig {
                can_id: 0x524,
                data_format: byteordered::Endianness::Big,
                sampling_mode: ISASampleModeEnum::Disabled,
            },
            temperature: ISAMeasurementConfig {
                can_id: 0x525,
                data_format: byteordered::Endianness::Big,
                sampling_mode: ISASampleModeEnum::Disabled,
            },
            power: ISAMeasurementConfig {
                can_id: 0x526,
                data_format: byteordered::Endianness::Big,
                sampling_mode: ISASampleModeEnum::Disabled,
            },
            current_counter: ISAMeasurementConfig {
                can_id: 0x527,
                data_format: byteordered::Endianness::Big,
                sampling_mode: ISASampleModeEnum::Disabled,
            },
            energy_counter: ISAMeasurementConfig {
                can_id: 0x528,
                data_format: byteordered::Endianness::Big,
                sampling_mode: ISASampleModeEnum::Disabled,
            },
        }
    }
    pub fn get_serial_number(self) -> (u32, [u32; 8]) {
        let id = 0x411;
        let mut data: [u32; 8] = [0 as u32; 8];
        data[0] = 0x7b; // GET_SERIAL_NUMBER
                        // All the other 7 bytes are 0.
        (id, data)
    }

    pub fn stop_sensor(self) -> (u32, [u32; 8]) {
        let id = 0x411;
        let mut data: [u32; 8] = [0 as u32; 8];
        data[0] = 0x34; // Set operation mode
        data[1] = 0x00; // Stop
        data[2] = 0x01; // Start on reset.
        data[3] = 0x00; // Access key (big end)
        data[4] = 0x00; // Access key (little end)
        (id, data)
    }
    pub fn set_current(self) {
        let id = 0x411;
        let mut data: [u32; 8];
    }
}
/*
fn openinverter() -> Self {
    Self {
    }
} */
