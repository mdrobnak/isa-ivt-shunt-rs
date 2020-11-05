extern crate byteordered;
pub struct ISAConfig {
    pub current: ISAMeasurementConfig,
    pub voltage_1: ISAMeasurementConfig,
    pub voltage_2: ISAMeasurementConfig,
    pub voltage_3: ISAMeasurementConfig,
    /*
    pub temperature: ISAMeasurementConfig,
    pub power: ISAMeasurementConfig,
    pub current_counter: ISAMeasurementConfig,
    pub energy_counter: ISAMeasurementConfig,
    */
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
                can_id: 0x537,
                data_format: byteordered::Endianness::Big,
                sampling_mode: ISASampleModeEnum::Disabled,
            },
            voltage_1: ISAMeasurementConfig {
                can_id: 0x538,
                data_format: byteordered::Endianness::Big,
                sampling_mode: ISASampleModeEnum::Disabled,
            },
            voltage_2: ISAMeasurementConfig {
                can_id: 0x539,
                data_format: byteordered::Endianness::Big,
                sampling_mode: ISASampleModeEnum::Disabled,
            },
            voltage_3: ISAMeasurementConfig {
                can_id: 0x540,
                data_format: byteordered::Endianness::Big,
                sampling_mode: ISASampleModeEnum::Disabled,
            },
        }
    }
}
/*
fn openinverter() -> Self {
    Self {
    }
} */
