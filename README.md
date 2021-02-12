# isa-ivt-shunt-rs
Rust code to support the Isabellenhuette IVT-S high voltage shunt

Note: Items which use both current and voltage measurements reference U1, therefore that should be the highest voltage if taking multiple measurements.

Works out of the box with with default configuration. Simply pass in a `&[u8; 6]` reference and the CAN ID to `process_data(id, &data_slice)` and you will get an object which contains the measurements of the sensor.

You can set a catch-all process for your incoming 6 byte data and it will automatically figure out the correct data type based on configured ID and MuxID.

Example usage:

```
extern crate isa_ivt_shunt;

let sensor_config = isa_ivt_shunt::ISAConfig::new();
let sensor_measurement = isa_ivt_shunt::ISAMeasurement::new();

if can_recv_data && data.len == 6 {
    let sensor_data = [ data[0], data[1], data[2], data[3], data[4], data[5] ];
    sensor_measurement.process_data(&sensor_data, can_id);
}
```
