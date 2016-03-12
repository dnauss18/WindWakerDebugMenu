use libtww::game::Flag;
use libtww::Addr;

pub struct FlagEditor {
    pub addr: Addr,
    pub bit: u8,
}

static mut flag_editor: FlagEditor = FlagEditor {
    addr: 0x2C,
    bit: 0,
};

impl FlagEditor {
    pub fn get() -> &'static mut FlagEditor {
        unsafe { &mut flag_editor }
    }

    pub fn next_bit(&mut self) {
        if self.bit < 7 {
            self.bit += 1;
        }
    }

    pub fn previous_bit(&mut self) {
        if self.bit > 0 {
            self.bit -= 1;
        }
    }

    pub fn next_address(&mut self) {
        self.addr += 1;
    }

    pub fn previous_address(&mut self) {
        self.addr -= 1;
    }

    pub fn to_flag(&self) -> Flag {
        let mask = 1 << self.bit;
        Flag(0x803B8700 | self.addr, mask)
    }

    pub fn activate(&self) {
        self.to_flag().activate();
    }

    pub fn deactivate(&self) {
        self.to_flag().deactivate();
    }

    pub fn is_active(&self) -> bool {
        self.to_flag().is_active()
    }

    pub fn flip(&self) {
        let flag = self.to_flag();
        if flag.is_active() {
            flag.deactivate()
        } else {
            flag.activate()
        }
    }
}
