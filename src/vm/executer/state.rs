use super::stack::*;
pub struct State {
    pub stack: Stack,
    current_proto: super::super::super::bin_format::Prototype,
    pc: usize,
    ir: u32,
}
impl State {
    pub fn new(proto:super::super::super::bin_format::Prototype)->State{
        State{
            stack:Stack::new(),
            current_proto:proto,
            pc:0,
            ir:0,
        }
    }
    pub fn pc(&self) -> usize {
        self.pc
    }
    pub fn sub_pc(&mut self, n: usize) {
        self.pc -= n;
    }
    pub fn add_pc(&mut self, n: usize) {
        self.pc += n;
    }

    pub fn fetch(&mut self) -> u32 {
        let instr = self.current_proto.instruction_table[self.pc as usize];
        self.pc += 1;
        self.ir = instr;
        return instr;
    }
    pub fn execute(&mut self) {
        loop {
            let mut ins = self.fetch();
            let iins = ins as u8;
            loop {
                // vm
                if iins == 0x24{
                    return;
                }
                if ins == 0 {
                    // debug
                    println!("{}", "NOP");
                }
                // load
                else if ins < 0 && ins > 0 {
                }
                // cf
                else if ins < 0 && ins > 0 {
                }
                // comp
                else if ins < 0 && ins > 0 {
                }
                // num
                else if iins >= 0x60 && iins < 0x8f {
                    use super::super::op::arith::*;
                    match iins {
                        NEGM => {}
                        ADDM => {}
                        SUBM => {}
                        MULM => {}
                        MODM => {}
                        POWM => {}
                        DIVM => {}
                        NEG => {
                            let opmodes = NEGM_OP.get_fix().opmode.get_ab(ins);
                            let rs1 = opmodes.0;
                            let rs2 = opmodes.1;
                            use super::PrimeType::*;
                            match &mut self.stack.stack[rs2 as usize] {
                                Null => {
                                    self.stack.stack[rs1 as usize] = Null;
                                }
                                Char(c) => {
                                    self.stack.stack[rs1 as usize] = Char((-(*c as i16)) as u16)
                                }
                                Int(i) => {
                                    self.stack.stack[rs1 as usize] = Int((-(*i as i32)) as u32)
                                }
                                Num(n) => self.stack.stack[rs1 as usize] = Num(-*n),
                                _ => unimplemented!(),
                            }
                            break;
                        }
                        ADD => {}
                        SUB => {}
                        MUL => {}
                        MOD => {}
                        POW => {}
                        DIV => {}
                        IDIV => {}
                        BNOT => {}
                        BAND => {}
                        BOR => {}
                        BXOR => {}
                        SHL => {}
                        SHR => {}
                        LEN => {}
                        _ => panic!("ERROR! INSTRUCTION NOT SUPPORTED"),
                    }
                }
                // stack
                else if ins < 0 && ins > 0 {
                }
                // user def
                else if ins < 0 && ins > 0 {
                }
                // debug
                else if ins < 0 && ins > 0 {
                } else {
                    panic!("ERROR INSTRUCTION '0x{:02X}' NOT SUPPORTED", ins);
                }
            }
        }
    }
}
