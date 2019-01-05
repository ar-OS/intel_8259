pub const IO_BASE_ADDR_MASTER: u8 = 0x20;
pub const IO_BASE_ADDR_SLAVE: u8 = 0xA0;

// Offset + PIC1 addr
pub const CMD_INIT_MASTER: u16 = IO_BASE_ADDR_MASTER as u16;
pub const DAT_INIT_MASTER: u16 = CMD_INIT_MASTER + 1;
pub const OFFSET_MASTER: u8 = 0x20;
// Offset + PIC2 addr
pub const CMD_INIT_SLAVE: u16 = IO_BASE_ADDR_SLAVE as u16;
pub const DAT_INIT_SLAVE: u16 = CMD_INIT_SLAVE + 1;
pub const OFFSET_SLAVE: u8 = 0x28;
// Offset + PIC1 end
pub const CMD_INIT: u8 = 0x10; // Initialization of the PICs
pub const CMD_END_INTERRUPT: u8 = 0x20;
pub const CMD_PROTECTED_MODE: u8 = 0x01; // CMD for 8086 mode
