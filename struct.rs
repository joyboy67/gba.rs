struct Registers {
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  f: u8,
  h: u8,
  l: u8,
}

struct CPU {
  registers: Registers,
  pc: u16,
  sp: u16,
  bus: MemoryBus,
}

struct MemoryBus {
  memory: [u8; 0xFFFF]
}

//

enum Instruction {
  ADD(ArithmeticTarget),
  JP(JumpTest),
  LD(LoadType),
}

enum ArithmeticTarget {
  A, B, C, D, E, H, L,
}

enum JumpTest {
  NotZero,
  Zero,
  NotCarry,
  Carry,
  Always
}

enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}
enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}
enum LoadType {
  Byte(LoadByteTarget, LoadByteSource),
}
