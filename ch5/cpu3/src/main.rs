struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    // The stack’s maximum height is 16. After 16 nested function calls, the program encounters a stack overflow.
    stack: [u16; 16],
    // Giving the stack_pointer type usize makes it easier to index values within the stack.
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;
        
            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; },
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        // Adds the current position_in_memory to the stack. This memory address
        // is two bytes higher than the calling location as it is incremented
        // within the body of the run() method.
        stack[sp] = self.position_in_memory as u16;
        // Increments self.stack_pointer to prevent self.position_in_memory from
        // being overwritten until it needs to be accessed again
        // in a subsequent return.
        self.stack_pointer += 1;
        // Modifies self.position_in_memory to effect jumping to that address.
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        // Jumps to the position in memory where an earlier call was made.
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        // The overflowing_add() method for u8 returns (u8, bool).
        // The bool is true when overflow is detected.
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        }
        else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00; // Sets opcode to 0x2100: CALL the function at 0x100.
    mem[0x002] = 0x21; mem[0x003] = 0x00; // Sets opcode to 0x2100: CALL the function at 0x100.
    mem[0x004] = 0x00; mem[0x005] = 0x00; // Sets opcode to 0x0000: HALT.
    // Not strictly necessary as cpu.memory is initialized with null bytes.

    mem[0x100] = 0x80; mem[0x101] = 0x14; // Sets opcode to 0x8014: ADD register 1’s value to register 0.
    mem[0x102] = 0x80; mem[0x103] = 0x14; // Sets opcode to 0x8014: ADD register 1’s value to register 0.
    mem[0x104] = 0x00; mem[0x105] = 0xEE; // Sets opcode to 0x00EE: RETURN.

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 + 10) + (10 + 10) = {}", cpu.registers[0]);
}
