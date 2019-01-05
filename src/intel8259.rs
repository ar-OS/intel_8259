use crate::timer::Timer;
use crate::pic::PIC;
use crate::consts::{CMD_INIT_MASTER, CMD_INIT_SLAVE, DAT_INIT_MASTER, DAT_INIT_SLAVE, CMD_INIT, CMD_PROTECTED_MODE};

pub struct Intel8259 {
    pics: [PIC; 2],
}

impl Intel8259 {
    pub unsafe fn new(addr_pic1: u8, addr_pic2: u8) -> Intel8259 {
        Intel8259 {
            pics: [
                PIC {
                    addr: addr_pic1,
                    command: cpuio::UnsafePort::new(CMD_INIT_MASTER),
                    data: cpuio::UnsafePort::new(DAT_INIT_MASTER),
                },
                PIC {
                    addr: addr_pic2,
                    command: cpuio::UnsafePort::new(CMD_INIT_SLAVE),
                    data: cpuio::UnsafePort::new(DAT_INIT_SLAVE),
                },
            ]
        }
    }

    pub unsafe fn init(&mut self) {
        // As we don't have interrupts for timers, we use this Linux kernel hack
        // This hack considers that writing garbage at 0x80 takes long time enough
        // to add a delay for old motherboards
        let mut timer = Timer::new();

        // Save the current state of the masks
        let mask_master = self.pics[0].read_data();
        let mask_slave = self.pics[1].read_data();

        // Initialize both PICs
        self.pics[0].write_command(CMD_INIT | CMD_PROTECTED_MODE);
        timer.wait();
        self.pics[1].write_command(CMD_INIT | CMD_PROTECTED_MODE);
        timer.wait();

        self.pics[0].write_data(self.pics[0].addr);
        timer.wait();
        self.pics[1].write_data(self.pics[1].addr);
        timer.wait();

        // Configure the chaining in telling 
        // Master that he needs to write to 
        // Slave at IRQ2 (0x04), and tell Slave
        // its cascade identity (0x02)
        self.pics[0].write_data(0x04);
        timer.wait();
        self.pics[1].write_data(0x02);
        timer.wait();

        self.pics[0].write_data(CMD_PROTECTED_MODE);
        timer.wait();
        self.pics[1].write_data(CMD_PROTECTED_MODE);
        timer.wait();

        // Restore the masks
        self.pics[0].write_data(mask_master);
        self.pics[1].write_data(mask_slave);
    }

    pub fn handles_interrupt(&self, interrupt: u8) -> bool {
        self.pics.iter().any(|p| p.handles_interrupt(interrupt))
    }

    pub unsafe fn send_end_interrupt(&mut self, interrupt: u8) {
        if self.handles_interrupt(interrupt) {
            if self.pics[0].handles_interrupt(interrupt) {
                self.pics[0].send_end_interrupt();
            }
            self.pics[1].send_end_interrupt();
        }
    }
}
