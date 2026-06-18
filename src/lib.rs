use std::fmt::Display;

/// Symbolic representation of all Game Boy operation mnemonics
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Mnemonic {
    NOP,
    STOP,
    HALT,
    DI,
    EI,

    LD,
    LDH,
    PUSH,
    POP,

    INC,
    DEC,
    ADD,
    ADC,
    SUB,
    SBC,
    CP,

    RL,
    RLA,
    RLC,
    RLCA,
    RR,
    RRA,
    RRC,
    RRCA,
    SLA,
    SRA,
    SRL,
    SWAP,

    AND,
    OR,
    XOR,
    CPL,

    BIT,
    RES,
    SET,

    CALL,
    JP,
    JR,
    RET,
    RETI,
    RST,

    DAA,
    SCF,
    CCF,
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Mnemonic {
    pub fn is_jump(&self) -> bool {
        use Mnemonic::*;

        match self {
            CALL | RET | RETI | JP | JR | RST => true,
            _ => false
        }
    }
}

/// Symbolic representation of all Game Boy operation operands
///
/// * `-R` -> Reference. e.g. `HLR -> [HL]`
/// * `-I` -> Increment
/// * `-D` -> Decrement
/// * `c-` -> condition. e.g. `cNC -> NC`
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Operand {
    A,
    B,
    C,
    CR, // [C] -> $FF00+C
    D,
    E,
    H,
    L,
    SP,

    AF,
    BC,
    BCR, // [BC]
    DE,
    DER, // [DE]
    HL,
    HLR, // [HL]
    HLI, // [HLI]
    HLD, // [HLD]

    cZ,
    cNZ,
    cC,
    cNC,

    N8(u8),
    I8(i8),
    N16(u16),
    N16R(u16), // [n16],
    FF(u8), // [$FF+u8]

    Index(u8),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Operand {
    pub fn is_8bit(&self) -> bool {
        use Operand::*;

        match self {
            A | B | C | D | E | H | L | CR | BCR | DER | HLR | HLI | HLD | N8(_) | I8(_) | N16R(_) | FF(_) =>  true,
            _ => false
        }
    }

    pub fn is_16bit(&self) -> bool {
        use Operand::*;

        match self {
            AF | BC | DE | HL | N16(_)  =>  true,
            _ => false
        }
    }

    pub fn is_register(&self) -> bool {
        use Operand::*;

        match self {
            A | B | C | D | E | H | L | AF | BC | DE | HL | N16(_)  =>  true,
            _ => false
        }
    }

    pub fn is_condition(&self) -> bool {
        use Operand::*;

        match self {
            cZ | cNZ | cC | cNC  =>  true,
            _ => false
        }
    }

    pub fn is_ref(&self) -> bool {
        use Operand::*;

        match self {
            CR | BCR | DER | HLR | HLI | HLD | N16R(_) | FF(_) => true,
            _ => false
        }
    }

    pub fn is_immediate(&self) -> bool {
        use Operand::*;

        match self {
            N8(_) | I8(_) | N16(_) =>  true,
            _ => false
        }
    }

    pub fn is_immediate_ref(&self) -> bool {
        if let Operand::N16R(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_ff(&self) -> bool {
        if let Operand::FF(_) = self {
            true
        } else {
            false
        }
    }
}

/// Symbolic representation of a Game Boy binary operation. It contains one opcode Mnemonic for the
/// and 1-2 operands.
pub struct Operation {
    pub mnemonic: Mnemonic,
    pub operands: Vec<Operand>,
}

impl Operation {
    pub fn new(mnemonic: Mnemonic, operands: Vec<Operand>) -> Self {
        Operation { mnemonic, operands }
    }

    pub fn into_tuple(&self) -> (&Mnemonic, &Vec<Operand>) {
        (&self.mnemonic, &self.operands)
    }
}

/// Compute the next offset relative to the current PC address's operation
///
/// # Examples
///
/// ```ignore
/// let bus = SimpleBus::new(vec![0x01, 0x34, 0x12]);
/// let operation = decode(&bus, addr)?;
///
/// assert_eq!(next_offset(operation), 3);
/// ```
///
pub fn next_offset(operation: &Operation) -> u16 {
    // There's at least one byte read
    let mut count = 1;

    use Operand::*;
    use Mnemonic::*;

    // CB Prefix
    if let RLC | RRC | RL | RR | SLA | SRA | SRL | SWAP | BIT | RES | SET = operation.mnemonic {
        count += 1;
    }

    for op in operation.operands.iter() {
        match op {
            N8(_) => {
                if !matches!(operation.mnemonic, RST) {
                    count += 1;
                }
            },
            FF(_) => {
                count += 1;
            }
            I8(_) => {
                count += 1;
            },
            N16(_) => {
                count += 2;
            },
            N16R(_) => {
                count += 2;
            },

            _ => ()
        }
    }

    count
}

/// Translate a symbolic representation into a textual one, compatible with RGBDS syntax and
/// following preferences.
///
/// # Examples
///
/// ```ignore
/// let prefs = Preferences{upcase: true, comma_space: true};
/// let result = render(&(LD, vec![N16(0x1234)]), 0x0, &prefs);
///
/// assert_eq!(result, Ok("LD BC, $1234".to_string()))
/// ```
///
/// # Errors
///
/// * String allocation during formatting failed
///
pub fn render(operation: &Operation, prefs: &Preferences) -> Result<String, std::fmt::Error> {
    let mut buffer = String::new();
    let mn = &operation.mnemonic;
    let ops = &operation.operands;

    use std::fmt::Write;
    use Operand::*;
    use Mnemonic::*;

    write!(buffer, "{}", mn.to_string())?;

    for (idx, op) in ops.iter().enumerate() {
        if idx == 0 {
            write!(buffer, " ")?;
        } else {
            write!(buffer, ",")?;
            if prefs.comma_space {
                write!(buffer, " ")?;
            }
        }

        match op {
            cZ | cNZ | cC | cNC => {
                let v = op.to_string();
                write!(buffer, "{}", &v[1..])?;
            }
            BCR | DER | HLR | CR => {
                let mut v = op.to_string();
                v.pop();
                write!(buffer, "[{}]", v)?;
            },
            HLI | HLD => write!(buffer, "[{}]", op)?,
            N8(byte) => {
                write!(buffer, "${:02X}", byte)?;
            },
            FF(byte) => {
                write!(buffer, "[$FF{:02X}]", byte)?;
            }
            I8(byte) => {
                if let LD = mn {
                    let val = *byte as i8;
                    let sign = if val < 0 { "" } else { "+" };
                    write!(buffer, "SP{}{}", sign, val)?;
                } else {
                    write!(buffer, "{}", *byte as i8)?;
                }
            },
            N16(word) => {
                write!(buffer, "${:04X}", word)?;
            },
            N16R(word) => {
                write!(buffer, "[${:04X}]", word)?;
            },
            Index(idx) => write!(buffer, "{}", idx)?,
            val => write!(buffer, "{}", val.to_string())?,
        }
    }

    if !prefs.upcase {
        buffer = buffer.to_lowercase();
    }

    Ok(buffer)
}

/// Trait to be implemented by the `disassemble` function caller. This allows the `disassemble` function to
/// access the binary Game Boy data.
pub trait MemoryBus {
    fn read_byte(&self, addr: u16) -> Option<u8>;
    fn read_word(&self, addr: u16) -> Option<u16>;
}

/// Vendor-provided bus wrapper over vector of bytes. Also used for testing
pub struct SimpleBus {
    data: Vec<u8>,
}

impl SimpleBus {
    pub fn new(data: Vec<u8>) -> SimpleBus {
        SimpleBus { data }
    }
}

impl MemoryBus for SimpleBus {
    fn read_byte(&self, addr: u16) -> Option<u8> {
        let idx = addr as usize;

        if idx >= self.data.len() {
            None
        } else {
            Some(self.data[idx])
        }
    }

    fn read_word(&self, addr: u16) -> Option<u16> {
        let idx = addr as usize;

        if idx + 1 >= self.data.len() {
            None
        } else {
            Some((self.data[idx + 1] as u16) << 8 | self.data[idx] as u16)
        }
    }
}

/// Display preferences for the `disassemble` function.
///
/// * `upcase`: return the textual representation as UPCASE letters (including hexadecimal
/// representation)
/// * `comma_space`: add or a not a space after a comma with 2 operands (e.g. `ld a, b`)
pub struct Preferences {
    pub upcase: bool,
    pub comma_space: bool,
}

impl Preferences {
    pub fn new() -> Preferences {
        Preferences { upcase: false, comma_space: true }
    }
}

/// Return a PC offset and a textual representation as a String of a Game Boy binary operation following the
/// RGBDS syntax.
///
/// # Example
/// ```ignore
/// let bus = SimpleBus::new(vec![0x01, 0x34, 0x12]);
/// let prefs = Preferences{upcase: true, comma_space: true};
/// let result = disassemble(&bus, 0x0, &prefs);
///
/// assert_eq!(result, Ok((3, "LD BC, $1234".to_string())))
/// ```
///
/// # Errors
///
/// * The operation needs one or two operands but an insufficient number is found
/// * The opcode isn't a valid Game Boy operation (unsupported)
/// * String allocation during formatting failed
///
/// # Result
///
/// Returns a tuple containing the number of bytes (as a u16) consumed and the textual representation in a
/// String.
///
/// The byte count number can be used to increment a PC register in an emulator.
pub fn disassemble(bus: &impl MemoryBus, addr: u16, prefs: &Preferences) -> Result<(u16, String), String> {
    let operation = decode(bus, addr)?;
    let offset = next_offset(&operation);

    let repr = render(&operation, prefs).map_err(|e| format!("A formatting error occured: {}", e))?;

    Ok((offset, repr))
}

#[deprecated(since="1.1.0", note="Please use disassemble instead")]
pub fn disass(bus: &impl MemoryBus, addr: u16, prefs: &Preferences) -> Result<(u16, String), String> {
    disassemble(bus, addr, prefs)
}

// Shortcut to handle fetching next byte and erroring
fn next_byte(bus: &impl MemoryBus, addr: u16) -> Result<u8, String> {
    bus.read_byte(addr).ok_or(format!("Byte operand not available"))
}

// Shortcut to handle fetching next word and erroring
fn next_word(bus: &impl MemoryBus, addr: u16) -> Result<u16, String> {
    bus.read_word(addr).ok_or(format!("Word operand not available"))
}

/// Decode 1-3 bytes and return a symbolic representation using the Mnemonic and Operand types.
///
/// # Example
///
/// ```ignore
/// let bus = SimpleBus::new(vec![0x01, 0x34, 0x12]);
/// let (mn, ops) = decode(&bus, 0x00);
///
/// assert_eq!(mn, Mnemonic::LD);
/// assert_eq!(ops, vec![N16(0x1234)])
/// ```
///
/// # Errors
///
/// * The operation needs one or two operands but an insufficient number is found
/// * The opcode isn't a valid Game Boy operation (unsupported)
///
/// # Returns
///
/// Upon success, this function returns a tuple consisting of a Mnemonic and a Vector of Operand's
///
pub fn decode(bus: &impl MemoryBus, addr: u16) -> Result<Operation, String> {
    let opcode = bus.read_byte(addr).ok_or(format!("Opcode not found at address {:04X}", addr))?;
    let addr = addr + 1;

    use Mnemonic::*;
    use Operand::*;

    let res = match opcode {
        0x00 => (NOP, vec![]),
        0x01 => (LD, vec![BC, N16(next_word(bus, addr)?)]),
        0x02 => (LD, vec![BCR, A]),
        0x03 => (INC, vec![BC]),
        0x04 => (INC, vec![B]),
        0x05 => (DEC, vec![B]),
        0x06 => (LD, vec![B, N8(next_byte(bus, addr)?)]),
        0x07 => (RLCA, vec![]),

        0x08 => (LD, vec![N16R(next_word(bus, addr)?), SP]),
        0x09 => (ADD, vec![HL, BC]),
        0x0A => (LD, vec![A, BCR]),
        0x0B => (DEC, vec![BC]),
        0x0C => (INC, vec![C]),
        0x0D => (DEC, vec![C]),
        0x0E => (LD, vec![C, N8(next_byte(bus, addr)?)]),
        0x0F => (RRCA, vec![]),

        0x10 => (STOP, vec![]),
        0x11 => (LD, vec![DE, N16(next_word(bus, addr)?)]),
        0x12 => (LD, vec![DER, A]),
        0x13 => (INC, vec![DE]),
        0x14 => (INC, vec![D]),
        0x15 => (DEC, vec![D]),
        0x16 => (LD, vec![D, N8(next_byte(bus, addr)?)]),
        0x17 => (RLA, vec![]),

        0x18 => (JR, vec![I8(next_byte(bus, addr)? as i8)]),
        0x19 => (ADD, vec![HL, DE]),
        0x1A => (LD, vec![A, DER]),
        0x1B => (DEC, vec![DE]),
        0x1C => (INC, vec![E]),
        0x1D => (DEC, vec![E]),
        0x1E => (LD, vec![E, N8(next_byte(bus, addr)?)]),
        0x1F => (RRA, vec![]),

        0x20 => (JR, vec![cNZ, I8(next_byte(bus, addr)? as i8)]),
        0x21 => (LD, vec![HL, N16(next_word(bus, addr)?)]),
        0x22 => (LD, vec![HLI, A]),
        0x23 => (INC, vec![HL]),
        0x24 => (INC, vec![H]),
        0x25 => (DEC, vec![H]),
        0x26 => (LD, vec![H, N8(next_byte(bus, addr)?)]),
        0x27 => (DAA, vec![]),

        0x28 => (JR, vec![cZ, I8(next_byte(bus, addr)? as i8)]),
        0x29 => (ADD, vec![HL, HL]),
        0x2A => (LD, vec![A, HLI]),
        0x2B => (DEC, vec![HL]),
        0x2C => (INC, vec![L]),
        0x2D => (DEC, vec![L]),
        0x2E => (LD, vec![L, N8(next_byte(bus, addr)?)]),
        0x2F => (CPL, vec![]),

        0x30 => (JR, vec![cNC, I8(next_byte(bus, addr)? as i8)]),
        0x31 => (LD, vec![SP, N16(next_word(bus, addr)?)]),
        0x32 => (LD, vec![HLD, A]),
        0x33 => (INC, vec![SP]),
        0x34 => (INC, vec![HLR]),
        0x35 => (DEC, vec![HLR]),
        0x36 => (LD, vec![HLR, N8(next_byte(bus, addr)?)]),
        0x37 => (SCF, vec![]),

        0x38 => (JR, vec![cC, I8(next_byte(bus, addr)? as i8)]),
        0x39 => (ADD, vec![HL, SP]),
        0x3A => (LD, vec![A, HLD]),
        0x3B => (DEC, vec![SP]),
        0x3C => (INC, vec![A]),
        0x3D => (DEC, vec![A]),
        0x3E => (LD, vec![A, N8(next_byte(bus, addr)?)]),
        0x3F => (CCF, vec![]),

        0x40 => (LD, vec![B, B]),
        0x41 => (LD, vec![B, C]),
        0x42 => (LD, vec![B, D]),
        0x43 => (LD, vec![B, E]),
        0x44 => (LD, vec![B, H]),
        0x45 => (LD, vec![B, L]),
        0x46 => (LD, vec![B, HLR]),
        0x47 => (LD, vec![B, A]),

        0x48 => (LD, vec![C, B]),
        0x49 => (LD, vec![C, C]),
        0x4A => (LD, vec![C, D]),
        0x4B => (LD, vec![C, E]),
        0x4C => (LD, vec![C, H]),
        0x4D => (LD, vec![C, L]),
        0x4E => (LD, vec![C, HLR]),
        0x4F => (LD, vec![C, A]),

        0x50 => (LD, vec![D, B]),
        0x51 => (LD, vec![D, C]),
        0x52 => (LD, vec![D, D]),
        0x53 => (LD, vec![D, E]),
        0x54 => (LD, vec![D, H]),
        0x55 => (LD, vec![D, L]),
        0x56 => (LD, vec![D, HLR]),
        0x57 => (LD, vec![D, A]),

        0x58 => (LD, vec![E, B]),
        0x59 => (LD, vec![E, C]),
        0x5A => (LD, vec![E, D]),
        0x5B => (LD, vec![E, E]),
        0x5C => (LD, vec![E, H]),
        0x5D => (LD, vec![E, L]),
        0x5E => (LD, vec![E, HLR]),
        0x5F => (LD, vec![E, A]),

        0x60 => (LD, vec![H, B]),
        0x61 => (LD, vec![H, C]),
        0x62 => (LD, vec![H, D]),
        0x63 => (LD, vec![H, E]),
        0x64 => (LD, vec![H, H]),
        0x65 => (LD, vec![H, L]),
        0x66 => (LD, vec![H, HLR]),
        0x67 => (LD, vec![H, A]),

        0x68 => (LD, vec![L, B]),
        0x69 => (LD, vec![L, C]),
        0x6A => (LD, vec![L, D]),
        0x6B => (LD, vec![L, E]),
        0x6C => (LD, vec![L, H]),
        0x6D => (LD, vec![L, L]),
        0x6E => (LD, vec![L, HLR]),
        0x6F => (LD, vec![L, A]),

        0x70 => (LD, vec![HLR, B]),
        0x71 => (LD, vec![HLR, C]),
        0x72 => (LD, vec![HLR, D]),
        0x73 => (LD, vec![HLR, E]),
        0x74 => (LD, vec![HLR, H]),
        0x75 => (LD, vec![HLR, L]),
        0x76 => (HALT, vec![]),
        0x77 => (LD, vec![HLR, A]),

        0x78 => (LD, vec![A, B]),
        0x79 => (LD, vec![A, C]),
        0x7A => (LD, vec![A, D]),
        0x7B => (LD, vec![A, E]),
        0x7C => (LD, vec![A, H]),
        0x7D => (LD, vec![A, L]),
        0x7E => (LD, vec![A, HLR]),
        0x7F => (LD, vec![A, A]),

        0x80 => (ADD, vec![A, B]),
        0x81 => (ADD, vec![A, C]),
        0x82 => (ADD, vec![A, D]),
        0x83 => (ADD, vec![A, E]),
        0x84 => (ADD, vec![A, H]),
        0x85 => (ADD, vec![A, L]),
        0x86 => (ADD, vec![A, HLR]),
        0x87 => (ADD, vec![A, A]),

        0x88 => (ADC, vec![A, B]),
        0x89 => (ADC, vec![A, C]),
        0x8A => (ADC, vec![A, D]),
        0x8B => (ADC, vec![A, E]),
        0x8C => (ADC, vec![A, H]),
        0x8D => (ADC, vec![A, L]),
        0x8E => (ADC, vec![A, HLR]),
        0x8F => (ADC, vec![A, A]),

        0x90 => (SUB, vec![A, B]),
        0x91 => (SUB, vec![A, C]),
        0x92 => (SUB, vec![A, D]),
        0x93 => (SUB, vec![A, E]),
        0x94 => (SUB, vec![A, H]),
        0x95 => (SUB, vec![A, L]),
        0x96 => (SUB, vec![A, HLR]),
        0x97 => (SUB, vec![A, A]),

        0x98 => (SBC, vec![A, B]),
        0x99 => (SBC, vec![A, C]),
        0x9A => (SBC, vec![A, D]),
        0x9B => (SBC, vec![A, E]),
        0x9C => (SBC, vec![A, H]),
        0x9D => (SBC, vec![A, L]),
        0x9E => (SBC, vec![A, HLR]),
        0x9F => (SBC, vec![A, A]),

        0xA0 => (AND, vec![A, B]),
        0xA1 => (AND, vec![A, C]),
        0xA2 => (AND, vec![A, D]),
        0xA3 => (AND, vec![A, E]),
        0xA4 => (AND, vec![A, H]),
        0xA5 => (AND, vec![A, L]),
        0xA6 => (AND, vec![A, HLR]),
        0xA7 => (AND, vec![A, A]),

        0xA8 => (XOR, vec![A, B]),
        0xA9 => (XOR, vec![A, C]),
        0xAA => (XOR, vec![A, D]),
        0xAB => (XOR, vec![A, E]),
        0xAC => (XOR, vec![A, H]),
        0xAD => (XOR, vec![A, L]),
        0xAE => (XOR, vec![A, HLR]),
        0xAF => (XOR, vec![A, A]),

        0xB0 => (OR, vec![A, B]),
        0xB1 => (OR, vec![A, C]),
        0xB2 => (OR, vec![A, D]),
        0xB3 => (OR, vec![A, E]),
        0xB4 => (OR, vec![A, H]),
        0xB5 => (OR, vec![A, L]),
        0xB6 => (OR, vec![A, HLR]),
        0xB7 => (OR, vec![A, A]),

        0xB8 => (CP, vec![A, B]),
        0xB9 => (CP, vec![A, C]),
        0xBA => (CP, vec![A, D]),
        0xBB => (CP, vec![A, E]),
        0xBC => (CP, vec![A, H]),
        0xBD => (CP, vec![A, L]),
        0xBE => (CP, vec![A, HLR]),
        0xBF => (CP, vec![A, A]),

        0xC0 => (RET, vec![cNZ]),
        0xC1 => (POP, vec![BC]),
        0xC2 => (JP, vec![cNZ, N16(next_word(bus, addr)?)]),
        0xC3 => (JP, vec![N16(next_word(bus, addr)?)]),
        0xC4 => (CALL, vec![cNZ, N16(next_word(bus, addr)?)]),
        0xC5 => (PUSH, vec![BC]),
        0xC6 => (ADD, vec![A, N8(next_byte(bus, addr)?)]),
        0xC7 => (RST, vec![N8(0x00)]),

        0xC8 => (RET, vec![cZ]),
        0xC9 => (RET, vec![]),
        0xCA => (JP, vec![cZ, N16(next_word(bus, addr)?)]),
        0xCB => cb_prefix(next_byte(bus, addr)?),
        0xCC => (CALL, vec![cZ, N16(next_word(bus, addr)?)]),
        0xCD => (CALL, vec![N16(next_word(bus, addr)?)]),
        0xCE => (ADC, vec![A, N8(next_byte(bus, addr)?)]),
        0xCF => (RST, vec![N8(0x08)]),

        0xD0 => (RET, vec![cNC]),
        0xD1 => (POP, vec![DE]),
        0xD2 => (JP, vec![cNC, N16(next_word(bus, addr)?)]),
        0xD4 => (CALL, vec![cNC, N16(next_word(bus, addr)?)]),
        0xD5 => (PUSH, vec![DE]),
        0xD6 => (SUB, vec![A, N8(next_byte(bus, addr)?)]),
        0xD7 => (RST, vec![N8(0x10)]),

        0xD8 => (RET, vec![C]),
        0xD9 => (RETI, vec![]),
        0xDA => (JP, vec![cC, N16(next_word(bus, addr)?)]),
        0xDC => (CALL, vec![cC, N16(next_word(bus, addr)?)]),
        0xDE => (SBC, vec![A, N8(next_byte(bus, addr)?)]),
        0xDF => (RST, vec![N8(0x18)]),

        0xE0 => (LDH, vec![FF(next_byte(bus, addr)?), A]),
        0xE1 => (POP, vec![HL]),
        0xE2 => (LDH, vec![CR, A]),
        0xE5 => (PUSH, vec![HL]),
        0xE6 => (AND, vec![A, N8(next_byte(bus, addr)?)]),
        0xE7 => (RST, vec![N8(0x20)]),

        0xE8 => (ADD, vec![SP, I8(next_byte(bus, addr)? as i8)]),
        0xE9 => (JP, vec![HL]),
        0xEA => (LD, vec![N16R(next_word(bus, addr)?), A]),
        0xEE => (XOR, vec![A, N8(next_byte(bus, addr)?)]),
        0xEF => (RST, vec![N8(0x28)]),

        0xF0 => (LDH, vec![A, FF(next_byte(bus, addr)?)]),
        0xF1 => (POP, vec![AF]),
        0xF2 => (LDH, vec![A, CR]),
        0xF3 => (DI, vec![]),
        0xF5 => (PUSH, vec![AF]),
        0xF6 => (OR, vec![A, N8(next_byte(bus, addr)?)]),
        0xF7 => (RST, vec![N8(0x30)]),

        0xF8 => (LD, vec![HL, I8(next_byte(bus, addr)? as i8)]),
        0xF9 => (LD, vec![SP, HL]),
        0xFA => (LD, vec![A, N16R(next_word(bus, addr)?)]),
        0xFB => (EI, vec![]),
        0xFE => (CP, vec![A, N8(next_byte(bus, addr)?)]),
        0xFF => (RST, vec![N8(0x38)]),

        _ => Err(format!("Unsupported opcode {:02X}", opcode))?,
    };

    Ok(Operation::new(res.0, res.1))
}

fn cb_prefix(opcode: u8) -> (Mnemonic, Vec<Operand>) {
    use Mnemonic::*;
    use Operand::*;

    match opcode {
        0x00 => (RLC, vec![B]),
        0x01 => (RLC, vec![C]),
        0x02 => (RLC, vec![D]),
        0x03 => (RLC, vec![E]),
        0x04 => (RLC, vec![H]),
        0x05 => (RLC, vec![L]),
        0x06 => (RLC, vec![HLR]),
        0x07 => (RLC, vec![A]),

        0x08 => (RRC, vec![B]),
        0x09 => (RRC, vec![C]),
        0x0A => (RRC, vec![D]),
        0x0B => (RRC, vec![E]),
        0x0C => (RRC, vec![H]),
        0x0D => (RRC, vec![L]),
        0x0E => (RRC, vec![HLR]),
        0x0F => (RRC, vec![A]),

        0x10 => (RL, vec![B]),
        0x11 => (RL, vec![C]),
        0x12 => (RL, vec![D]),
        0x13 => (RL, vec![E]),
        0x14 => (RL, vec![H]),
        0x15 => (RL, vec![L]),
        0x16 => (RL, vec![HLR]),
        0x17 => (RL, vec![A]),

        0x18 => (RR, vec![B]),
        0x19 => (RR, vec![C]),
        0x1A => (RR, vec![D]),
        0x1B => (RR, vec![E]),
        0x1C => (RR, vec![H]),
        0x1D => (RR, vec![L]),
        0x1E => (RR, vec![HLR]),
        0x1F => (RR, vec![A]),

        0x20 => (SLA, vec![B]),
        0x21 => (SLA, vec![C]),
        0x22 => (SLA, vec![D]),
        0x23 => (SLA, vec![E]),
        0x24 => (SLA, vec![H]),
        0x25 => (SLA, vec![L]),
        0x26 => (SLA, vec![HLR]),
        0x27 => (SLA, vec![A]),

        0x28 => (SRA, vec![B]),
        0x29 => (SRA, vec![C]),
        0x2A => (SRA, vec![D]),
        0x2B => (SRA, vec![E]),
        0x2C => (SRA, vec![H]),
        0x2D => (SRA, vec![L]),
        0x2E => (SRA, vec![HLR]),
        0x2F => (SRA, vec![A]),

        0x30 => (SWAP, vec![B]),
        0x31 => (SWAP, vec![C]),
        0x32 => (SWAP, vec![D]),
        0x33 => (SWAP, vec![E]),
        0x34 => (SWAP, vec![H]),
        0x35 => (SWAP, vec![L]),
        0x36 => (SWAP, vec![HLR]),
        0x37 => (SWAP, vec![A]),

        0x38 => (SRL, vec![B]),
        0x39 => (SRL, vec![C]),
        0x3A => (SRL, vec![D]),
        0x3B => (SRL, vec![E]),
        0x3C => (SRL, vec![H]),
        0x3D => (SRL, vec![L]),
        0x3E => (SRL, vec![HLR]),
        0x3F => (SRL, vec![A]),

        0x40 => (BIT, vec![Index(0), B]),
        0x41 => (BIT, vec![Index(0), C]),
        0x42 => (BIT, vec![Index(0), D]),
        0x43 => (BIT, vec![Index(0), E]),
        0x44 => (BIT, vec![Index(0), H]),
        0x45 => (BIT, vec![Index(0), L]),
        0x46 => (BIT, vec![Index(0), HLR]),
        0x47 => (BIT, vec![Index(0), A]),

        0x48 => (BIT, vec![Index(1), B]),
        0x49 => (BIT, vec![Index(1), C]),
        0x4A => (BIT, vec![Index(1), D]),
        0x4B => (BIT, vec![Index(1), E]),
        0x4C => (BIT, vec![Index(1), H]),
        0x4D => (BIT, vec![Index(1), L]),
        0x4E => (BIT, vec![Index(1), HLR]),
        0x4F => (BIT, vec![Index(1), A]),

        0x50 => (BIT, vec![Index(2), B]),
        0x51 => (BIT, vec![Index(2), C]),
        0x52 => (BIT, vec![Index(2), D]),
        0x53 => (BIT, vec![Index(2), E]),
        0x54 => (BIT, vec![Index(2), H]),
        0x55 => (BIT, vec![Index(2), L]),
        0x56 => (BIT, vec![Index(2), HLR]),
        0x57 => (BIT, vec![Index(2), A]),

        0x58 => (BIT, vec![Index(3), B]),
        0x59 => (BIT, vec![Index(3), C]),
        0x5A => (BIT, vec![Index(3), D]),
        0x5B => (BIT, vec![Index(3), E]),
        0x5C => (BIT, vec![Index(3), H]),
        0x5D => (BIT, vec![Index(3), L]),
        0x5E => (BIT, vec![Index(3), HLR]),
        0x5F => (BIT, vec![Index(3), A]),

        0x60 => (BIT, vec![Index(4), B]),
        0x61 => (BIT, vec![Index(4), C]),
        0x62 => (BIT, vec![Index(4), D]),
        0x63 => (BIT, vec![Index(4), E]),
        0x64 => (BIT, vec![Index(4), H]),
        0x65 => (BIT, vec![Index(4), L]),
        0x66 => (BIT, vec![Index(4), HLR]),
        0x67 => (BIT, vec![Index(4), A]),

        0x68 => (BIT, vec![Index(5), B]),
        0x69 => (BIT, vec![Index(5), C]),
        0x6A => (BIT, vec![Index(5), D]),
        0x6B => (BIT, vec![Index(5), E]),
        0x6C => (BIT, vec![Index(5), H]),
        0x6D => (BIT, vec![Index(5), L]),
        0x6E => (BIT, vec![Index(5), HLR]),
        0x6F => (BIT, vec![Index(5), A]),

        0x70 => (BIT, vec![Index(6), B]),
        0x71 => (BIT, vec![Index(6), C]),
        0x72 => (BIT, vec![Index(6), D]),
        0x73 => (BIT, vec![Index(6), E]),
        0x74 => (BIT, vec![Index(6), H]),
        0x75 => (BIT, vec![Index(6), L]),
        0x76 => (BIT, vec![Index(6), HLR]),
        0x77 => (BIT, vec![Index(6), A]),

        0x78 => (BIT, vec![Index(7), B]),
        0x79 => (BIT, vec![Index(7), C]),
        0x7A => (BIT, vec![Index(7), D]),
        0x7B => (BIT, vec![Index(7), E]),
        0x7C => (BIT, vec![Index(7), H]),
        0x7D => (BIT, vec![Index(7), L]),
        0x7E => (BIT, vec![Index(7), HLR]),
        0x7F => (BIT, vec![Index(7), A]),

        0x80 => (RES, vec![Index(0), B]),
        0x81 => (RES, vec![Index(0), C]),
        0x82 => (RES, vec![Index(0), D]),
        0x83 => (RES, vec![Index(0), E]),
        0x84 => (RES, vec![Index(0), H]),
        0x85 => (RES, vec![Index(0), L]),
        0x86 => (RES, vec![Index(0), HLR]),
        0x87 => (RES, vec![Index(0), A]),

        0x88 => (RES, vec![Index(1), B]),
        0x89 => (RES, vec![Index(1), C]),
        0x8A => (RES, vec![Index(1), D]),
        0x8B => (RES, vec![Index(1), E]),
        0x8C => (RES, vec![Index(1), H]),
        0x8D => (RES, vec![Index(1), L]),
        0x8E => (RES, vec![Index(1), HLR]),
        0x8F => (RES, vec![Index(1), A]),

        0x90 => (RES, vec![Index(2), B]),
        0x91 => (RES, vec![Index(2), C]),
        0x92 => (RES, vec![Index(2), D]),
        0x93 => (RES, vec![Index(2), E]),
        0x94 => (RES, vec![Index(2), H]),
        0x95 => (RES, vec![Index(2), L]),
        0x96 => (RES, vec![Index(2), HLR]),
        0x97 => (RES, vec![Index(2), A]),

        0x98 => (RES, vec![Index(3), B]),
        0x99 => (RES, vec![Index(3), C]),
        0x9A => (RES, vec![Index(3), D]),
        0x9B => (RES, vec![Index(3), E]),
        0x9C => (RES, vec![Index(3), H]),
        0x9D => (RES, vec![Index(3), L]),
        0x9E => (RES, vec![Index(3), HLR]),
        0x9F => (RES, vec![Index(3), A]),

        0xA0 => (RES, vec![Index(4), B]),
        0xA1 => (RES, vec![Index(4), C]),
        0xA2 => (RES, vec![Index(4), D]),
        0xA3 => (RES, vec![Index(4), E]),
        0xA4 => (RES, vec![Index(4), H]),
        0xA5 => (RES, vec![Index(4), L]),
        0xA6 => (RES, vec![Index(4), HLR]),
        0xA7 => (RES, vec![Index(4), A]),

        0xA8 => (RES, vec![Index(5), B]),
        0xA9 => (RES, vec![Index(5), C]),
        0xAA => (RES, vec![Index(5), D]),
        0xAB => (RES, vec![Index(5), E]),
        0xAC => (RES, vec![Index(5), H]),
        0xAD => (RES, vec![Index(5), L]),
        0xAE => (RES, vec![Index(5), HLR]),
        0xAF => (RES, vec![Index(5), A]),

        0xB0 => (RES, vec![Index(6), B]),
        0xB1 => (RES, vec![Index(6), C]),
        0xB2 => (RES, vec![Index(6), D]),
        0xB3 => (RES, vec![Index(6), E]),
        0xB4 => (RES, vec![Index(6), H]),
        0xB5 => (RES, vec![Index(6), L]),
        0xB6 => (RES, vec![Index(6), HLR]),
        0xB7 => (RES, vec![Index(6), A]),

        0xB8 => (RES, vec![Index(7), B]),
        0xB9 => (RES, vec![Index(7), C]),
        0xBA => (RES, vec![Index(7), D]),
        0xBB => (RES, vec![Index(7), E]),
        0xBC => (RES, vec![Index(7), H]),
        0xBD => (RES, vec![Index(7), L]),
        0xBE => (RES, vec![Index(7), HLR]),
        0xBF => (RES, vec![Index(7), A]),

        0xC0 => (SET, vec![Index(0), B]),
        0xC1 => (SET, vec![Index(0), C]),
        0xC2 => (SET, vec![Index(0), D]),
        0xC3 => (SET, vec![Index(0), E]),
        0xC4 => (SET, vec![Index(0), H]),
        0xC5 => (SET, vec![Index(0), L]),
        0xC6 => (SET, vec![Index(0), HLR]),
        0xC7 => (SET, vec![Index(0), A]),

        0xC8 => (SET, vec![Index(1), B]),
        0xC9 => (SET, vec![Index(1), C]),
        0xCA => (SET, vec![Index(1), D]),
        0xCB => (SET, vec![Index(1), E]),
        0xCC => (SET, vec![Index(1), H]),
        0xCD => (SET, vec![Index(1), L]),
        0xCE => (SET, vec![Index(1), HLR]),
        0xCF => (SET, vec![Index(1), A]),

        0xD0 => (SET, vec![Index(2), B]),
        0xD1 => (SET, vec![Index(2), C]),
        0xD2 => (SET, vec![Index(2), D]),
        0xD3 => (SET, vec![Index(2), E]),
        0xD4 => (SET, vec![Index(2), H]),
        0xD5 => (SET, vec![Index(2), L]),
        0xD6 => (SET, vec![Index(2), HLR]),
        0xD7 => (SET, vec![Index(2), A]),

        0xD8 => (SET, vec![Index(3), B]),
        0xD9 => (SET, vec![Index(3), C]),
        0xDA => (SET, vec![Index(3), D]),
        0xDB => (SET, vec![Index(3), E]),
        0xDC => (SET, vec![Index(3), H]),
        0xDD => (SET, vec![Index(3), L]),
        0xDE => (SET, vec![Index(3), HLR]),
        0xDF => (SET, vec![Index(3), A]),

        0xE0 => (SET, vec![Index(4), B]),
        0xE1 => (SET, vec![Index(4), C]),
        0xE2 => (SET, vec![Index(4), D]),
        0xE3 => (SET, vec![Index(4), E]),
        0xE4 => (SET, vec![Index(4), H]),
        0xE5 => (SET, vec![Index(4), L]),
        0xE6 => (SET, vec![Index(4), HLR]),
        0xE7 => (SET, vec![Index(4), A]),

        0xE8 => (SET, vec![Index(5), B]),
        0xE9 => (SET, vec![Index(5), C]),
        0xEA => (SET, vec![Index(5), D]),
        0xEB => (SET, vec![Index(5), E]),
        0xEC => (SET, vec![Index(5), H]),
        0xED => (SET, vec![Index(5), L]),
        0xEE => (SET, vec![Index(5), HLR]),
        0xEF => (SET, vec![Index(5), A]),

        0xF0 => (SET, vec![Index(6), B]),
        0xF1 => (SET, vec![Index(6), C]),
        0xF2 => (SET, vec![Index(6), D]),
        0xF3 => (SET, vec![Index(6), E]),
        0xF4 => (SET, vec![Index(6), H]),
        0xF5 => (SET, vec![Index(6), L]),
        0xF6 => (SET, vec![Index(6), HLR]),
        0xF7 => (SET, vec![Index(6), A]),

        0xF8 => (SET, vec![Index(7), B]),
        0xF9 => (SET, vec![Index(7), C]),
        0xFA => (SET, vec![Index(7), D]),
        0xFB => (SET, vec![Index(7), E]),
        0xFC => (SET, vec![Index(7), H]),
        0xFD => (SET, vec![Index(7), L]),
        0xFE => (SET, vec![Index(7), HLR]),
        0xFF => (SET, vec![Index(7), A]),
    }
}

#[cfg(test)]
mod tests;
