use usbd_hid::descriptor::generator_prelude::*;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum BleReportType {
    Keyboard = 0x01,
    Mouse = 0x02,
    Media = 0x03,
    System = 0x04,
    Vial = 0x05
}

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        (report_id = 0x01,) = {
            (usage_page = KEYBOARD, usage_min = 0xe0, usage_max = 0xe7) = {
                #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
            };
            (logical_min = 0,) = {
                #[item_settings constant,variable,absolute] reserved=input;
            };
            (usage_page = LEDS, usage_min = 0x01, usage_max = 0x05) = {
                #[packed_bits 5] #[item_settings data,variable,absolute] leds=output;
            };
            (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0xdd) = {
                #[item_settings data,array,absolute] keycodes=input;
            };
        };
    },
    (collection = APPLICATION, usage_page = CONSUMER, usage = CONSUMER_CONTROL) = {
        (report_id = 0x04,) = {
            (usage_min = 0x81, usage_max = 0xb7, logical_min = 0x01) = {
                #[item_settings data,array,absolute,not_null] system_usage_id=input;
            };
        };
    }
)]
#[allow(dead_code)]
pub(crate) struct BleKeyboardReport {
    pub(crate) modifier: u8,
    pub(crate) reserved: u8,
    pub(crate) leds: u8,
    pub(crate) keycodes: [u8; 6],   // for now, 6 KRO
    pub(crate) media_usage_id: u16,
    pub(crate) system_usage_id: u8,
    pub(crate) modifier: u8,
}
