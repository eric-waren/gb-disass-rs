use std::fmt::Display;

/// Symbolic representation of all Game Boy operation mnemonics
#[derive(Debug)]
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

/// Symbolic representation of all Game Boy operation operands
///
/// * `-R` -> Reference. e.g. `HLR -> [HL]`
/// * `-I` -> Increment
/// * `-D` -> Decrement
/// * `c-` -> condition. e.g. `cNC -> NC`
#[derive(Debug)]
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

/// Symbolic representation of a Game Boy binary operation. It contains one opcode Mnemonic for the
/// and 1-2 operands.
pub type Operation = (Mnemonic, Vec<Operand>);

/// Compute the next offset relative to the current PC address's operation
///
/// # Examples
///
/// ```ignore
/// let bus = GameboyBus::new(vec![0x01, 0x12, 0x34]);
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
/// let bus = GameboyBus::new(vec![0x01, 0x12, 0x34]);
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
/// let bus = GameboyBus::new(vec![0x01, 0x12, 0x34]);
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
mod tests {
    use super::*;

    struct GameboyBus {
        data: Vec<u8>,
    }

    impl GameboyBus {
        pub fn new(data: Vec<u8>) -> GameboyBus {
            GameboyBus { data }
        }
    }

    impl MemoryBus for GameboyBus {
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
                Some((self.data[idx] as u16) << 8 | self.data[idx + 1] as u16)
            }
        }
    }

    // 0x0X
    #[test]
    fn nop() {
        let bus = GameboyBus::new(vec![0x00]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((1, "NOP".to_string())))
    }

    #[test]
    fn ld_bc_n16() {
        let bus = GameboyBus::new(vec![0x01, 0xab, 0xcd]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "LD BC, $ABCD".to_string())))
    }

    #[test]
    fn ld_bc_missing_byte() {
        let bus = GameboyBus::new(vec![0x01, 0x12]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x0, &prefs);

        assert!(result.is_err())
    }

    #[test]
    fn ld_bc_missing_word() {
        let bus = GameboyBus::new(vec![0x01]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x0, &prefs);

        assert!(result.is_err())
    }

    #[test]
    fn ld_bc_n16r_lower() {
        let bus = GameboyBus::new(vec![0x01, 0xab, 0xcd]);
        let prefs = Preferences{upcase: false, comma_space: true};
        let result = disassemble(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "ld bc, $abcd".to_string())))
    }

    #[test]
    fn ld_bc_n16r_no_space() {
        let bus = GameboyBus::new(vec![0x01, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: false};
        let result = disassemble(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "LD BC,$1234".to_string())))
    }

    #[test]
    fn ld_bc_n16r_lower_no_space() {
        let bus = GameboyBus::new(vec![0x01, 0x12, 0x34]);
        let prefs = Preferences{upcase: false, comma_space: false};
        let result = disassemble(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "ld bc,$1234".to_string())))
    }

    #[test]
    fn ld_bcr_a() {
        let bus = GameboyBus::new(vec![0x02]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [BC], A".to_string())))
    }

    #[test]
    fn inc_bc() {
        let bus = GameboyBus::new(vec![0x03]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC BC".to_string())))
    }

    #[test]
    fn inc_b() {
        let bus = GameboyBus::new(vec![0x04]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC B".to_string())))
    }

    #[test]
    fn dec_b() {
        let bus = GameboyBus::new(vec![0x05]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC B".to_string())))
    }

    #[test]
    fn ld_b_n8() {
        let bus = GameboyBus::new(vec![0x06, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD B, $2A".to_string())))
    }

    #[test]
    fn rlca() {
        let bus = GameboyBus::new(vec![0x07]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RLCA".to_string())))
    }

    #[test]
    fn ld_n16r_sp() {
        let bus = GameboyBus::new(vec![0x08, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "LD [$1234], SP".to_string())))
    }

    #[test]
    fn add_hl_bc() {
        let bus = GameboyBus::new(vec![0x09]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD HL, BC".to_string())))
    }

    #[test]
    fn ld_a_bcr() {
        let bus = GameboyBus::new(vec![0x0A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, [BC]".to_string())))
    }

    #[test]
    fn dec_bc() {
        let bus = GameboyBus::new(vec![0x0B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC BC".to_string())))
    }

    #[test]
    fn inc_c() {
        let bus = GameboyBus::new(vec![0x0C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC C".to_string())))
    }

    #[test]
    fn dec_c() {
        let bus = GameboyBus::new(vec![0x0D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC C".to_string())))
    }

    #[test]
    fn ld_c_n8() {
        let bus = GameboyBus::new(vec![0x0E, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD C, $2A".to_string())))
    }

    #[test]
    fn rrca() {
        let bus = GameboyBus::new(vec![0x0F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RRCA".to_string())))
    }

    // 0x1X
    #[test]
    fn stop() {
        let bus = GameboyBus::new(vec![0x10]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((1, "STOP".to_string())))
    }

    #[test]
    fn ld_de_n16() {
        let bus = GameboyBus::new(vec![0x11, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "LD DE, $1234".to_string())))
    }

    #[test]
    fn ld_der_a() {
        let bus = GameboyBus::new(vec![0x12]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [DE], A".to_string())))
    }

    #[test]
    fn inc_de() {
        let bus = GameboyBus::new(vec![0x13]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC DE".to_string())))
    }

    #[test]
    fn inc_d() {
        let bus = GameboyBus::new(vec![0x14]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC D".to_string())))
    }

    #[test]
    fn dec_d() {
        let bus = GameboyBus::new(vec![0x15]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC D".to_string())))
    }

    #[test]
    fn ld_d_n8() {
        let bus = GameboyBus::new(vec![0x16, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD D, $2A".to_string())))
    }

    #[test]
    fn rla() {
        let bus = GameboyBus::new(vec![0x17]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RLA".to_string())))
    }

    #[test]
    fn jr_i8() {
        let bus = GameboyBus::new(vec![0x18, 0xFC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "JR -4".to_string())))
    }

    #[test]
    fn add_hl_de() {
        let bus = GameboyBus::new(vec![0x19]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD HL, DE".to_string())))
    }

    #[test]
    fn ld_a_der() {
        let bus = GameboyBus::new(vec![0x1A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, [DE]".to_string())))
    }

    #[test]
    fn dec_de() {
        let bus = GameboyBus::new(vec![0x1B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC DE".to_string())))
    }

    #[test]
    fn inc_e() {
        let bus = GameboyBus::new(vec![0x1C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC E".to_string())))
    }

    #[test]
    fn dec_e() {
        let bus = GameboyBus::new(vec![0x1D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC E".to_string())))
    }

    #[test]
    fn ld_e_n8() {
        let bus = GameboyBus::new(vec![0x1E, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD E, $2A".to_string())))
    }

    #[test]
    fn rra() {
        let bus = GameboyBus::new(vec![0x1F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RRA".to_string())))
    }

    // 0x2X
    #[test]
    fn jr_nz_i8() {
        let bus = GameboyBus::new(vec![0x20, 0xFC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((2, "JR NZ, -4".to_string())))
    }

    #[test]
    fn ld_hl_n16() {
        let bus = GameboyBus::new(vec![0x21, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "LD HL, $1234".to_string())))
    }

    #[test]
    fn ld_hli_a() {
        let bus = GameboyBus::new(vec![0x22]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HLI], A".to_string())))
    }

    #[test]
    fn inc_hl() {
        let bus = GameboyBus::new(vec![0x23]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC HL".to_string())))
    }

    #[test]
    fn inc_h() {
        let bus = GameboyBus::new(vec![0x24]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC H".to_string())))
    }

    #[test]
    fn dec_h() {
        let bus = GameboyBus::new(vec![0x25]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC H".to_string())))
    }

    #[test]
    fn ld_h_n8() {
        let bus = GameboyBus::new(vec![0x26, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD H, $2A".to_string())))
    }

    #[test]
    fn daa() {
        let bus = GameboyBus::new(vec![0x27]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DAA".to_string())))
    }

    #[test]
    fn jr_z_i8() {
        let bus = GameboyBus::new(vec![0x28, 0xFC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "JR Z, -4".to_string())))
    }

    #[test]
    fn add_hl_hl() {
        let bus = GameboyBus::new(vec![0x29]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD HL, HL".to_string())))
    }

    #[test]
    fn ld_a_hli() {
        let bus = GameboyBus::new(vec![0x2A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, [HLI]".to_string())))
    }

    #[test]
    fn dec_hl() {
        let bus = GameboyBus::new(vec![0x2B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC HL".to_string())))
    }

    #[test]
    fn inc_l() {
        let bus = GameboyBus::new(vec![0x2C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC L".to_string())))
    }

    #[test]
    fn dec_l() {
        let bus = GameboyBus::new(vec![0x2D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC L".to_string())))
    }

    #[test]
    fn ld_l_n8() {
        let bus = GameboyBus::new(vec![0x2E, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD L, $2A".to_string())))
    }

    #[test]
    fn cpl() {
        let bus = GameboyBus::new(vec![0x2F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CPL".to_string())))
    }

    // 0x3X
    #[test]
    fn jr_nc_i8() {
        let bus = GameboyBus::new(vec![0x30, 0xFC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((2, "JR NC, -4".to_string())))
    }

    #[test]
    fn ld_sp_n16() {
        let bus = GameboyBus::new(vec![0x31, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "LD SP, $1234".to_string())))
    }

    #[test]
    fn ld_hld_a() {
        let bus = GameboyBus::new(vec![0x32]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HLD], A".to_string())))
    }

    #[test]
    fn inc_sp() {
        let bus = GameboyBus::new(vec![0x33]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC SP".to_string())))
    }

    #[test]
    fn inc_hlr() {
        let bus = GameboyBus::new(vec![0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC [HL]".to_string())))
    }

    #[test]
    fn dec_hlr() {
        let bus = GameboyBus::new(vec![0x35]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC [HL]".to_string())))
    }

    #[test]
    fn ld_hlr_n8() {
        let bus = GameboyBus::new(vec![0x36, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD [HL], $2A".to_string())))
    }

    #[test]
    fn scf() {
        let bus = GameboyBus::new(vec![0x37]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SCF".to_string())))
    }

    #[test]
    fn jr_c_i8() {
        let bus = GameboyBus::new(vec![0x38, 0xFC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "JR C, -4".to_string())))
    }

    #[test]
    fn add_hl_sp() {
        let bus = GameboyBus::new(vec![0x39]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD HL, SP".to_string())))
    }

    #[test]
    fn ld_a_hld() {
        let bus = GameboyBus::new(vec![0x3A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, [HLD]".to_string())))
    }

    #[test]
    fn dec_sp() {
        let bus = GameboyBus::new(vec![0x3B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC SP".to_string())))
    }

    #[test]
    fn inc_a() {
        let bus = GameboyBus::new(vec![0x3C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC A".to_string())))
    }

    #[test]
    fn dec_a() {
        let bus = GameboyBus::new(vec![0x3D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC A".to_string())))
    }

    #[test]
    fn ld_a_n8() {
        let bus = GameboyBus::new(vec![0x3E, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD A, $2A".to_string())))
    }

    #[test]
    fn ccf() {
        let bus = GameboyBus::new(vec![0x3F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CCF".to_string())))
    }

    // 0x4X
    #[test]
    fn ld_b_b() {
        let bus = GameboyBus::new(vec![0x40]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, B".to_string())))
    }

    #[test]
    fn ld_b_c() {
        let bus = GameboyBus::new(vec![0x41]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, C".to_string())))
    }

    #[test]
    fn ld_b_d() {
        let bus = GameboyBus::new(vec![0x42]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, D".to_string())))
    }

    #[test]
    fn ld_b_e() {
        let bus = GameboyBus::new(vec![0x43]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, E".to_string())))
    }

    #[test]
    fn ld_b_h() {
        let bus = GameboyBus::new(vec![0x44]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, H".to_string())))
    }

    #[test]
    fn ld_b_l() {
        let bus = GameboyBus::new(vec![0x45]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, L".to_string())))
    }

    #[test]
    fn ld_b_hlr() {
        let bus = GameboyBus::new(vec![0x46]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, [HL]".to_string())))
    }

    #[test]
    fn ld_b_a() {
        let bus = GameboyBus::new(vec![0x47]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, A".to_string())))
    }

    #[test]
    fn ld_c_b() {
        let bus = GameboyBus::new(vec![0x48]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, B".to_string())))
    }

    #[test]
    fn ld_c_c() {
        let bus = GameboyBus::new(vec![0x49]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, C".to_string())))
    }

    #[test]
    fn ld_c_d() {
        let bus = GameboyBus::new(vec![0x4a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, D".to_string())))
    }

    #[test]
    fn ld_c_e() {
        let bus = GameboyBus::new(vec![0x4b]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, E".to_string())))
    }

    #[test]
    fn ld_c_h() {
        let bus = GameboyBus::new(vec![0x4c]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, H".to_string())))
    }

    #[test]
    fn ld_c_l() {
        let bus = GameboyBus::new(vec![0x4d]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, L".to_string())))
    }

    #[test]
    fn ld_c_hlr() {
        let bus = GameboyBus::new(vec![0x4e]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, [HL]".to_string())))
    }

    #[test]
    fn ld_c_a() {
        let bus = GameboyBus::new(vec![0x4f]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, A".to_string())))
    }

    // 0x5X
    #[test]
    fn ld_d_b() {
        let bus = GameboyBus::new(vec![0x50]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, B".to_string())))
    }

    #[test]
    fn ld_d_c() {
        let bus = GameboyBus::new(vec![0x51]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, C".to_string())))
    }

    #[test]
    fn ld_d_d() {
        let bus = GameboyBus::new(vec![0x52]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, D".to_string())))
    }

    #[test]
    fn ld_d_e() {
        let bus = GameboyBus::new(vec![0x53]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, E".to_string())))
    }

    #[test]
    fn ld_d_h() {
        let bus = GameboyBus::new(vec![0x54]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, H".to_string())))
    }

    #[test]
    fn ld_d_l() {
        let bus = GameboyBus::new(vec![0x55]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, L".to_string())))
    }

    #[test]
    fn ld_d_hlr() {
        let bus = GameboyBus::new(vec![0x56]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, [HL]".to_string())))
    }

    #[test]
    fn ld_d_a() {
        let bus = GameboyBus::new(vec![0x57]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, A".to_string())))
    }

    #[test]
    fn ld_e_b() {
        let bus = GameboyBus::new(vec![0x58]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, B".to_string())))
    }

    #[test]
    fn ld_e_c() {
        let bus = GameboyBus::new(vec![0x59]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, C".to_string())))
    }

    #[test]
    fn ld_e_d() {
        let bus = GameboyBus::new(vec![0x5a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, D".to_string())))
    }

    #[test]
    fn ld_e_e() {
        let bus = GameboyBus::new(vec![0x5b]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, E".to_string())))
    }

    #[test]
    fn ld_e_h() {
        let bus = GameboyBus::new(vec![0x5c]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, H".to_string())))
    }

    #[test]
    fn ld_e_l() {
        let bus = GameboyBus::new(vec![0x5d]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, L".to_string())))
    }

    #[test]
    fn ld_e_hlr() {
        let bus = GameboyBus::new(vec![0x5e]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, [HL]".to_string())))
    }

    #[test]
    fn ld_e_a() {
        let bus = GameboyBus::new(vec![0x5f]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, A".to_string())))
    }

    // 0x6X
    #[test]
    fn ld_h_b() {
        let bus = GameboyBus::new(vec![0x60]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, B".to_string())))
    }

    #[test]
    fn ld_h_c() {
        let bus = GameboyBus::new(vec![0x61]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, C".to_string())))
    }

    #[test]
    fn ld_h_d() {
        let bus = GameboyBus::new(vec![0x62]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, D".to_string())))
    }

    #[test]
    fn ld_h_e() {
        let bus = GameboyBus::new(vec![0x63]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, E".to_string())))
    }

    #[test]
    fn ld_h_h() {
        let bus = GameboyBus::new(vec![0x64]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, H".to_string())))
    }

    #[test]
    fn ld_h_l() {
        let bus = GameboyBus::new(vec![0x65]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, L".to_string())))
    }

    #[test]
    fn ld_h_hlr() {
        let bus = GameboyBus::new(vec![0x66]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, [HL]".to_string())))
    }

    #[test]
    fn ld_h_a() {
        let bus = GameboyBus::new(vec![0x67]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, A".to_string())))
    }

    #[test]
    fn ld_l_b() {
        let bus = GameboyBus::new(vec![0x68]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, B".to_string())))
    }

    #[test]
    fn ld_l_c() {
        let bus = GameboyBus::new(vec![0x69]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, C".to_string())))
    }

    #[test]
    fn ld_l_d() {
        let bus = GameboyBus::new(vec![0x6a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, D".to_string())))
    }

    #[test]
    fn ld_l_e() {
        let bus = GameboyBus::new(vec![0x6b]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, E".to_string())))
    }

    #[test]
    fn ld_l_h() {
        let bus = GameboyBus::new(vec![0x6c]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, H".to_string())))
    }

    #[test]
    fn ld_l_l() {
        let bus = GameboyBus::new(vec![0x6d]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, L".to_string())))
    }

    #[test]
    fn ld_l_hlr() {
        let bus = GameboyBus::new(vec![0x6e]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, [HL]".to_string())))
    }

    #[test]
    fn ld_l_a() {
        let bus = GameboyBus::new(vec![0x6f]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, A".to_string())))
    }

    // 0x7X
    #[test]
    fn ld_hlr_b() {
        let bus = GameboyBus::new(vec![0x70]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], B".to_string())))
    }

    #[test]
    fn ld_hlr_c() {
        let bus = GameboyBus::new(vec![0x71]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], C".to_string())))
    }

    #[test]
    fn ld_hlr_d() {
        let bus = GameboyBus::new(vec![0x72]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], D".to_string())))
    }

    #[test]
    fn ld_hlr_e() {
        let bus = GameboyBus::new(vec![0x73]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], E".to_string())))
    }

    #[test]
    fn ld_hlr_h() {
        let bus = GameboyBus::new(vec![0x74]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], H".to_string())))
    }

    #[test]
    fn ld_hlr_l() {
        let bus = GameboyBus::new(vec![0x75]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], L".to_string())))
    }

    #[test]
    fn halt() {
        let bus = GameboyBus::new(vec![0x76]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "HALT".to_string())))
    }

    #[test]
    fn ld_hlr_a() {
        let bus = GameboyBus::new(vec![0x77]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], A".to_string())))
    }

    #[test]
    fn ld_a_b() {
        let bus = GameboyBus::new(vec![0x78]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, B".to_string())))
    }

    #[test]
    fn ld_a_c() {
        let bus = GameboyBus::new(vec![0x79]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, C".to_string())))
    }

    #[test]
    fn ld_a_d() {
        let bus = GameboyBus::new(vec![0x7a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, D".to_string())))
    }

    #[test]
    fn ld_a_e() {
        let bus = GameboyBus::new(vec![0x7b]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, E".to_string())))
    }

    #[test]
    fn ld_a_h() {
        let bus = GameboyBus::new(vec![0x7c]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, H".to_string())))
    }

    #[test]
    fn ld_a_l() {
        let bus = GameboyBus::new(vec![0x7d]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, L".to_string())))
    }

    #[test]
    fn ld_a_hlr() {
        let bus = GameboyBus::new(vec![0x7e]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, [HL]".to_string())))
    }

    #[test]
    fn ld_a_a() {
        let bus = GameboyBus::new(vec![0x7f]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, A".to_string())))
    }

    // 0x8X
    #[test]
    fn add_a_b() {
        let bus = GameboyBus::new(vec![0x80]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, B".to_string())))
    }

    #[test]
    fn add_a_c() {
        let bus = GameboyBus::new(vec![0x81]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, C".to_string())))
    }

    #[test]
    fn add_a_d() {
        let bus = GameboyBus::new(vec![0x82]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, D".to_string())))
    }

    #[test]
    fn add_a_e() {
        let bus = GameboyBus::new(vec![0x83]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, E".to_string())))
    }

    #[test]
    fn add_a_h() {
        let bus = GameboyBus::new(vec![0x84]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, H".to_string())))
    }

    #[test]
    fn add_a_l() {
        let bus = GameboyBus::new(vec![0x85]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, L".to_string())))
    }

    #[test]
    fn add_a_hlr() {
        let bus = GameboyBus::new(vec![0x86]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, [HL]".to_string())))
    }

    #[test]
    fn add_a_a() {
        let bus = GameboyBus::new(vec![0x87]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, A".to_string())))
    }

    #[test]
    fn adc_a_b() {
        let bus = GameboyBus::new(vec![0x88]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, B".to_string())))
    }

    #[test]
    fn adc_a_c() {
        let bus = GameboyBus::new(vec![0x89]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, C".to_string())))
    }

    #[test]
    fn adc_a_d() {
        let bus = GameboyBus::new(vec![0x8a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, D".to_string())))
    }

    #[test]
    fn adc_a_e() {
        let bus = GameboyBus::new(vec![0x8b]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, E".to_string())))
    }

    #[test]
    fn adc_a_h() {
        let bus = GameboyBus::new(vec![0x8c]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, H".to_string())))
    }

    #[test]
    fn adc_a_l() {
        let bus = GameboyBus::new(vec![0x8d]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, L".to_string())))
    }

    #[test]
    fn adc_a_hlr() {
        let bus = GameboyBus::new(vec![0x8e]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, [HL]".to_string())))
    }

    #[test]
    fn adc_a_a() {
        let bus = GameboyBus::new(vec![0x8f]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, A".to_string())))
    }

    // 0x9X
    #[test]
    fn sub_a_b() {
        let bus = GameboyBus::new(vec![0x90]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, B".to_string())))
    }

    #[test]
    fn sub_a_c() {
        let bus = GameboyBus::new(vec![0x91]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, C".to_string())))
    }

    #[test]
    fn sub_a_d() {
        let bus = GameboyBus::new(vec![0x92]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, D".to_string())))
    }

    #[test]
    fn sub_a_e() {
        let bus = GameboyBus::new(vec![0x93]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, E".to_string())))
    }

    #[test]
    fn sub_a_h() {
        let bus = GameboyBus::new(vec![0x94]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, H".to_string())))
    }

    #[test]
    fn sub_a_l() {
        let bus = GameboyBus::new(vec![0x95]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, L".to_string())))
    }

    #[test]
    fn sub_a_hlr() {
        let bus = GameboyBus::new(vec![0x96]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, [HL]".to_string())))
    }

    #[test]
    fn sub_a_a() {
        let bus = GameboyBus::new(vec![0x97]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, A".to_string())))
    }

    #[test]
    fn sbc_a_b() {
        let bus = GameboyBus::new(vec![0x98]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, B".to_string())))
    }

    #[test]
    fn sbc_a_c() {
        let bus = GameboyBus::new(vec![0x99]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, C".to_string())))
    }

    #[test]
    fn sbc_a_d() {
        let bus = GameboyBus::new(vec![0x9a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, D".to_string())))
    }

    #[test]
    fn sbc_a_e() {
        let bus = GameboyBus::new(vec![0x9b]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, E".to_string())))
    }

    #[test]
    fn sbc_a_h() {
        let bus = GameboyBus::new(vec![0x9c]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, H".to_string())))
    }

    #[test]
    fn sbc_a_l() {
        let bus = GameboyBus::new(vec![0x9d]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, L".to_string())))
    }

    #[test]
    fn sbc_a_hlr() {
        let bus = GameboyBus::new(vec![0x9e]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, [HL]".to_string())))
    }

    #[test]
    fn sbc_a_a() {
        let bus = GameboyBus::new(vec![0x9f]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, A".to_string())))
    }

    // 0xAX
    #[test]
    fn and_a_b() {
        let bus = GameboyBus::new(vec![0xA0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, B".to_string())))
    }

    #[test]
    fn and_a_c() {
        let bus = GameboyBus::new(vec![0xA1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, C".to_string())))
    }

    #[test]
    fn and_a_d() {
        let bus = GameboyBus::new(vec![0xA2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, D".to_string())))
    }

    #[test]
    fn and_a_e() {
        let bus = GameboyBus::new(vec![0xA3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, E".to_string())))
    }

    #[test]
    fn and_a_h() {
        let bus = GameboyBus::new(vec![0xA4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, H".to_string())))
    }

    #[test]
    fn and_a_l() {
        let bus = GameboyBus::new(vec![0xA5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, L".to_string())))
    }

    #[test]
    fn and_a_hlr() {
        let bus = GameboyBus::new(vec![0xA6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, [HL]".to_string())))
    }

    #[test]
    fn and_a_a() {
        let bus = GameboyBus::new(vec![0xA7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, A".to_string())))
    }

    #[test]
    fn xor_a_b() {
        let bus = GameboyBus::new(vec![0xA8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, B".to_string())))
    }

    #[test]
    fn xor_a_c() {
        let bus = GameboyBus::new(vec![0xA9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, C".to_string())))
    }

    #[test]
    fn xor_a_d() {
        let bus = GameboyBus::new(vec![0xAa]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, D".to_string())))
    }

    #[test]
    fn xor_a_e() {
        let bus = GameboyBus::new(vec![0xAb]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, E".to_string())))
    }

    #[test]
    fn xor_a_h() {
        let bus = GameboyBus::new(vec![0xAc]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, H".to_string())))
    }

    #[test]
    fn xor_a_l() {
        let bus = GameboyBus::new(vec![0xAd]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, L".to_string())))
    }

    #[test]
    fn xor_a_hlr() {
        let bus = GameboyBus::new(vec![0xAe]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, [HL]".to_string())))
    }

    #[test]
    fn xor_a_a() {
        let bus = GameboyBus::new(vec![0xAf]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, A".to_string())))
    }

    // 0xBX
    #[test]
    fn or_a_b() {
        let bus = GameboyBus::new(vec![0xB0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, B".to_string())))
    }

    #[test]
    fn or_a_c() {
        let bus = GameboyBus::new(vec![0xB1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, C".to_string())))
    }

    #[test]
    fn or_a_d() {
        let bus = GameboyBus::new(vec![0xB2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, D".to_string())))
    }

    #[test]
    fn or_a_e() {
        let bus = GameboyBus::new(vec![0xB3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, E".to_string())))
    }

    #[test]
    fn or_a_h() {
        let bus = GameboyBus::new(vec![0xB4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, H".to_string())))
    }

    #[test]
    fn or_a_l() {
        let bus = GameboyBus::new(vec![0xB5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, L".to_string())))
    }

    #[test]
    fn or_a_hlr() {
        let bus = GameboyBus::new(vec![0xB6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, [HL]".to_string())))
    }

    #[test]
    fn or_a_a() {
        let bus = GameboyBus::new(vec![0xB7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, A".to_string())))
    }

    #[test]
    fn cp_a_b() {
        let bus = GameboyBus::new(vec![0xB8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, B".to_string())))
    }

    #[test]
    fn cp_a_c() {
        let bus = GameboyBus::new(vec![0xB9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, C".to_string())))
    }

    #[test]
    fn cp_a_d() {
        let bus = GameboyBus::new(vec![0xBa]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, D".to_string())))
    }

    #[test]
    fn cp_a_e() {
        let bus = GameboyBus::new(vec![0xBb]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, E".to_string())))
    }

    #[test]
    fn cp_a_h() {
        let bus = GameboyBus::new(vec![0xBc]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, H".to_string())))
    }

    #[test]
    fn cp_a_l() {
        let bus = GameboyBus::new(vec![0xBd]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, L".to_string())))
    }

    #[test]
    fn cp_a_hlr() {
        let bus = GameboyBus::new(vec![0xBe]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, [HL]".to_string())))
    }

    #[test]
    fn cp_a_a() {
        let bus = GameboyBus::new(vec![0xBf]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, A".to_string())))
    }

    // 0xCX
    #[test]
    fn ret_nz() {
        let bus = GameboyBus::new(vec![0xC0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RET NZ".to_string())))
    }

    #[test]
    fn pop_bc() {
        let bus = GameboyBus::new(vec![0xC1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "POP BC".to_string())))
    }

    #[test]
    fn jp_nz_n16() {
        let bus = GameboyBus::new(vec![0xC2, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "JP NZ, $1234".to_string())))
    }

    #[test]
    fn jp_n16() {
        let bus = GameboyBus::new(vec![0xC3, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "JP $1234".to_string())))
    }

    #[test]
    fn call_nz_n16() {
        let bus = GameboyBus::new(vec![0xC4, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "CALL NZ, $1234".to_string())))
    }

    #[test]
    fn push_bc() {
        let bus = GameboyBus::new(vec![0xC5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "PUSH BC".to_string())))
    }

    #[test]
    fn add_a_n8() {
        let bus = GameboyBus::new(vec![0xC6, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "ADD A, $2A".to_string())))
    }

    #[test]
    fn rst_00() {
        let bus = GameboyBus::new(vec![0xC7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $00".to_string())))
    }

    #[test]
    fn ret_z() {
        let bus = GameboyBus::new(vec![0xC8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RET Z".to_string())))
    }

    #[test]
    fn ret() {
        let bus = GameboyBus::new(vec![0xC9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RET".to_string())))
    }

    #[test]
    fn jp_z_n16() {
        let bus = GameboyBus::new(vec![0xCA, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "JP Z, $1234".to_string())))
    }

    #[test]
    fn call_z_n16() {
        let bus = GameboyBus::new(vec![0xCC, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "CALL Z, $1234".to_string())))
    }

    #[test]
    fn call_n16() {
        let bus = GameboyBus::new(vec![0xCD, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "CALL $1234".to_string())))
    }

    #[test]
    fn adc_a_n8() {
        let bus = GameboyBus::new(vec![0xCE, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "ADC A, $2A".to_string())))
    }

    #[test]
    fn rst_08() {
        let bus = GameboyBus::new(vec![0xCF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $08".to_string())))
    }

    // 0xDX
    #[test]
    fn ret_nc() {
        let bus = GameboyBus::new(vec![0xD0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RET NC".to_string())))
    }

    #[test]
    fn pop_de() {
        let bus = GameboyBus::new(vec![0xD1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "POP DE".to_string())))
    }

    #[test]
    fn jp_nc_n16() {
        let bus = GameboyBus::new(vec![0xD2, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "JP NC, $1234".to_string())))
    }

    #[test]
    fn call_nc_n16() {
        let bus = GameboyBus::new(vec![0xD4, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "CALL NC, $1234".to_string())))
    }

    #[test]
    fn push_de() {
        let bus = GameboyBus::new(vec![0xD5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "PUSH DE".to_string())))
    }

    #[test]
    fn sub_a_n8() {
        let bus = GameboyBus::new(vec![0xD6, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SUB A, $2A".to_string())))
    }

    #[test]
    fn rst_10() {
        let bus = GameboyBus::new(vec![0xD7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $10".to_string())))
    }

    #[test]
    fn ret_c() {
        let bus = GameboyBus::new(vec![0xD8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RET C".to_string())))
    }

    #[test]
    fn reti() {
        let bus = GameboyBus::new(vec![0xD9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RETI".to_string())))
    }

    #[test]
    fn jp_c_n16() {
        let bus = GameboyBus::new(vec![0xDA, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "JP C, $1234".to_string())))
    }

    #[test]
    fn call_c_n16() {
        let bus = GameboyBus::new(vec![0xDC, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "CALL C, $1234".to_string())))
    }

    #[test]
    fn sbc_a_n8() {
        let bus = GameboyBus::new(vec![0xDE, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SBC A, $2A".to_string())))
    }

    #[test]
    fn rst_18() {
        let bus = GameboyBus::new(vec![0xDF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $18".to_string())))
    }

    // 0xEX
    #[test]
    fn ldh_n8_a() {
        let bus = GameboyBus::new(vec![0xE0, 0xDE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LDH [$FFDE], A".to_string())))
    }

    #[test]
    fn pop_hl() {
        let bus = GameboyBus::new(vec![0xE1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "POP HL".to_string())))
    }

    #[test]
    fn ldh_cr_a() {
        let bus = GameboyBus::new(vec![0xE2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LDH [C], A".to_string())))
    }

    #[test]
    fn push_hl() {
        let bus = GameboyBus::new(vec![0xE5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "PUSH HL".to_string())))
    }

    #[test]
    fn and_a_n8() {
        let bus = GameboyBus::new(vec![0xE6, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "AND A, $2A".to_string())))
    }

    #[test]
    fn rst_20() {
        let bus = GameboyBus::new(vec![0xE7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $20".to_string())))
    }

    #[test]
    fn add_sp_i8_neg() {
        let bus = GameboyBus::new(vec![0xE8, 0xFD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "ADD SP, -3".to_string())))
    }

    #[test]
    fn add_sp_i8_pos() {
        let bus = GameboyBus::new(vec![0xE8, 0x02]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "ADD SP, 2".to_string())))
    }

    #[test]
    fn jp_hl() {
        let bus = GameboyBus::new(vec![0xE9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "JP HL".to_string())))
    }

    #[test]
    fn ld_n16r_a() {
        let bus = GameboyBus::new(vec![0xEA, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "LD [$1234], A".to_string())))
    }

    #[test]
    fn xor_a_n8() {
        let bus = GameboyBus::new(vec![0xEE, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "XOR A, $2A".to_string())))
    }

    #[test]
    fn rst_28() {
        let bus = GameboyBus::new(vec![0xEF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $28".to_string())))
    }

    // 0xFX
    #[test]
    fn ldh_a_n8() {
        let bus = GameboyBus::new(vec![0xF0, 0xDE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LDH A, [$FFDE]".to_string())))
    }

    #[test]
    fn pop_af() {
        let bus = GameboyBus::new(vec![0xF1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "POP AF".to_string())))
    }

    #[test]
    fn ldh_a_cr() {
        let bus = GameboyBus::new(vec![0xF2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LDH A, [C]".to_string())))
    }

    #[test]
    fn di() {
        let bus = GameboyBus::new(vec![0xF3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DI".to_string())))
    }

    #[test]
    fn push_af() {
        let bus = GameboyBus::new(vec![0xF5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "PUSH AF".to_string())))
    }

    #[test]
    fn or_a_n8() {
        let bus = GameboyBus::new(vec![0xF6, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "OR A, $2A".to_string())))
    }

    #[test]
    fn rst_30() {
        let bus = GameboyBus::new(vec![0xF7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $30".to_string())))
    }

    #[test]
    fn ld_sp_i8_neg() {
        let bus = GameboyBus::new(vec![0xF8, 0xFD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD HL, SP-3".to_string())))
    }

    #[test]
    fn ld_sp_i8_pos() {
        let bus = GameboyBus::new(vec![0xF8, 0x02]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD HL, SP+2".to_string())))
    }

    #[test]
    fn ld_sp_hl() {
        let bus = GameboyBus::new(vec![0xF9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD SP, HL".to_string())))
    }

    #[test]
    fn ld_a_n16r() {
        let bus = GameboyBus::new(vec![0xFA, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "LD A, [$1234]".to_string())))
    }

    #[test]
    fn ei() {
        let bus = GameboyBus::new(vec![0xFB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "EI".to_string())))
    }

    #[test]
    fn cp_a_n8() {
        let bus = GameboyBus::new(vec![0xFE, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "CP A, $2A".to_string())))
    }

    #[test]
    fn rst_38() {
        let bus = GameboyBus::new(vec![0xFF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $38".to_string())))
    }

    // 0xCB-prefix
    // 0x0X
    #[test]
    fn rlc_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x00]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC B".to_string())))
    }

    #[test]
    fn rlc_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x01]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC C".to_string())))
    }

    #[test]
    fn rlc_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x02]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC D".to_string())))
    }

    #[test]
    fn rlc_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x03]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC E".to_string())))
    }

    #[test]
    fn rlc_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x04]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC H".to_string())))
    }

    #[test]
    fn rlc_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x05]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC L".to_string())))
    }

    #[test]
    fn rlc_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x06]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC [HL]".to_string())))
    }

    #[test]
    fn rlc_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x07]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC A".to_string())))
    }

    #[test]
    fn rrc_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x08]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC B".to_string())))
    }

    #[test]
    fn rrc_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x09]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC C".to_string())))
    }

    #[test]
    fn rrc_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x0A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC D".to_string())))
    }

    #[test]
    fn rrc_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x0B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC E".to_string())))
    }

    #[test]
    fn rrc_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x0C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC H".to_string())))
    }

    #[test]
    fn rrc_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x0D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC L".to_string())))
    }

    #[test]
    fn rrc_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x0E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC [HL]".to_string())))
    }

    #[test]
    fn rrc_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x0F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC A".to_string())))
    }

    // 0x1X
    #[test]
    fn rl_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x10]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL B".to_string())))
    }

    #[test]
    fn rl_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x11]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL C".to_string())))
    }

    #[test]
    fn rl_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x12]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL D".to_string())))
    }

    #[test]
    fn rl_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x13]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL E".to_string())))
    }

    #[test]
    fn rl_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x14]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL H".to_string())))
    }

    #[test]
    fn rl_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x15]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL L".to_string())))
    }

    #[test]
    fn rl_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x16]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL [HL]".to_string())))
    }

    #[test]
    fn rl_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x17]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL A".to_string())))
    }

    #[test]
    fn rr_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x18]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR B".to_string())))
    }

    #[test]
    fn rr_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x19]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR C".to_string())))
    }

    #[test]
    fn rr_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x1A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR D".to_string())))
    }

    #[test]
    fn rr_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x1B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR E".to_string())))
    }

    #[test]
    fn rr_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x1C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR H".to_string())))
    }

    #[test]
    fn rr_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x1D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR L".to_string())))
    }

    #[test]
    fn rr_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x1E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR [HL]".to_string())))
    }

    #[test]
    fn rr_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x1F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR A".to_string())))
    }

    // 0x2X
    #[test]
    fn sla_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x20]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA B".to_string())))
    }

    #[test]
    fn sla_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x21]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA C".to_string())))
    }

    #[test]
    fn sla_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x22]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA D".to_string())))
    }

    #[test]
    fn sla_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x23]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA E".to_string())))
    }

    #[test]
    fn sla_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x24]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA H".to_string())))
    }

    #[test]
    fn sla_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x25]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA L".to_string())))
    }

    #[test]
    fn sla_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x26]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA [HL]".to_string())))
    }

    #[test]
    fn sla_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x27]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA A".to_string())))
    }

    #[test]
    fn sra_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x28]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA B".to_string())))
    }

    #[test]
    fn sra_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x29]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA C".to_string())))
    }

    #[test]
    fn sra_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x2A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA D".to_string())))
    }

    #[test]
    fn sra_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x2B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA E".to_string())))
    }

    #[test]
    fn sra_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x2C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA H".to_string())))
    }

    #[test]
    fn sra_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x2D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA L".to_string())))
    }

    #[test]
    fn sra_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x2E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA [HL]".to_string())))
    }

    #[test]
    fn sra_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x2F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA A".to_string())))
    }

    // 0x3X
    #[test]
    fn swap_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x30]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP B".to_string())))
    }

    #[test]
    fn swap_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x31]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP C".to_string())))
    }

    #[test]
    fn swap_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x32]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP D".to_string())))
    }

    #[test]
    fn swap_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x33]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP E".to_string())))
    }

    #[test]
    fn swap_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP H".to_string())))
    }

    #[test]
    fn swap_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x35]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP L".to_string())))
    }

    #[test]
    fn swap_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x36]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP [HL]".to_string())))
    }

    #[test]
    fn swap_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x37]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP A".to_string())))
    }

    #[test]
    fn srl_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x38]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL B".to_string())))
    }

    #[test]
    fn srl_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x39]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL C".to_string())))
    }

    #[test]
    fn srl_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x3A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL D".to_string())))
    }

    #[test]
    fn srl_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x3B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL E".to_string())))
    }

    #[test]
    fn srl_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x3C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL H".to_string())))
    }

    #[test]
    fn srl_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x3D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL L".to_string())))
    }

    #[test]
    fn srl_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x3E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL [HL]".to_string())))
    }

    #[test]
    fn srl_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x3F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL A".to_string())))
    }

    // 0x4X
    #[test]
    fn bit_0_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x40]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, B".to_string())))
    }

    #[test]
    fn bit_0_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x41]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, C".to_string())))
    }

    #[test]
    fn bit_0_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x42]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, D".to_string())))
    }

    #[test]
    fn bit_0_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x43]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, E".to_string())))
    }

    #[test]
    fn bit_0_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x44]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, H".to_string())))
    }

    #[test]
    fn bit_0_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x45]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, L".to_string())))
    }

    #[test]
    fn bit_0_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x46]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, [HL]".to_string())))
    }

    #[test]
    fn bit_0_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x47]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, A".to_string())))
    }

    #[test]
    fn bit_1_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x48]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, B".to_string())))
    }

    #[test]
    fn bit_1_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x49]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, C".to_string())))
    }

    #[test]
    fn bit_1_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x4A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, D".to_string())))
    }

    #[test]
    fn bit_1_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x4B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, E".to_string())))
    }

    #[test]
    fn bit_1_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x4C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, H".to_string())))
    }

    #[test]
    fn bit_1_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x4D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, L".to_string())))
    }

    #[test]
    fn bit_1_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x4E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, [HL]".to_string())))
    }

    #[test]
    fn bit_1_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x4F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, A".to_string())))
    }

    // 0x5X
    #[test]
    fn bit_2_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x50]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, B".to_string())))
    }

    #[test]
    fn bit_2_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x51]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, C".to_string())))
    }

    #[test]
    fn bit_2_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x52]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, D".to_string())))
    }

    #[test]
    fn bit_2_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x53]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, E".to_string())))
    }

    #[test]
    fn bit_2_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x54]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, H".to_string())))
    }

    #[test]
    fn bit_2_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x55]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, L".to_string())))
    }

    #[test]
    fn bit_2_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x56]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, [HL]".to_string())))
    }

    #[test]
    fn bit_2_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x57]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, A".to_string())))
    }

    #[test]
    fn bit_3_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x58]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, B".to_string())))
    }

    #[test]
    fn bit_3_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x59]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, C".to_string())))
    }

    #[test]
    fn bit_3_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x5A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, D".to_string())))
    }

    #[test]
    fn bit_3_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x5B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, E".to_string())))
    }

    #[test]
    fn bit_3_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x5C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, H".to_string())))
    }

    #[test]
    fn bit_3_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x5D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, L".to_string())))
    }

    #[test]
    fn bit_3_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x5E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, [HL]".to_string())))
    }

    #[test]
    fn bit_3_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x5F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, A".to_string())))
    }

    // 0x6X
    #[test]
    fn bit_4_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x60]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, B".to_string())))
    }

    #[test]
    fn bit_4_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x61]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, C".to_string())))
    }

    #[test]
    fn bit_4_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x62]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, D".to_string())))
    }

    #[test]
    fn bit_4_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x63]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, E".to_string())))
    }

    #[test]
    fn bit_4_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x64]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, H".to_string())))
    }

    #[test]
    fn bit_4_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x65]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, L".to_string())))
    }

    #[test]
    fn bit_4_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x66]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, [HL]".to_string())))
    }

    #[test]
    fn bit_4_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x67]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, A".to_string())))
    }

    #[test]
    fn bit_5_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x68]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, B".to_string())))
    }

    #[test]
    fn bit_5_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x69]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, C".to_string())))
    }

    #[test]
    fn bit_5_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x6A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, D".to_string())))
    }

    #[test]
    fn bit_5_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x6B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, E".to_string())))
    }

    #[test]
    fn bit_5_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x6C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, H".to_string())))
    }

    #[test]
    fn bit_5_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x6D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, L".to_string())))
    }

    #[test]
    fn bit_5_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x6E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, [HL]".to_string())))
    }

    #[test]
    fn bit_5_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x6F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, A".to_string())))
    }

    // 0x7X
    #[test]
    fn bit_6_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x70]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, B".to_string())))
    }

    #[test]
    fn bit_6_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x71]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, C".to_string())))
    }

    #[test]
    fn bit_6_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x72]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, D".to_string())))
    }

    #[test]
    fn bit_6_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x73]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, E".to_string())))
    }

    #[test]
    fn bit_6_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x74]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, H".to_string())))
    }

    #[test]
    fn bit_6_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x75]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, L".to_string())))
    }

    #[test]
    fn bit_6_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x76]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, [HL]".to_string())))
    }

    #[test]
    fn bit_6_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x77]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, A".to_string())))
    }

    #[test]
    fn bit_7_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x78]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, B".to_string())))
    }

    #[test]
    fn bit_7_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x79]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, C".to_string())))
    }

    #[test]
    fn bit_7_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x7A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, D".to_string())))
    }

    #[test]
    fn bit_7_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x7B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, E".to_string())))
    }

    #[test]
    fn bit_7_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x7C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, H".to_string())))
    }

    #[test]
    fn bit_7_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x7D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, L".to_string())))
    }

    #[test]
    fn bit_7_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x7E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, [HL]".to_string())))
    }

    #[test]
    fn bit_7_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x7F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, A".to_string())))
    }

    // 0x8X
    #[test]
    fn res_0_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x80]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, B".to_string())))
    }

    #[test]
    fn res_0_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x81]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, C".to_string())))
    }

    #[test]
    fn res_0_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x82]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, D".to_string())))
    }

    #[test]
    fn res_0_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x83]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, E".to_string())))
    }

    #[test]
    fn res_0_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x84]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, H".to_string())))
    }

    #[test]
    fn res_0_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x85]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, L".to_string())))
    }

    #[test]
    fn res_0_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x86]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, [HL]".to_string())))
    }

    #[test]
    fn res_0_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x87]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, A".to_string())))
    }

    #[test]
    fn res_1_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x88]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, B".to_string())))
    }

    #[test]
    fn res_1_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x89]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, C".to_string())))
    }

    #[test]
    fn res_1_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x8A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, D".to_string())))
    }

    #[test]
    fn res_1_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x8B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, E".to_string())))
    }

    #[test]
    fn res_1_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x8C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, H".to_string())))
    }

    #[test]
    fn res_1_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x8D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, L".to_string())))
    }

    #[test]
    fn res_1_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x8E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, [HL]".to_string())))
    }

    #[test]
    fn res_1_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x8F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, A".to_string())))
    }

    // 0x9X
    #[test]
    fn res_2_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x90]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, B".to_string())))
    }

    #[test]
    fn res_2_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x91]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, C".to_string())))
    }

    #[test]
    fn res_2_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x92]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, D".to_string())))
    }

    #[test]
    fn res_2_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x93]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, E".to_string())))
    }

    #[test]
    fn res_2_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x94]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, H".to_string())))
    }

    #[test]
    fn res_2_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x95]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, L".to_string())))
    }

    #[test]
    fn res_2_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x96]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, [HL]".to_string())))
    }

    #[test]
    fn res_2_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x97]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, A".to_string())))
    }

    #[test]
    fn res_3_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x98]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, B".to_string())))
    }

    #[test]
    fn res_3_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x99]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, C".to_string())))
    }

    #[test]
    fn res_3_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x9A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, D".to_string())))
    }

    #[test]
    fn res_3_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x9B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, E".to_string())))
    }

    #[test]
    fn res_3_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x9C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, H".to_string())))
    }

    #[test]
    fn res_3_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x9D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, L".to_string())))
    }

    #[test]
    fn res_3_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x9E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, [HL]".to_string())))
    }

    #[test]
    fn res_3_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x9F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, A".to_string())))
    }

    // 0xAX
    #[test]
    fn res_4_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xA0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, B".to_string())))
    }

    #[test]
    fn res_4_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xA1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, C".to_string())))
    }

    #[test]
    fn res_4_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xA2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, D".to_string())))
    }

    #[test]
    fn res_4_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xA3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, E".to_string())))
    }

    #[test]
    fn res_4_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xA4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, H".to_string())))
    }

    #[test]
    fn res_4_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xA5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, L".to_string())))
    }

    #[test]
    fn res_4_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xA6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, [HL]".to_string())))
    }

    #[test]
    fn res_4_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xA7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, A".to_string())))
    }

    #[test]
    fn res_5_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xA8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, B".to_string())))
    }

    #[test]
    fn res_5_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xA9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, C".to_string())))
    }

    #[test]
    fn res_5_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xAA]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, D".to_string())))
    }

    #[test]
    fn res_5_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xAB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, E".to_string())))
    }

    #[test]
    fn res_5_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xAC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, H".to_string())))
    }

    #[test]
    fn res_5_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xAD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, L".to_string())))
    }

    #[test]
    fn res_5_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xAE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, [HL]".to_string())))
    }

    #[test]
    fn res_5_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xAF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, A".to_string())))
    }

    // 0xBX
    #[test]
    fn res_6_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xB0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, B".to_string())))
    }

    #[test]
    fn res_6_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xB1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, C".to_string())))
    }

    #[test]
    fn res_6_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xB2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, D".to_string())))
    }

    #[test]
    fn res_6_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xB3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, E".to_string())))
    }

    #[test]
    fn res_6_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xB4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, H".to_string())))
    }

    #[test]
    fn res_6_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xB5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, L".to_string())))
    }

    #[test]
    fn res_6_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xB6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, [HL]".to_string())))
    }

    #[test]
    fn res_6_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xB7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, A".to_string())))
    }

    #[test]
    fn res_7_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xB8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, B".to_string())))
    }

    #[test]
    fn res_7_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xB9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, C".to_string())))
    }

    #[test]
    fn res_7_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xBA]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, D".to_string())))
    }

    #[test]
    fn res_7_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xBB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, E".to_string())))
    }

    #[test]
    fn res_7_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xBC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, H".to_string())))
    }

    #[test]
    fn res_7_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xBD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, L".to_string())))
    }

    #[test]
    fn res_7_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xBE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, [HL]".to_string())))
    }

    #[test]
    fn res_7_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xBF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, A".to_string())))
    }

    // 0xCX
    #[test]
    fn set_0_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xC0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, B".to_string())))
    }

    #[test]
    fn set_0_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xC1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, C".to_string())))
    }

    #[test]
    fn set_0_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xC2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, D".to_string())))
    }

    #[test]
    fn set_0_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xC3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, E".to_string())))
    }

    #[test]
    fn set_0_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xC4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, H".to_string())))
    }

    #[test]
    fn set_0_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xC5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, L".to_string())))
    }

    #[test]
    fn set_0_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xC6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, [HL]".to_string())))
    }

    #[test]
    fn set_0_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xC7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, A".to_string())))
    }

    #[test]
    fn set_1_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xC8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, B".to_string())))
    }

    #[test]
    fn set_1_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xC9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, C".to_string())))
    }

    #[test]
    fn set_1_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xCA]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, D".to_string())))
    }

    #[test]
    fn set_1_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xCB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, E".to_string())))
    }

    #[test]
    fn set_1_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xCC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, H".to_string())))
    }

    #[test]
    fn set_1_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xCD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, L".to_string())))
    }

    #[test]
    fn set_1_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xCE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, [HL]".to_string())))
    }

    #[test]
    fn set_1_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xCF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, A".to_string())))
    }

    // 0xDX
    #[test]
    fn set_2_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xD0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, B".to_string())))
    }

    #[test]
    fn set_2_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xD1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, C".to_string())))
    }

    #[test]
    fn set_2_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xD2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, D".to_string())))
    }

    #[test]
    fn set_2_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xD3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, E".to_string())))
    }

    #[test]
    fn set_2_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xD4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, H".to_string())))
    }

    #[test]
    fn set_2_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xD5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, L".to_string())))
    }

    #[test]
    fn set_2_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xD6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, [HL]".to_string())))
    }

    #[test]
    fn set_2_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xD7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, A".to_string())))
    }

    #[test]
    fn set_3_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xD8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, B".to_string())))
    }

    #[test]
    fn set_3_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xD9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, C".to_string())))
    }

    #[test]
    fn set_3_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xDA]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, D".to_string())))
    }

    #[test]
    fn set_3_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xDB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, E".to_string())))
    }

    #[test]
    fn set_3_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xDC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, H".to_string())))
    }

    #[test]
    fn set_3_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xDD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, L".to_string())))
    }

    #[test]
    fn set_3_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xDE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, [HL]".to_string())))
    }

    #[test]
    fn set_3_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xDF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, A".to_string())))
    }

    // 0xEX
    #[test]
    fn set_4_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xE0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, B".to_string())))
    }

    #[test]
    fn set_4_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xE1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, C".to_string())))
    }

    #[test]
    fn set_4_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xE2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, D".to_string())))
    }

    #[test]
    fn set_4_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xE3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, E".to_string())))
    }

    #[test]
    fn set_4_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xE4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, H".to_string())))
    }

    #[test]
    fn set_4_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xE5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, L".to_string())))
    }

    #[test]
    fn set_4_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xE6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, [HL]".to_string())))
    }

    #[test]
    fn set_4_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xE7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, A".to_string())))
    }

    #[test]
    fn set_5_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xE8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, B".to_string())))
    }

    #[test]
    fn set_5_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xE9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, C".to_string())))
    }

    #[test]
    fn set_5_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xEA]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, D".to_string())))
    }

    #[test]
    fn set_5_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xEB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, E".to_string())))
    }

    #[test]
    fn set_5_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xEC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, H".to_string())))
    }

    #[test]
    fn set_5_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xED]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, L".to_string())))
    }

    #[test]
    fn set_5_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xEE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, [HL]".to_string())))
    }

    #[test]
    fn set_5_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xEF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, A".to_string())))
    }

    // 0xFX
    #[test]
    fn set_6_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xF0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, B".to_string())))
    }

    #[test]
    fn set_6_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xF1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, C".to_string())))
    }

    #[test]
    fn set_6_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xF2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, D".to_string())))
    }

    #[test]
    fn set_6_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xF3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, E".to_string())))
    }

    #[test]
    fn set_6_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xF4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, H".to_string())))
    }

    #[test]
    fn set_6_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xF5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, L".to_string())))
    }

    #[test]
    fn set_6_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xF6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, [HL]".to_string())))
    }

    #[test]
    fn set_6_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xF7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, A".to_string())))
    }

    #[test]
    fn set_7_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xF8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, B".to_string())))
    }

    #[test]
    fn set_7_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xF9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, C".to_string())))
    }

    #[test]
    fn set_7_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xFA]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, D".to_string())))
    }

    #[test]
    fn set_7_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xFB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, E".to_string())))
    }

    #[test]
    fn set_7_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xFC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, H".to_string())))
    }

    #[test]
    fn set_7_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xFD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, L".to_string())))
    }

    #[test]
    fn set_7_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xFE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, [HL]".to_string())))
    }

    #[test]
    fn set_7_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xFF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disassemble(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, A".to_string())))
    }
}
