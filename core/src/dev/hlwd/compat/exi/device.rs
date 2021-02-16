
/// Representing a particular EXI device.
#[derive(Debug, Clone, Copy)]
pub enum EXIDeviceKind {
    CardSlotA,
    CardSlotB,
    UsbGecko,
}
impl EXIDeviceKind {
    pub fn resolve(idx: usize, cs: u32) -> Option<Self> {
        match (idx, cs) {
            (0, 0) => Some(Self::CardSlotA),
            (1, 0) => Some(Self::CardSlotB),
            (1, 1) => Some(Self::UsbGecko),
            (_, _) => None,
        }
    }
}


//pub trait ExiDevice {
//    fn imm_read(&mut self, gt
//}
