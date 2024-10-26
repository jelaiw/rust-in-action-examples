fn main() {
    let opcode: u16 = 0x71E4;

    // Hexadecimal notation is convenient for these operations as each hexadecimal
    // digit represents 4 bits. A 0xF value selects all bits from a nibble.
    let c = (opcode & 0xF000) >> 12;
    let x = (opcode & 0x0F00) >> 8;
    let y = (opcode & 0x00F0) >> 4;
    let d = (opcode & 0x000F) >> 0;

    assert_eq!(c, 0x7);
    assert_eq!(x, 0x1);
    assert_eq!(y, 0xE);
    assert_eq!(d, 0x4);

    // Select multiple nibbles by increasing the width of the filter. 
    let nnn = opcode & 0x0FFF;
    let kk = opcode & 0x00FF;

    assert_eq!(nnn, 0x1E4);
    assert_eq!(kk, 0xE4);
}
