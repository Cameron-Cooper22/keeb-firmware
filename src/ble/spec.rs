use nrf_softdevice::ble::Uuid;

pub(crate) const BLE_HID_SERVICE_UUID: Uuid = Uuid::new_16(0x1812);

pub(crate) enum BleSpecification {
    DeviceInformation = 0x180a,
    BatteryService = 0x180f
}

pub(crate) enum BleCharacteristics {
    BatteryLevel = 0x2a19,
    ModelNumber = 0x2a24,
    SerialNumber = 0x2a25,
    FirmwareRevision = 0x2a26,
    HardwareRevision = 0x2a27,
    SoftwareRevision = 0x2a28,
    ManufacturerName = 0x2a29,
    PnpId = 0x2a50,
    HidInfo = 0x2a4a,
    ReportMap = 0x2a4b,
    HidControlPoint = 0x2a4c,
    HidReport = 0x2a4d,
    ProtocolMode = 0x2a4e
}

pub(crate) enum BleDescriptor {
    ReportReference = 0x2908
}

pub(crate) trait ToUuid {
    fn uuid(self) -> Uuid;
}

impl ToUuid for BleDescriptor {
    fn uuid(self) -> Uuid {
        Uuid::new_16(self as u16)
    }
}

impl ToUuid for BleCharacteristics {
    fn uuid(self) -> Uuid {
        Uuid::new_16(self as u16)
    }
}

impl ToUuid for BleSpecification {
    fn uuid(self) -> Uuid {
        Uuid::new_16(self as u16)
    }
}
