use cpuio;
use crate::consts::CMD_END_INTERRUPT;

pub struct PIC {
    pub addr: u8,
    /// Port to send an interrupt
    pub command : cpuio::UnsafePort<u8>,
    pub data : cpuio::UnsafePort<u8>,
}

impl PIC {
    /// handles an interrupt
    pub fn handles_interrupt(&self, interrupt: u8) -> bool {
        interrupt >= self.addr && interrupt < self.addr + 8
    }
    /// signal the end of the interrupt
    pub unsafe fn send_end_interrupt(&mut self) {
        self.command.write(CMD_END_INTERRUPT);
    }
    /// read from data
    pub unsafe fn read_data(&mut self) -> u8 {
        self.data.read()
    }
    /// read from command
    pub unsafe fn read_command(&mut self) -> u8 {
        self.command.read()
    }
    /// write to data
    pub unsafe fn write_data(&mut self, n_data: u8) {
        self.data.write(n_data);
    }
    /// write to command
    pub unsafe fn write_command(&mut self, n_data: u8) {
        self.command.write(n_data);
    }
}

