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
            A | B | C | D | E | H | L | CR | BCR | DER | HLR | HLI | HLD | N8(_) | I8(_) | N16R(_) =>  true,
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
            CR | BCR | DER | HLR | HLI | HLD | N16R(_) => true,
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
}

/// Symbolic representation of a Game Boy binary operation. It contains one opcode Mnemonic for the
/// and 1-2 operands.
pub type Operation = (Mnemonic, Vec<Operand>);

/// Compute the next offset relative to the current PC address's operation
///
/// # Examples
///
/// ```ignore
/// let bus = SimpleBus::new(vec![0x01, 0x34, 0x12]);
/// let operation = decode(&bus, addr)?;
///
/// assert_eq!(next_operation_offset(operation), 3);
/// ```
///
pub fn next_operation_offset(operation: &Operation) -> u16 {
    // There's at least one byte read
    let mut count = 1;

    use Operand::*;
    use Mnemonic::*;

    // CB Prefix
    if let RLC | RRC | RL | RR | SLA | SRA | SRL | SWAP | BIT | RES | SET = operation.0 {
        count += 1;
    }

    for op in operation.1.iter() {
        match op {
            N8(_) => {
                if !matches!(operation.0, RST) {
                    count += 1;
                }
            },
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
    let mn = &operation.0;
    let ops = &operation.1;

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
                if let LDH = mn {
                    write!(buffer, "[$FF{:02X}]", byte)?;
                } else {
                    write!(buffer, "${:02X}", byte)?;
                }
            },
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
    let offset = next_operation_offset(&operation);

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

    match opcode {
        0x00 => Ok((NOP, vec![])),
        0x01 => Ok((LD, vec![BC, N16(next_word(bus, addr)?)])),
        0x02 => Ok((LD, vec![BCR, A])),
        0x03 => Ok((INC, vec![BC])),
        0x04 => Ok((INC, vec![B])),
        0x05 => Ok((DEC, vec![B])),
        0x06 => Ok((LD, vec![B, N8(next_byte(bus, addr)?)])),
        0x07 => Ok((RLCA, vec![])),

        0x08 => Ok((LD, vec![N16R(next_word(bus, addr)?), SP])),
        0x09 => Ok((ADD, vec![HL, BC])),
        0x0A => Ok((LD, vec![A, BCR])),
        0x0B => Ok((DEC, vec![BC])),
        0x0C => Ok((INC, vec![C])),
        0x0D => Ok((DEC, vec![C])),
        0x0E => Ok((LD, vec![C, N8(next_byte(bus, addr)?)])),
        0x0F => Ok((RRCA, vec![])),

        0x10 => Ok((STOP, vec![])),
        0x11 => Ok((LD, vec![DE, N16(next_word(bus, addr)?)])),
        0x12 => Ok((LD, vec![DER, A])),
        0x13 => Ok((INC, vec![DE])),
        0x14 => Ok((INC, vec![D])),
        0x15 => Ok((DEC, vec![D])),
        0x16 => Ok((LD, vec![D, N8(next_byte(bus, addr)?)])),
        0x17 => Ok((RLA, vec![])),

        0x18 => Ok((JR, vec![I8(next_byte(bus, addr)? as i8)])),
        0x19 => Ok((ADD, vec![HL, DE])),
        0x1A => Ok((LD, vec![A, DER])),
        0x1B => Ok((DEC, vec![DE])),
        0x1C => Ok((INC, vec![E])),
        0x1D => Ok((DEC, vec![E])),
        0x1E => Ok((LD, vec![E, N8(next_byte(bus, addr)?)])),
        0x1F => Ok((RRA, vec![])),

        0x20 => Ok((JR, vec![cNZ, I8(next_byte(bus, addr)? as i8)])),
        0x21 => Ok((LD, vec![HL, N16(next_word(bus, addr)?)])),
        0x22 => Ok((LD, vec![HLI, A])),
        0x23 => Ok((INC, vec![HL])),
        0x24 => Ok((INC, vec![H])),
        0x25 => Ok((DEC, vec![H])),
        0x26 => Ok((LD, vec![H, N8(next_byte(bus, addr)?)])),
        0x27 => Ok((DAA, vec![])),

        0x28 => Ok((JR, vec![cZ, I8(next_byte(bus, addr)? as i8)])),
        0x29 => Ok((ADD, vec![HL, HL])),
        0x2A => Ok((LD, vec![A, HLI])),
        0x2B => Ok((DEC, vec![HL])),
        0x2C => Ok((INC, vec![L])),
        0x2D => Ok((DEC, vec![L])),
        0x2E => Ok((LD, vec![L, N8(next_byte(bus, addr)?)])),
        0x2F => Ok((CPL, vec![])),

        0x30 => Ok((JR, vec![cNC, I8(next_byte(bus, addr)? as i8)])),
        0x31 => Ok((LD, vec![SP, N16(next_word(bus, addr)?)])),
        0x32 => Ok((LD, vec![HLD, A])),
        0x33 => Ok((INC, vec![SP])),
        0x34 => Ok((INC, vec![HLR])),
        0x35 => Ok((DEC, vec![HLR])),
        0x36 => Ok((LD, vec![HLR, N8(next_byte(bus, addr)?)])),
        0x37 => Ok((SCF, vec![])),

        0x38 => Ok((JR, vec![cC, I8(next_byte(bus, addr)? as i8)])),
        0x39 => Ok((ADD, vec![HL, SP])),
        0x3A => Ok((LD, vec![A, HLD])),
        0x3B => Ok((DEC, vec![SP])),
        0x3C => Ok((INC, vec![A])),
        0x3D => Ok((DEC, vec![A])),
        0x3E => Ok((LD, vec![A, N8(next_byte(bus, addr)?)])),
        0x3F => Ok((CCF, vec![])),

        0x40 => Ok((LD, vec![B, B])),
        0x41 => Ok((LD, vec![B, C])),
        0x42 => Ok((LD, vec![B, D])),
        0x43 => Ok((LD, vec![B, E])),
        0x44 => Ok((LD, vec![B, H])),
        0x45 => Ok((LD, vec![B, L])),
        0x46 => Ok((LD, vec![B, HLR])),
        0x47 => Ok((LD, vec![B, A])),

        0x48 => Ok((LD, vec![C, B])),
        0x49 => Ok((LD, vec![C, C])),
        0x4A => Ok((LD, vec![C, D])),
        0x4B => Ok((LD, vec![C, E])),
        0x4C => Ok((LD, vec![C, H])),
        0x4D => Ok((LD, vec![C, L])),
        0x4E => Ok((LD, vec![C, HLR])),
        0x4F => Ok((LD, vec![C, A])),

        0x50 => Ok((LD, vec![D, B])),
        0x51 => Ok((LD, vec![D, C])),
        0x52 => Ok((LD, vec![D, D])),
        0x53 => Ok((LD, vec![D, E])),
        0x54 => Ok((LD, vec![D, H])),
        0x55 => Ok((LD, vec![D, L])),
        0x56 => Ok((LD, vec![D, HLR])),
        0x57 => Ok((LD, vec![D, A])),

        0x58 => Ok((LD, vec![E, B])),
        0x59 => Ok((LD, vec![E, C])),
        0x5A => Ok((LD, vec![E, D])),
        0x5B => Ok((LD, vec![E, E])),
        0x5C => Ok((LD, vec![E, H])),
        0x5D => Ok((LD, vec![E, L])),
        0x5E => Ok((LD, vec![E, HLR])),
        0x5F => Ok((LD, vec![E, A])),

        0x60 => Ok((LD, vec![H, B])),
        0x61 => Ok((LD, vec![H, C])),
        0x62 => Ok((LD, vec![H, D])),
        0x63 => Ok((LD, vec![H, E])),
        0x64 => Ok((LD, vec![H, H])),
        0x65 => Ok((LD, vec![H, L])),
        0x66 => Ok((LD, vec![H, HLR])),
        0x67 => Ok((LD, vec![H, A])),

        0x68 => Ok((LD, vec![L, B])),
        0x69 => Ok((LD, vec![L, C])),
        0x6A => Ok((LD, vec![L, D])),
        0x6B => Ok((LD, vec![L, E])),
        0x6C => Ok((LD, vec![L, H])),
        0x6D => Ok((LD, vec![L, L])),
        0x6E => Ok((LD, vec![L, HLR])),
        0x6F => Ok((LD, vec![L, A])),

        0x70 => Ok((LD, vec![HLR, B])),
        0x71 => Ok((LD, vec![HLR, C])),
        0x72 => Ok((LD, vec![HLR, D])),
        0x73 => Ok((LD, vec![HLR, E])),
        0x74 => Ok((LD, vec![HLR, H])),
        0x75 => Ok((LD, vec![HLR, L])),
        0x76 => Ok((HALT, vec![])),
        0x77 => Ok((LD, vec![HLR, A])),

        0x78 => Ok((LD, vec![A, B])),
        0x79 => Ok((LD, vec![A, C])),
        0x7A => Ok((LD, vec![A, D])),
        0x7B => Ok((LD, vec![A, E])),
        0x7C => Ok((LD, vec![A, H])),
        0x7D => Ok((LD, vec![A, L])),
        0x7E => Ok((LD, vec![A, HLR])),
        0x7F => Ok((LD, vec![A, A])),

        0x80 => Ok((ADD, vec![A, B])),
        0x81 => Ok((ADD, vec![A, C])),
        0x82 => Ok((ADD, vec![A, D])),
        0x83 => Ok((ADD, vec![A, E])),
        0x84 => Ok((ADD, vec![A, H])),
        0x85 => Ok((ADD, vec![A, L])),
        0x86 => Ok((ADD, vec![A, HLR])),
        0x87 => Ok((ADD, vec![A, A])),

        0x88 => Ok((ADC, vec![A, B])),
        0x89 => Ok((ADC, vec![A, C])),
        0x8A => Ok((ADC, vec![A, D])),
        0x8B => Ok((ADC, vec![A, E])),
        0x8C => Ok((ADC, vec![A, H])),
        0x8D => Ok((ADC, vec![A, L])),
        0x8E => Ok((ADC, vec![A, HLR])),
        0x8F => Ok((ADC, vec![A, A])),

        0x90 => Ok((SUB, vec![A, B])),
        0x91 => Ok((SUB, vec![A, C])),
        0x92 => Ok((SUB, vec![A, D])),
        0x93 => Ok((SUB, vec![A, E])),
        0x94 => Ok((SUB, vec![A, H])),
        0x95 => Ok((SUB, vec![A, L])),
        0x96 => Ok((SUB, vec![A, HLR])),
        0x97 => Ok((SUB, vec![A, A])),

        0x98 => Ok((SBC, vec![A, B])),
        0x99 => Ok((SBC, vec![A, C])),
        0x9A => Ok((SBC, vec![A, D])),
        0x9B => Ok((SBC, vec![A, E])),
        0x9C => Ok((SBC, vec![A, H])),
        0x9D => Ok((SBC, vec![A, L])),
        0x9E => Ok((SBC, vec![A, HLR])),
        0x9F => Ok((SBC, vec![A, A])),

        0xA0 => Ok((AND, vec![A, B])),
        0xA1 => Ok((AND, vec![A, C])),
        0xA2 => Ok((AND, vec![A, D])),
        0xA3 => Ok((AND, vec![A, E])),
        0xA4 => Ok((AND, vec![A, H])),
        0xA5 => Ok((AND, vec![A, L])),
        0xA6 => Ok((AND, vec![A, HLR])),
        0xA7 => Ok((AND, vec![A, A])),

        0xA8 => Ok((XOR, vec![A, B])),
        0xA9 => Ok((XOR, vec![A, C])),
        0xAA => Ok((XOR, vec![A, D])),
        0xAB => Ok((XOR, vec![A, E])),
        0xAC => Ok((XOR, vec![A, H])),
        0xAD => Ok((XOR, vec![A, L])),
        0xAE => Ok((XOR, vec![A, HLR])),
        0xAF => Ok((XOR, vec![A, A])),

        0xB0 => Ok((OR, vec![A, B])),
        0xB1 => Ok((OR, vec![A, C])),
        0xB2 => Ok((OR, vec![A, D])),
        0xB3 => Ok((OR, vec![A, E])),
        0xB4 => Ok((OR, vec![A, H])),
        0xB5 => Ok((OR, vec![A, L])),
        0xB6 => Ok((OR, vec![A, HLR])),
        0xB7 => Ok((OR, vec![A, A])),

        0xB8 => Ok((CP, vec![A, B])),
        0xB9 => Ok((CP, vec![A, C])),
        0xBA => Ok((CP, vec![A, D])),
        0xBB => Ok((CP, vec![A, E])),
        0xBC => Ok((CP, vec![A, H])),
        0xBD => Ok((CP, vec![A, L])),
        0xBE => Ok((CP, vec![A, HLR])),
        0xBF => Ok((CP, vec![A, A])),

        0xC0 => Ok((RET, vec![cNZ])),
        0xC1 => Ok((POP, vec![BC])),
        0xC2 => Ok((JP, vec![cNZ, N16(next_word(bus, addr)?)])),
        0xC3 => Ok((JP, vec![N16(next_word(bus, addr)?)])),
        0xC4 => Ok((CALL, vec![cNZ, N16(next_word(bus, addr)?)])),
        0xC5 => Ok((PUSH, vec![BC])),
        0xC6 => Ok((ADD, vec![A, N8(next_byte(bus, addr)?)])),
        0xC7 => Ok((RST, vec![N8(0x00)])),

        0xC8 => Ok((RET, vec![cZ])),
        0xC9 => Ok((RET, vec![])),
        0xCA => Ok((JP, vec![cZ, N16(next_word(bus, addr)?)])),
        0xCB => Ok(cb_prefix(next_byte(bus, addr)?)),
        0xCC => Ok((CALL, vec![cZ, N16(next_word(bus, addr)?)])),
        0xCD => Ok((CALL, vec![N16(next_word(bus, addr)?)])),
        0xCE => Ok((ADC, vec![A, N8(next_byte(bus, addr)?)])),
        0xCF => Ok((RST, vec![N8(0x08)])),

        0xD0 => Ok((RET, vec![cNC])),
        0xD1 => Ok((POP, vec![DE])),
        0xD2 => Ok((JP, vec![cNC, N16(next_word(bus, addr)?)])),
        0xD4 => Ok((CALL, vec![cNC, N16(next_word(bus, addr)?)])),
        0xD5 => Ok((PUSH, vec![DE])),
        0xD6 => Ok((SUB, vec![A, N8(next_byte(bus, addr)?)])),
        0xD7 => Ok((RST, vec![N8(0x10)])),

        0xD8 => Ok((RET, vec![C])),
        0xD9 => Ok((RETI, vec![])),
        0xDA => Ok((JP, vec![cC, N16(next_word(bus, addr)?)])),
        0xDC => Ok((CALL, vec![cC, N16(next_word(bus, addr)?)])),
        0xDE => Ok((SBC, vec![A, N8(next_byte(bus, addr)?)])),
        0xDF => Ok((RST, vec![N8(0x18)])),

        0xE0 => Ok((LDH, vec![N8(next_byte(bus, addr)?), A])),
        0xE1 => Ok((POP, vec![HL])),
        0xE2 => Ok((LDH, vec![CR, A])),
        0xE5 => Ok((PUSH, vec![HL])),
        0xE6 => Ok((AND, vec![A, N8(next_byte(bus, addr)?)])),
        0xE7 => Ok((RST, vec![N8(0x20)])),

        0xE8 => Ok((ADD, vec![SP, I8(next_byte(bus, addr)? as i8)])),
        0xE9 => Ok((JP, vec![HL])),
        0xEA => Ok((LD, vec![N16R(next_word(bus, addr)?), A])),
        0xEE => Ok((XOR, vec![A, N8(next_byte(bus, addr)?)])),
        0xEF => Ok((RST, vec![N8(0x28)])),

        0xF0 => Ok((LDH, vec![A, N8(next_byte(bus, addr)?)])),
        0xF1 => Ok((POP, vec![AF])),
        0xF2 => Ok((LDH, vec![A, CR])),
        0xF3 => Ok((DI, vec![])),
        0xF5 => Ok((PUSH, vec![AF])),
        0xF6 => Ok((OR, vec![A, N8(next_byte(bus, addr)?)])),
        0xF7 => Ok((RST, vec![N8(0x30)])),

        0xF8 => Ok((LD, vec![HL, I8(next_byte(bus, addr)? as i8)])),
        0xF9 => Ok((LD, vec![SP, HL])),
        0xFA => Ok((LD, vec![A, N16R(next_word(bus, addr)?)])),
        0xFB => Ok((EI, vec![])),
        0xFE => Ok((CP, vec![A, N8(next_byte(bus, addr)?)])),
        0xFF => Ok((RST, vec![N8(0x38)])),

        _ => Err(format!("Unsupported opcode {:02X}", opcode)),
    }
}

fn cb_prefix(opcode: u8) -> Operation {
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
