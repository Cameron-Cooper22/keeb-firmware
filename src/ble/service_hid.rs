use nrf_softdevice::{self, ble::{gatt_server::{self, builder::ServiceBuilder, characteristic::{Attribute, Metadata, Properties}, RegisterError}, GattError, SecurityMode}, Softdevice};
use usbd_hid::descriptor::SerializedDescriptor;

use super::spec::{BleCharacteristics, ToUuid, BLE_HID_SERVICE_UUID};

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct HidService {
    hid_info: u16,
    report_map: u16,
    hid_control: u16,
    pub(crate) input_keyboard: u16,
    input_keyboard_cccd: u16,
    input_keyboard_descriptor: u16,
    pub(crate) output_keyboard: u16,
    output_keyboard_descriptor: u16,
    pub(crate) input_media_keys: u16,
    input_media_keys_cccd: u16,
    input_media_keys_descriptor: u16,
    pub(crate) input_system_keys: u16,
    input_system_keys_cccd: u16,
    input_system_keys_descriptor: u16
}

impl HidService {
    pub(crate) fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, BLE_HID_SERVICE_UUID)?;

        let hid_info_handle = service_builder
            .add_characteristic(
                BleCharacteristics::HidInfo.uuid(),
                Attribute::new([
                    0x1u8, 0x1u8, // hid versioln 1.1
                    0x00u8, //country
                    0x03u8, // remote wake + normally connectable
                ]).security(SecurityMode::JustWorks), 
                Metadata::new(Properties::new().read())
            )?
            .build();

        let report_map_handle = service_builder;
    }

}
