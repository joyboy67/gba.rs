impl CPU {
  fn execute(&mut self, instruction: Instruction) {
    match instruction {

      Instruction::ADD(target) => {
        match target {
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let new_value = self.add(value);
            self.registers.a = new_value;
          }
          _ => { /* TODO: plus de cibles */ }
        }
      },

      Instruction::JP(test) => {
        let jump_condition = match test {
            JumpTest::NotZero => !self.registers.f.zero,
            JumpTest::NotCarry => !self.registers.f.carry,
            JumpTest::Zero => self.registers.f.zero,
            JumpTest::Carry => self.registers.f.carry,
            JumpTest::Always => true
        };
        self.jump(jump_condition)
      },

      Instruction::LD(load_type) => {
        match load_type {
        LoadType::Byte(target, source) => {
            let source_value = match source {
              LoadByteSource::A => self.registers.a,
              LoadByteSource::D8 => self.read_next_byte(),
              LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
              _ => { panic!("TODO: implémentation d'autres sources") }
            };
            match target {
              LoadByteTarget::A => self.registers.a = source_value,
              LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
              _ => { panic!("TODO: implémentation de plus de cibles") }
            };
            match source {
              LoadByteSource::D8  => self.pc.wrapping_add(2),
              _                   => self.pc.wrapping_add(1),
            }
          }
          _ => { panic!("TODO: autre types de chargement") }
        }
      },

      Instruction::PUSH(target) => {
        let value = match target {
          StackTarget::BC => self.registers.get_bc(),
          _ => { panic!("TODO: support de plus de cibles") }
        };
        self.push(value);
        self.pc.wrapping_add(1);
      },

      _ => { /* TODO: support de plus d'instructions */ }

    }
  }

  fn add(&mut self, value: u8) -> u8 {
    let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
    self.registers.f.zero = new_value == 0;
    self.registers.f.subtract = false;
    self.registers.f.carry = did_overflow;
    self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
    new_value;
  }

  fn step(&mut self) {
    let mut instruction_byte = self.bus.read_byte(self.pc);

    let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
      self.execute(instruction)
    } else {
      panic!("Instruction inconnue trouvée pour: 0x{:x}", instruction_byte);
    };

    self.pc = next_pc;
  }

    fn jump(&self, should_jump: bool) -> u16 {
     if should_jump {
       let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
       let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
       (most_significant_byte << 8) | least_significant_byte
     } else {

       self.pc.wrapping_add(3)
     }
    }

    fn push(&mut self, value: u16) {
     self.sp = self.sp.wrapping_sub(1);
     self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
     self.sp = self.sp.wrapping_sub(1);
     self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

}
