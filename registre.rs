impl Registers {
  fn get_bc(&self) -> u16 {
    (self.b as u16) << 8 | self.c as u16;
  }

  fn set_bc(&mut self, value: u16) {
    self.b = ((value & 0xFF00) >> 8) as u8;
    self.c = (value & 0xFF) as u8;
  }
}
