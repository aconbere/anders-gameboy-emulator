use device;

pub struct NotImplemented {}

impl device::Device for NotImplemented {
    fn get(&self, a: u16) -> u8 {
        panic!(
            "Device: {:?} with address {:X} is not implemented.",
            device::get_kind(a),
            a
        )
    }

    fn set(&mut self, a: u16, _: u8) {
        panic!(
            "Device: {:?} with address {:X} is not implemented.",
            device::get_kind(a),
            a
        )
    }
}
