use cpuio;
use crate::consts::CMD_END_INTERRUPT;

pub struct PIC {
    pub addr: u8,
    pub command : cpuio::UnsafePort<u8>,
    pub data : cpuio::UnsafePort<u8>,
}

impl PIC {
    pub fn handles_interrupt(&self, interrupt: u8) -> bool {
        interrupt >= self.addr && interrupt < self.addr + 8
    }
    pub unsafe fn send_end_interrupt(&mut self) {
        self.command.write(CMD_END_INTERRUPT);
    }
    pub unsafe fn read_data(&mut self) -> u8 {
        self.data.read()
    }
    pub unsafe fn read_command(&mut self) -> u8 {
        self.command.read()
    }
    pub unsafe fn write_data(&mut self, n_data: u8) {
        self.data.write(n_data);
    }
    pub unsafe fn write_command(&mut self, n_data: u8) {
        self.command.write(n_data);
    }
}

