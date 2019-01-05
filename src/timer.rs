use cpuio::Port;

pub struct Timer {
    port: Port<u8>,
}

impl Timer {
    pub unsafe fn new() -> Timer {
        Timer {
            port: Port::new(0x80)
        }
    }

    pub unsafe fn wait(&mut self) {
        self.port.write(42);
    }
}

