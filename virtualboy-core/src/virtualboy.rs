use super::rom::Rom;
use super::interconnect::Interconnect;
use super::v810::V810;

pub struct VirtualBoy {
    pub interconnect: Interconnect,
    pub cpu: V810,
}

impl VirtualBoy {
    pub fn new(rom: Rom) -> Self {
        let mut cpu = V810::new();
        cpu.reset();

        VirtualBoy {
            interconnect: Interconnect::new(rom),
            cpu,
        }
    }

    pub fn step(&mut self) -> usize {
        let cycles = self.cpu.step(&mut self.interconnect);

        if let Some(interrupt_code) = self.interconnect.cycles(cycles) {
            self.cpu.request_interrupt(interrupt_code);
        }

        cycles
    }
}