use std::fmt::Display;

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

// -R -> Reference. e.g. HLR -> [HL]
// -I -> Increment
// -D -> Decrement
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

    // Conditions
    cZ,
    cNZ,
    cC,
    cNC,

    N8(Option<u8>),
    I8(Option<u8>),
    N16(Option<u16>),
    N16R(Option<u16>), // [n16],

    Index(u8),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// decode
fn d(mn: Mnemonic, ops: &[Operand], prefs: &Preferences) -> Result<(u16, String), String> {
    let mut buffer = String::new();
    let mut count = 1;

    use std::fmt::Write;
    use Operand::*;

    write!(buffer, "{}", mn.to_string()).unwrap();

    for (idx, op) in ops.iter().enumerate() {
        if idx == 0 {
            write!(buffer, " ").unwrap();
        } else {
            write!(buffer, ",").unwrap();
            if prefs.comma_space {
                write!(buffer, " ").unwrap();
            }
        }

        match op {
            cZ | cNZ | cC | cNC => {
                let v = op.to_string();
                write!(buffer, "{}", &v[1..]).unwrap();
            }
            BCR | DER | HLR | CR => {
                let mut v = op.to_string();
                v.pop();
                write!(buffer, "[{}]", v).unwrap();
            },
            HLI | HLD => write!(buffer, "[{}]", op).unwrap(),
            N8(byte) => {
                if let Mnemonic::LDH = mn {
                    if let Some(val) = byte {
                        write!(buffer, "[$FF{:02X}]", val).unwrap();
                        count += 1;
                    } else {
                        return Err("Couldn't get byte operand value".to_string())
                    }
                } else {
                    if let Some(val) = byte {
                        write!(buffer, "${:02X}", val).unwrap();
                        if !matches!(mn, Mnemonic::RST) {
                            count += 1;
                        }
                    } else {
                        return Err("Couldn't get byte operand value".to_string())
                    }
                }
            },
            I8(byte) => {
                if let Mnemonic::LD = mn {
                    if let Some(val) = byte {
                        let val = *val as i8;
                        let sign = if val < 0 { "" } else { "+" };
                        write!(buffer, "SP{}{}", sign, val).unwrap();
                        count += 1;
                    } else {
                        return Err("Couldn't get byte operand value".to_string())
                    }
                } else {
                    if let Some(val) = byte {
                        write!(buffer, "{}", *val as i8).unwrap();
                        count += 1;
                    } else {
                        return Err("Couldn't get byte operand value".to_string())
                    }
                }
            },
            N16(word) => {
                if let Some(val) = word {
                    write!(buffer, "${:04X}", val).unwrap();
                    count += 2;
                } else {
                    return Err("Couldn't get word operand value".to_string())
                }
            },
            N16R(word) => {
                if let Some(val) = word {
                    write!(buffer, "[${:04X}]", val).unwrap();
                    count += 2;
                } else {
                    return Err("Couldn't get word operand value".to_string())
                }
            },
            Index(idx) => write!(buffer, "{}", idx).unwrap(),
            val => write!(buffer, "{}", val.to_string()).unwrap(),
        }
    }

    if !prefs.upcase {
        buffer = buffer.to_lowercase();
    }

    Ok((count, buffer))
}

// Trait to be implemented by the `disass` function caller. This allows the `disass` function to
// access the binary Game Boy data.
pub trait MemoryBus {
    fn read_byte(&self, addr: u16) -> Option<u8>;
    fn read_word(&self, addr: u16) -> Option<u16>;
}

// Display preferences for the `disass` function.
//
// * `upcase`: return the textual representation as UPCASE letters (including hexadecimal)
// * `comma_space`: add or a not a space after a comma with 2 operands (e.g. `ld a, b`)
pub struct Preferences {
    pub upcase: bool,
    pub comma_space: bool,
}

impl Preferences {
    pub fn new() -> Preferences {
        Preferences { upcase: false, comma_space: true }
    }
}

// Return a textual representation of a Game Boy binary operation, compatible with the RGBDS syntax in a `String`.
//
// # Example
// ```
// let bus = GameboyBus::new(vec![0x01, 0x12, 0x34]);
// let prefs = Preferences{upcase: true, comma_space: true};
// let result = disass(&bus, 0x0, &prefs);
//
// assert_eq!(result, Ok((3, "LD BC, $1234".to_string())))
// ```
//
// # Errors
//
// * The operation needs one or two operands but an insufficient number is found
// * The opcode isn't a valid Game Boy operation (unsupported)
//
// # Result
//
// Returns a tuple containing the number of bytes (as a u16) consumed and the textual representation in a
// String.
//
// The byte count number can be used to increment a PC register in an emulator.
pub fn disass(bus: &impl MemoryBus, addr: u16, prefs: &Preferences) -> Result<(u16, String), String> {
    let opcode = bus.read_byte(addr).ok_or(format!("Opcode not found at address {:04X}", addr))?;

    let next_byte = bus.read_byte(addr + 1);
    let next_word = bus.read_word(addr + 1);

    use Mnemonic::*;
    use Operand::*;

    match opcode {
        0x00 => d(NOP, &[], prefs),
        0x01 => d(LD, &[BC, N16(next_word)], prefs),
        0x02 => d(LD, &[BCR, A], prefs),
        0x03 => d(INC, &[BC], prefs),
        0x04 => d(INC, &[B], prefs),
        0x05 => d(DEC, &[B], prefs),
        0x06 => d(LD, &[B, N8(next_byte)], prefs),
        0x07 => d(RLCA, &[], prefs),

        0x08 => d(LD, &[N16R(next_word), SP], prefs),
        0x09 => d(ADD, &[HL, BC], prefs),
        0x0A => d(LD, &[A, BCR], prefs),
        0x0B => d(DEC, &[BC], prefs),
        0x0C => d(INC, &[C], prefs),
        0x0D => d(DEC, &[C], prefs),
        0x0E => d(LD, &[C, N8(next_byte)], prefs),
        0x0F => d(RRCA, &[], prefs),

        0x10 => d(STOP, &[], prefs),
        0x11 => d(LD, &[DE, N16(next_word)], prefs),
        0x12 => d(LD, &[DER, A], prefs),
        0x13 => d(INC, &[DE], prefs),
        0x14 => d(INC, &[D], prefs),
        0x15 => d(DEC, &[D], prefs),
        0x16 => d(LD, &[D, N8(next_byte)], prefs),
        0x17 => d(RLA, &[], prefs),

        0x18 => d(JR, &[I8(next_byte)], prefs),
        0x19 => d(ADD, &[HL, DE], prefs),
        0x1A => d(LD, &[A, DER], prefs),
        0x1B => d(DEC, &[DE], prefs),
        0x1C => d(INC, &[E], prefs),
        0x1D => d(DEC, &[E], prefs),
        0x1E => d(LD, &[E, N8(next_byte)], prefs),
        0x1F => d(RRA, &[], prefs),

        0x20 => d(JR, &[cNZ, I8(next_byte)], prefs),
        0x21 => d(LD, &[HL, N16(next_word)], prefs),
        0x22 => d(LD, &[HLI, A], prefs),
        0x23 => d(INC, &[HL], prefs),
        0x24 => d(INC, &[H], prefs),
        0x25 => d(DEC, &[H], prefs),
        0x26 => d(LD, &[H, N8(next_byte)], prefs),
        0x27 => d(DAA, &[], prefs),

        0x28 => d(JR, &[cZ, I8(next_byte)], prefs),
        0x29 => d(ADD, &[HL, HL], prefs),
        0x2A => d(LD, &[A, HLI], prefs),
        0x2B => d(DEC, &[HL], prefs),
        0x2C => d(INC, &[L], prefs),
        0x2D => d(DEC, &[L], prefs),
        0x2E => d(LD, &[L, N8(next_byte)], prefs),
        0x2F => d(CPL, &[], prefs),

        0x30 => d(JR, &[cNC, I8(next_byte)], prefs),
        0x31 => d(LD, &[SP, N16(next_word)], prefs),
        0x32 => d(LD, &[HLD, A], prefs),
        0x33 => d(INC, &[SP], prefs),
        0x34 => d(INC, &[HLR], prefs),
        0x35 => d(DEC, &[HLR], prefs),
        0x36 => d(LD, &[HLR, N8(next_byte)], prefs),
        0x37 => d(SCF, &[], prefs),

        0x38 => d(JR, &[cC, I8(next_byte)], prefs),
        0x39 => d(ADD, &[HL, SP], prefs),
        0x3A => d(LD, &[A, HLD], prefs),
        0x3B => d(DEC, &[SP], prefs),
        0x3C => d(INC, &[A], prefs),
        0x3D => d(DEC, &[A], prefs),
        0x3E => d(LD, &[A, N8(next_byte)], prefs),
        0x3F => d(CCF, &[], prefs),

        0x40 => d(LD, &[B, B], prefs),
        0x41 => d(LD, &[B, C], prefs),
        0x42 => d(LD, &[B, D], prefs),
        0x43 => d(LD, &[B, E], prefs),
        0x44 => d(LD, &[B, H], prefs),
        0x45 => d(LD, &[B, L], prefs),
        0x46 => d(LD, &[B, HLR], prefs),
        0x47 => d(LD, &[B, A], prefs),

        0x48 => d(LD, &[C, B], prefs),
        0x49 => d(LD, &[C, C], prefs),
        0x4A => d(LD, &[C, D], prefs),
        0x4B => d(LD, &[C, E], prefs),
        0x4C => d(LD, &[C, H], prefs),
        0x4D => d(LD, &[C, L], prefs),
        0x4E => d(LD, &[C, HLR], prefs),
        0x4F => d(LD, &[C, A], prefs),

        0x50 => d(LD, &[D, B], prefs),
        0x51 => d(LD, &[D, C], prefs),
        0x52 => d(LD, &[D, D], prefs),
        0x53 => d(LD, &[D, E], prefs),
        0x54 => d(LD, &[D, H], prefs),
        0x55 => d(LD, &[D, L], prefs),
        0x56 => d(LD, &[D, HLR], prefs),
        0x57 => d(LD, &[D, A], prefs),

        0x58 => d(LD, &[E, B], prefs),
        0x59 => d(LD, &[E, C], prefs),
        0x5A => d(LD, &[E, D], prefs),
        0x5B => d(LD, &[E, E], prefs),
        0x5C => d(LD, &[E, H], prefs),
        0x5D => d(LD, &[E, L], prefs),
        0x5E => d(LD, &[E, HLR], prefs),
        0x5F => d(LD, &[E, A], prefs),

        0x60 => d(LD, &[H, B], prefs),
        0x61 => d(LD, &[H, C], prefs),
        0x62 => d(LD, &[H, D], prefs),
        0x63 => d(LD, &[H, E], prefs),
        0x64 => d(LD, &[H, H], prefs),
        0x65 => d(LD, &[H, L], prefs),
        0x66 => d(LD, &[H, HLR], prefs),
        0x67 => d(LD, &[H, A], prefs),

        0x68 => d(LD, &[L, B], prefs),
        0x69 => d(LD, &[L, C], prefs),
        0x6A => d(LD, &[L, D], prefs),
        0x6B => d(LD, &[L, E], prefs),
        0x6C => d(LD, &[L, H], prefs),
        0x6D => d(LD, &[L, L], prefs),
        0x6E => d(LD, &[L, HLR], prefs),
        0x6F => d(LD, &[L, A], prefs),

        0x70 => d(LD, &[HLR, B], prefs),
        0x71 => d(LD, &[HLR, C], prefs),
        0x72 => d(LD, &[HLR, D], prefs),
        0x73 => d(LD, &[HLR, E], prefs),
        0x74 => d(LD, &[HLR, H], prefs),
        0x75 => d(LD, &[HLR, L], prefs),
        0x76 => d(HALT, &[], prefs),
        0x77 => d(LD, &[HLR, A], prefs),

        0x78 => d(LD, &[A, B], prefs),
        0x79 => d(LD, &[A, C], prefs),
        0x7A => d(LD, &[A, D], prefs),
        0x7B => d(LD, &[A, E], prefs),
        0x7C => d(LD, &[A, H], prefs),
        0x7D => d(LD, &[A, L], prefs),
        0x7E => d(LD, &[A, HLR], prefs),
        0x7F => d(LD, &[A, A], prefs),

        0x80 => d(ADD, &[A, B], prefs),
        0x81 => d(ADD, &[A, C], prefs),
        0x82 => d(ADD, &[A, D], prefs),
        0x83 => d(ADD, &[A, E], prefs),
        0x84 => d(ADD, &[A, H], prefs),
        0x85 => d(ADD, &[A, L], prefs),
        0x86 => d(ADD, &[A, HLR], prefs),
        0x87 => d(ADD, &[A, A], prefs),

        0x88 => d(ADC, &[A, B], prefs),
        0x89 => d(ADC, &[A, C], prefs),
        0x8A => d(ADC, &[A, D], prefs),
        0x8B => d(ADC, &[A, E], prefs),
        0x8C => d(ADC, &[A, H], prefs),
        0x8D => d(ADC, &[A, L], prefs),
        0x8E => d(ADC, &[A, HLR], prefs),
        0x8F => d(ADC, &[A, A], prefs),

        0x90 => d(SUB, &[A, B], prefs),
        0x91 => d(SUB, &[A, C], prefs),
        0x92 => d(SUB, &[A, D], prefs),
        0x93 => d(SUB, &[A, E], prefs),
        0x94 => d(SUB, &[A, H], prefs),
        0x95 => d(SUB, &[A, L], prefs),
        0x96 => d(SUB, &[A, HLR], prefs),
        0x97 => d(SUB, &[A, A], prefs),

        0x98 => d(SBC, &[A, B], prefs),
        0x99 => d(SBC, &[A, C], prefs),
        0x9A => d(SBC, &[A, D], prefs),
        0x9B => d(SBC, &[A, E], prefs),
        0x9C => d(SBC, &[A, H], prefs),
        0x9D => d(SBC, &[A, L], prefs),
        0x9E => d(SBC, &[A, HLR], prefs),
        0x9F => d(SBC, &[A, A], prefs),

        0xA0 => d(AND, &[A, B], prefs),
        0xA1 => d(AND, &[A, C], prefs),
        0xA2 => d(AND, &[A, D], prefs),
        0xA3 => d(AND, &[A, E], prefs),
        0xA4 => d(AND, &[A, H], prefs),
        0xA5 => d(AND, &[A, L], prefs),
        0xA6 => d(AND, &[A, HLR], prefs),
        0xA7 => d(AND, &[A, A], prefs),

        0xA8 => d(XOR, &[A, B], prefs),
        0xA9 => d(XOR, &[A, C], prefs),
        0xAA => d(XOR, &[A, D], prefs),
        0xAB => d(XOR, &[A, E], prefs),
        0xAC => d(XOR, &[A, H], prefs),
        0xAD => d(XOR, &[A, L], prefs),
        0xAE => d(XOR, &[A, HLR], prefs),
        0xAF => d(XOR, &[A, A], prefs),

        0xB0 => d(OR, &[A, B], prefs),
        0xB1 => d(OR, &[A, C], prefs),
        0xB2 => d(OR, &[A, D], prefs),
        0xB3 => d(OR, &[A, E], prefs),
        0xB4 => d(OR, &[A, H], prefs),
        0xB5 => d(OR, &[A, L], prefs),
        0xB6 => d(OR, &[A, HLR], prefs),
        0xB7 => d(OR, &[A, A], prefs),

        0xB8 => d(CP, &[A, B], prefs),
        0xB9 => d(CP, &[A, C], prefs),
        0xBA => d(CP, &[A, D], prefs),
        0xBB => d(CP, &[A, E], prefs),
        0xBC => d(CP, &[A, H], prefs),
        0xBD => d(CP, &[A, L], prefs),
        0xBE => d(CP, &[A, HLR], prefs),
        0xBF => d(CP, &[A, A], prefs),

        0xC0 => d(RET, &[cNZ], prefs),
        0xC1 => d(POP, &[BC], prefs),
        0xC2 => d(JP, &[cNZ, N16(next_word)], prefs),
        0xC3 => d(JP, &[N16(next_word)], prefs),
        0xC4 => d(CALL, &[cNZ, N16(next_word)], prefs),
        0xC5 => d(PUSH, &[BC], prefs),
        0xC6 => d(ADD, &[A, N8(next_byte)], prefs),
        0xC7 => d(RST, &[N8(Some(0x00))], prefs),

        0xC8 => d(RET, &[cZ], prefs),
        0xC9 => d(RET, &[], prefs),
        0xCA => d(JP, &[cZ, N16(next_word)], prefs),
        0xCB => cb_prefix(next_byte, prefs),
        0xCC => d(CALL, &[cZ, N16(next_word)], prefs),
        0xCD => d(CALL, &[N16(next_word)], prefs),
        0xCE => d(ADC, &[A, N8(next_byte)], prefs),
        0xCF => d(RST, &[N8(Some(0x08))], prefs),

        0xD0 => d(RET, &[cNC], prefs),
        0xD1 => d(POP, &[DE], prefs),
        0xD2 => d(JP, &[cNC, N16(next_word)], prefs),
        0xD4 => d(CALL, &[cNC, N16(next_word)], prefs),
        0xD5 => d(PUSH, &[DE], prefs),
        0xD6 => d(SUB, &[A, N8(next_byte)], prefs),
        0xD7 => d(RST, &[N8(Some(0x10))], prefs),

        0xD8 => d(RET, &[C], prefs),
        0xD9 => d(RETI, &[], prefs),
        0xDA => d(JP, &[cC, N16(next_word)], prefs),
        0xDC => d(CALL, &[cC, N16(next_word)], prefs),
        0xDE => d(SBC, &[A, N8(next_byte)], prefs),
        0xDF => d(RST, &[N8(Some(0x18))], prefs),

        0xE0 => d(LDH, &[N8(next_byte), A], prefs),
        0xE1 => d(POP, &[HL], prefs),
        0xE2 => d(LDH, &[CR, A], prefs),
        0xE5 => d(PUSH, &[HL], prefs),
        0xE6 => d(AND, &[A, N8(next_byte)], prefs),
        0xE7 => d(RST, &[N8(Some(0x20))], prefs),

        0xE8 => d(ADD, &[SP, I8(next_byte)], prefs),
        0xE9 => d(JP, &[HL], prefs),
        0xEA => d(LD, &[N16R(next_word), A], prefs),
        0xEE => d(XOR, &[A, N8(next_byte)], prefs),
        0xEF => d(RST, &[N8(Some(0x28))], prefs),

        0xF0 => d(LDH, &[A, N8(next_byte)], prefs),
        0xF1 => d(POP, &[AF], prefs),
        0xF2 => d(LDH, &[A, CR], prefs),
        0xF3 => d(DI, &[], prefs),
        0xF5 => d(PUSH, &[AF], prefs),
        0xF6 => d(OR, &[A, N8(next_byte)], prefs),
        0xF7 => d(RST, &[N8(Some(0x30))], prefs),

        0xF8 => d(LD, &[HL, I8(next_byte)], prefs),
        0xF9 => d(LD, &[SP, HL], prefs),
        0xFA => d(LD, &[A, N16R(next_word)], prefs),
        0xFB => d(EI, &[], prefs),
        0xFE => d(CP, &[A, N8(next_byte)], prefs),
        0xFF => d(RST, &[N8(Some(0x38))], prefs),

        _ => Err(format!("Unsupported opcode {:02X}", opcode)),
    }
}

fn cb_prefix(byte: Option<u8>, prefs: &Preferences) -> Result<(u16, String), String> {
    let opcode = byte.ok_or("Couldn't read the next byte for CB prefix")?;

    use Mnemonic::*;
    use Operand::*;

    let (count, line) = match opcode {
        0x00 => d(RLC, &[B], prefs),
        0x01 => d(RLC, &[C], prefs),
        0x02 => d(RLC, &[D], prefs),
        0x03 => d(RLC, &[E], prefs),
        0x04 => d(RLC, &[H], prefs),
        0x05 => d(RLC, &[L], prefs),
        0x06 => d(RLC, &[HLR], prefs),
        0x07 => d(RLC, &[A], prefs),

        0x08 => d(RRC, &[B], prefs),
        0x09 => d(RRC, &[C], prefs),
        0x0A => d(RRC, &[D], prefs),
        0x0B => d(RRC, &[E], prefs),
        0x0C => d(RRC, &[H], prefs),
        0x0D => d(RRC, &[L], prefs),
        0x0E => d(RRC, &[HLR], prefs),
        0x0F => d(RRC, &[A], prefs),

        0x10 => d(RL, &[B], prefs),
        0x11 => d(RL, &[C], prefs),
        0x12 => d(RL, &[D], prefs),
        0x13 => d(RL, &[E], prefs),
        0x14 => d(RL, &[H], prefs),
        0x15 => d(RL, &[L], prefs),
        0x16 => d(RL, &[HLR], prefs),
        0x17 => d(RL, &[A], prefs),

        0x18 => d(RR, &[B], prefs),
        0x19 => d(RR, &[C], prefs),
        0x1A => d(RR, &[D], prefs),
        0x1B => d(RR, &[E], prefs),
        0x1C => d(RR, &[H], prefs),
        0x1D => d(RR, &[L], prefs),
        0x1E => d(RR, &[HLR], prefs),
        0x1F => d(RR, &[A], prefs),

        0x20 => d(SLA, &[B], prefs),
        0x21 => d(SLA, &[C], prefs),
        0x22 => d(SLA, &[D], prefs),
        0x23 => d(SLA, &[E], prefs),
        0x24 => d(SLA, &[H], prefs),
        0x25 => d(SLA, &[L], prefs),
        0x26 => d(SLA, &[HLR], prefs),
        0x27 => d(SLA, &[A], prefs),

        0x28 => d(SRA, &[B], prefs),
        0x29 => d(SRA, &[C], prefs),
        0x2A => d(SRA, &[D], prefs),
        0x2B => d(SRA, &[E], prefs),
        0x2C => d(SRA, &[H], prefs),
        0x2D => d(SRA, &[L], prefs),
        0x2E => d(SRA, &[HLR], prefs),
        0x2F => d(SRA, &[A], prefs),

        0x30 => d(SWAP, &[B], prefs),
        0x31 => d(SWAP, &[C], prefs),
        0x32 => d(SWAP, &[D], prefs),
        0x33 => d(SWAP, &[E], prefs),
        0x34 => d(SWAP, &[H], prefs),
        0x35 => d(SWAP, &[L], prefs),
        0x36 => d(SWAP, &[HLR], prefs),
        0x37 => d(SWAP, &[A], prefs),

        0x38 => d(SRL, &[B], prefs),
        0x39 => d(SRL, &[C], prefs),
        0x3A => d(SRL, &[D], prefs),
        0x3B => d(SRL, &[E], prefs),
        0x3C => d(SRL, &[H], prefs),
        0x3D => d(SRL, &[L], prefs),
        0x3E => d(SRL, &[HLR], prefs),
        0x3F => d(SRL, &[A], prefs),

        0x40 => d(BIT, &[Index(0), B], prefs),
        0x41 => d(BIT, &[Index(0), C], prefs),
        0x42 => d(BIT, &[Index(0), D], prefs),
        0x43 => d(BIT, &[Index(0), E], prefs),
        0x44 => d(BIT, &[Index(0), H], prefs),
        0x45 => d(BIT, &[Index(0), L], prefs),
        0x46 => d(BIT, &[Index(0), HLR], prefs),
        0x47 => d(BIT, &[Index(0), A], prefs),

        0x48 => d(BIT, &[Index(1), B], prefs),
        0x49 => d(BIT, &[Index(1), C], prefs),
        0x4A => d(BIT, &[Index(1), D], prefs),
        0x4B => d(BIT, &[Index(1), E], prefs),
        0x4C => d(BIT, &[Index(1), H], prefs),
        0x4D => d(BIT, &[Index(1), L], prefs),
        0x4E => d(BIT, &[Index(1), HLR], prefs),
        0x4F => d(BIT, &[Index(1), A], prefs),

        0x50 => d(BIT, &[Index(2), B], prefs),
        0x51 => d(BIT, &[Index(2), C], prefs),
        0x52 => d(BIT, &[Index(2), D], prefs),
        0x53 => d(BIT, &[Index(2), E], prefs),
        0x54 => d(BIT, &[Index(2), H], prefs),
        0x55 => d(BIT, &[Index(2), L], prefs),
        0x56 => d(BIT, &[Index(2), HLR], prefs),
        0x57 => d(BIT, &[Index(2), A], prefs),

        0x58 => d(BIT, &[Index(3), B], prefs),
        0x59 => d(BIT, &[Index(3), C], prefs),
        0x5A => d(BIT, &[Index(3), D], prefs),
        0x5B => d(BIT, &[Index(3), E], prefs),
        0x5C => d(BIT, &[Index(3), H], prefs),
        0x5D => d(BIT, &[Index(3), L], prefs),
        0x5E => d(BIT, &[Index(3), HLR], prefs),
        0x5F => d(BIT, &[Index(3), A], prefs),

        0x60 => d(BIT, &[Index(4), B], prefs),
        0x61 => d(BIT, &[Index(4), C], prefs),
        0x62 => d(BIT, &[Index(4), D], prefs),
        0x63 => d(BIT, &[Index(4), E], prefs),
        0x64 => d(BIT, &[Index(4), H], prefs),
        0x65 => d(BIT, &[Index(4), L], prefs),
        0x66 => d(BIT, &[Index(4), HLR], prefs),
        0x67 => d(BIT, &[Index(4), A], prefs),

        0x68 => d(BIT, &[Index(5), B], prefs),
        0x69 => d(BIT, &[Index(5), C], prefs),
        0x6A => d(BIT, &[Index(5), D], prefs),
        0x6B => d(BIT, &[Index(5), E], prefs),
        0x6C => d(BIT, &[Index(5), H], prefs),
        0x6D => d(BIT, &[Index(5), L], prefs),
        0x6E => d(BIT, &[Index(5), HLR], prefs),
        0x6F => d(BIT, &[Index(5), A], prefs),

        0x70 => d(BIT, &[Index(6), B], prefs),
        0x71 => d(BIT, &[Index(6), C], prefs),
        0x72 => d(BIT, &[Index(6), D], prefs),
        0x73 => d(BIT, &[Index(6), E], prefs),
        0x74 => d(BIT, &[Index(6), H], prefs),
        0x75 => d(BIT, &[Index(6), L], prefs),
        0x76 => d(BIT, &[Index(6), HLR], prefs),
        0x77 => d(BIT, &[Index(6), A], prefs),

        0x78 => d(BIT, &[Index(7), B], prefs),
        0x79 => d(BIT, &[Index(7), C], prefs),
        0x7A => d(BIT, &[Index(7), D], prefs),
        0x7B => d(BIT, &[Index(7), E], prefs),
        0x7C => d(BIT, &[Index(7), H], prefs),
        0x7D => d(BIT, &[Index(7), L], prefs),
        0x7E => d(BIT, &[Index(7), HLR], prefs),
        0x7F => d(BIT, &[Index(7), A], prefs),

        0x80 => d(RES, &[Index(0), B], prefs),
        0x81 => d(RES, &[Index(0), C], prefs),
        0x82 => d(RES, &[Index(0), D], prefs),
        0x83 => d(RES, &[Index(0), E], prefs),
        0x84 => d(RES, &[Index(0), H], prefs),
        0x85 => d(RES, &[Index(0), L], prefs),
        0x86 => d(RES, &[Index(0), HLR], prefs),
        0x87 => d(RES, &[Index(0), A], prefs),

        0x88 => d(RES, &[Index(1), B], prefs),
        0x89 => d(RES, &[Index(1), C], prefs),
        0x8A => d(RES, &[Index(1), D], prefs),
        0x8B => d(RES, &[Index(1), E], prefs),
        0x8C => d(RES, &[Index(1), H], prefs),
        0x8D => d(RES, &[Index(1), L], prefs),
        0x8E => d(RES, &[Index(1), HLR], prefs),
        0x8F => d(RES, &[Index(1), A], prefs),

        0x90 => d(RES, &[Index(2), B], prefs),
        0x91 => d(RES, &[Index(2), C], prefs),
        0x92 => d(RES, &[Index(2), D], prefs),
        0x93 => d(RES, &[Index(2), E], prefs),
        0x94 => d(RES, &[Index(2), H], prefs),
        0x95 => d(RES, &[Index(2), L], prefs),
        0x96 => d(RES, &[Index(2), HLR], prefs),
        0x97 => d(RES, &[Index(2), A], prefs),

        0x98 => d(RES, &[Index(3), B], prefs),
        0x99 => d(RES, &[Index(3), C], prefs),
        0x9A => d(RES, &[Index(3), D], prefs),
        0x9B => d(RES, &[Index(3), E], prefs),
        0x9C => d(RES, &[Index(3), H], prefs),
        0x9D => d(RES, &[Index(3), L], prefs),
        0x9E => d(RES, &[Index(3), HLR], prefs),
        0x9F => d(RES, &[Index(3), A], prefs),

        0xA0 => d(RES, &[Index(4), B], prefs),
        0xA1 => d(RES, &[Index(4), C], prefs),
        0xA2 => d(RES, &[Index(4), D], prefs),
        0xA3 => d(RES, &[Index(4), E], prefs),
        0xA4 => d(RES, &[Index(4), H], prefs),
        0xA5 => d(RES, &[Index(4), L], prefs),
        0xA6 => d(RES, &[Index(4), HLR], prefs),
        0xA7 => d(RES, &[Index(4), A], prefs),

        0xA8 => d(RES, &[Index(5), B], prefs),
        0xA9 => d(RES, &[Index(5), C], prefs),
        0xAA => d(RES, &[Index(5), D], prefs),
        0xAB => d(RES, &[Index(5), E], prefs),
        0xAC => d(RES, &[Index(5), H], prefs),
        0xAD => d(RES, &[Index(5), L], prefs),
        0xAE => d(RES, &[Index(5), HLR], prefs),
        0xAF => d(RES, &[Index(5), A], prefs),

        0xB0 => d(RES, &[Index(6), B], prefs),
        0xB1 => d(RES, &[Index(6), C], prefs),
        0xB2 => d(RES, &[Index(6), D], prefs),
        0xB3 => d(RES, &[Index(6), E], prefs),
        0xB4 => d(RES, &[Index(6), H], prefs),
        0xB5 => d(RES, &[Index(6), L], prefs),
        0xB6 => d(RES, &[Index(6), HLR], prefs),
        0xB7 => d(RES, &[Index(6), A], prefs),

        0xB8 => d(RES, &[Index(7), B], prefs),
        0xB9 => d(RES, &[Index(7), C], prefs),
        0xBA => d(RES, &[Index(7), D], prefs),
        0xBB => d(RES, &[Index(7), E], prefs),
        0xBC => d(RES, &[Index(7), H], prefs),
        0xBD => d(RES, &[Index(7), L], prefs),
        0xBE => d(RES, &[Index(7), HLR], prefs),
        0xBF => d(RES, &[Index(7), A], prefs),

        0xC0 => d(SET, &[Index(0), B], prefs),
        0xC1 => d(SET, &[Index(0), C], prefs),
        0xC2 => d(SET, &[Index(0), D], prefs),
        0xC3 => d(SET, &[Index(0), E], prefs),
        0xC4 => d(SET, &[Index(0), H], prefs),
        0xC5 => d(SET, &[Index(0), L], prefs),
        0xC6 => d(SET, &[Index(0), HLR], prefs),
        0xC7 => d(SET, &[Index(0), A], prefs),

        0xC8 => d(SET, &[Index(1), B], prefs),
        0xC9 => d(SET, &[Index(1), C], prefs),
        0xCA => d(SET, &[Index(1), D], prefs),
        0xCB => d(SET, &[Index(1), E], prefs),
        0xCC => d(SET, &[Index(1), H], prefs),
        0xCD => d(SET, &[Index(1), L], prefs),
        0xCE => d(SET, &[Index(1), HLR], prefs),
        0xCF => d(SET, &[Index(1), A], prefs),

        0xD0 => d(SET, &[Index(2), B], prefs),
        0xD1 => d(SET, &[Index(2), C], prefs),
        0xD2 => d(SET, &[Index(2), D], prefs),
        0xD3 => d(SET, &[Index(2), E], prefs),
        0xD4 => d(SET, &[Index(2), H], prefs),
        0xD5 => d(SET, &[Index(2), L], prefs),
        0xD6 => d(SET, &[Index(2), HLR], prefs),
        0xD7 => d(SET, &[Index(2), A], prefs),

        0xD8 => d(SET, &[Index(3), B], prefs),
        0xD9 => d(SET, &[Index(3), C], prefs),
        0xDA => d(SET, &[Index(3), D], prefs),
        0xDB => d(SET, &[Index(3), E], prefs),
        0xDC => d(SET, &[Index(3), H], prefs),
        0xDD => d(SET, &[Index(3), L], prefs),
        0xDE => d(SET, &[Index(3), HLR], prefs),
        0xDF => d(SET, &[Index(3), A], prefs),

        0xE0 => d(SET, &[Index(4), B], prefs),
        0xE1 => d(SET, &[Index(4), C], prefs),
        0xE2 => d(SET, &[Index(4), D], prefs),
        0xE3 => d(SET, &[Index(4), E], prefs),
        0xE4 => d(SET, &[Index(4), H], prefs),
        0xE5 => d(SET, &[Index(4), L], prefs),
        0xE6 => d(SET, &[Index(4), HLR], prefs),
        0xE7 => d(SET, &[Index(4), A], prefs),

        0xE8 => d(SET, &[Index(5), B], prefs),
        0xE9 => d(SET, &[Index(5), C], prefs),
        0xEA => d(SET, &[Index(5), D], prefs),
        0xEB => d(SET, &[Index(5), E], prefs),
        0xEC => d(SET, &[Index(5), H], prefs),
        0xED => d(SET, &[Index(5), L], prefs),
        0xEE => d(SET, &[Index(5), HLR], prefs),
        0xEF => d(SET, &[Index(5), A], prefs),

        0xF0 => d(SET, &[Index(6), B], prefs),
        0xF1 => d(SET, &[Index(6), C], prefs),
        0xF2 => d(SET, &[Index(6), D], prefs),
        0xF3 => d(SET, &[Index(6), E], prefs),
        0xF4 => d(SET, &[Index(6), H], prefs),
        0xF5 => d(SET, &[Index(6), L], prefs),
        0xF6 => d(SET, &[Index(6), HLR], prefs),
        0xF7 => d(SET, &[Index(6), A], prefs),

        0xF8 => d(SET, &[Index(7), B], prefs),
        0xF9 => d(SET, &[Index(7), C], prefs),
        0xFA => d(SET, &[Index(7), D], prefs),
        0xFB => d(SET, &[Index(7), E], prefs),
        0xFC => d(SET, &[Index(7), H], prefs),
        0xFD => d(SET, &[Index(7), L], prefs),
        0xFE => d(SET, &[Index(7), HLR], prefs),
        0xFF => d(SET, &[Index(7), A], prefs),
    }?;

    Ok((count + 1, line))
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
        let result = disass(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((1, "NOP".to_string())))
    }

    #[test]
    fn ld_bc_n16() {
        let bus = GameboyBus::new(vec![0x01, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "LD BC, $1234".to_string())))
    }

    #[test]
    fn ld_bc_missing_byte() {
        let bus = GameboyBus::new(vec![0x01, 0x12]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x0, &prefs);

        assert!(result.is_err())
    }

    #[test]
    fn ld_bc_missing_word() {
        let bus = GameboyBus::new(vec![0x01]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x0, &prefs);

        assert!(result.is_err())
    }

    #[test]
    fn ld_bc_n16r_lower() {
        let bus = GameboyBus::new(vec![0x01, 0x12, 0x34]);
        let prefs = Preferences{upcase: false, comma_space: true};
        let result = disass(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "ld bc, $1234".to_string())))
    }

    #[test]
    fn ld_bc_n16r_no_space() {
        let bus = GameboyBus::new(vec![0x01, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: false};
        let result = disass(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "LD BC,$1234".to_string())))
    }

    #[test]
    fn ld_bc_n16r_lower_no_space() {
        let bus = GameboyBus::new(vec![0x01, 0x12, 0x34]);
        let prefs = Preferences{upcase: false, comma_space: false};
        let result = disass(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "ld bc,$1234".to_string())))
    }

    #[test]
    fn ld_bcr_a() {
        let bus = GameboyBus::new(vec![0x02]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [BC], A".to_string())))
    }

    #[test]
    fn inc_bc() {
        let bus = GameboyBus::new(vec![0x03]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC BC".to_string())))
    }

    #[test]
    fn inc_b() {
        let bus = GameboyBus::new(vec![0x04]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC B".to_string())))
    }

    #[test]
    fn dec_b() {
        let bus = GameboyBus::new(vec![0x05]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC B".to_string())))
    }

    #[test]
    fn ld_b_n8() {
        let bus = GameboyBus::new(vec![0x06, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD B, $2A".to_string())))
    }

    #[test]
    fn rlca() {
        let bus = GameboyBus::new(vec![0x07]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RLCA".to_string())))
    }

    #[test]
    fn ld_n16r_sp() {
        let bus = GameboyBus::new(vec![0x08, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "LD [$1234], SP".to_string())))
    }

    #[test]
    fn add_hl_bc() {
        let bus = GameboyBus::new(vec![0x09]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD HL, BC".to_string())))
    }

    #[test]
    fn ld_a_bcr() {
        let bus = GameboyBus::new(vec![0x0A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, [BC]".to_string())))
    }

    #[test]
    fn dec_bc() {
        let bus = GameboyBus::new(vec![0x0B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC BC".to_string())))
    }

    #[test]
    fn inc_c() {
        let bus = GameboyBus::new(vec![0x0C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC C".to_string())))
    }

    #[test]
    fn dec_c() {
        let bus = GameboyBus::new(vec![0x0D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC C".to_string())))
    }

    #[test]
    fn ld_c_n8() {
        let bus = GameboyBus::new(vec![0x0E, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD C, $2A".to_string())))
    }

    #[test]
    fn rrca() {
        let bus = GameboyBus::new(vec![0x0F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RRCA".to_string())))
    }

    // 0x1X
    #[test]
    fn stop() {
        let bus = GameboyBus::new(vec![0x10]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((1, "STOP".to_string())))
    }

    #[test]
    fn ld_de_n16() {
        let bus = GameboyBus::new(vec![0x11, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "LD DE, $1234".to_string())))
    }

    #[test]
    fn ld_der_a() {
        let bus = GameboyBus::new(vec![0x12]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [DE], A".to_string())))
    }

    #[test]
    fn inc_de() {
        let bus = GameboyBus::new(vec![0x13]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC DE".to_string())))
    }

    #[test]
    fn inc_d() {
        let bus = GameboyBus::new(vec![0x14]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC D".to_string())))
    }

    #[test]
    fn dec_d() {
        let bus = GameboyBus::new(vec![0x15]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC D".to_string())))
    }

    #[test]
    fn ld_d_n8() {
        let bus = GameboyBus::new(vec![0x16, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD D, $2A".to_string())))
    }

    #[test]
    fn rla() {
        let bus = GameboyBus::new(vec![0x17]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RLA".to_string())))
    }

    #[test]
    fn jr_i8() {
        let bus = GameboyBus::new(vec![0x18, 0xFC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "JR -4".to_string())))
    }

    #[test]
    fn add_hl_de() {
        let bus = GameboyBus::new(vec![0x19]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD HL, DE".to_string())))
    }

    #[test]
    fn ld_a_der() {
        let bus = GameboyBus::new(vec![0x1A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, [DE]".to_string())))
    }

    #[test]
    fn dec_de() {
        let bus = GameboyBus::new(vec![0x1B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC DE".to_string())))
    }

    #[test]
    fn inc_e() {
        let bus = GameboyBus::new(vec![0x1C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC E".to_string())))
    }

    #[test]
    fn dec_e() {
        let bus = GameboyBus::new(vec![0x1D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC E".to_string())))
    }

    #[test]
    fn ld_e_n8() {
        let bus = GameboyBus::new(vec![0x1E, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD E, $2A".to_string())))
    }

    #[test]
    fn rra() {
        let bus = GameboyBus::new(vec![0x1F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RRA".to_string())))
    }

    // 0x2X
    #[test]
    fn jr_nz_i8() {
        let bus = GameboyBus::new(vec![0x20, 0xFC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((2, "JR NZ, -4".to_string())))
    }

    #[test]
    fn ld_hl_n16() {
        let bus = GameboyBus::new(vec![0x21, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "LD HL, $1234".to_string())))
    }

    #[test]
    fn ld_hli_a() {
        let bus = GameboyBus::new(vec![0x22]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HLI], A".to_string())))
    }

    #[test]
    fn inc_hl() {
        let bus = GameboyBus::new(vec![0x23]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC HL".to_string())))
    }

    #[test]
    fn inc_h() {
        let bus = GameboyBus::new(vec![0x24]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC H".to_string())))
    }

    #[test]
    fn dec_h() {
        let bus = GameboyBus::new(vec![0x25]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC H".to_string())))
    }

    #[test]
    fn ld_h_n8() {
        let bus = GameboyBus::new(vec![0x26, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD H, $2A".to_string())))
    }

    #[test]
    fn daa() {
        let bus = GameboyBus::new(vec![0x27]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DAA".to_string())))
    }

    #[test]
    fn jr_z_i8() {
        let bus = GameboyBus::new(vec![0x28, 0xFC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "JR Z, -4".to_string())))
    }

    #[test]
    fn add_hl_hl() {
        let bus = GameboyBus::new(vec![0x29]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD HL, HL".to_string())))
    }

    #[test]
    fn ld_a_hli() {
        let bus = GameboyBus::new(vec![0x2A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, [HLI]".to_string())))
    }

    #[test]
    fn dec_hl() {
        let bus = GameboyBus::new(vec![0x2B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC HL".to_string())))
    }

    #[test]
    fn inc_l() {
        let bus = GameboyBus::new(vec![0x2C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC L".to_string())))
    }

    #[test]
    fn dec_l() {
        let bus = GameboyBus::new(vec![0x2D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC L".to_string())))
    }

    #[test]
    fn ld_l_n8() {
        let bus = GameboyBus::new(vec![0x2E, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD L, $2A".to_string())))
    }

    #[test]
    fn cpl() {
        let bus = GameboyBus::new(vec![0x2F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CPL".to_string())))
    }

    // 0x3X
    #[test]
    fn jr_nc_i8() {
        let bus = GameboyBus::new(vec![0x30, 0xFC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((2, "JR NC, -4".to_string())))
    }

    #[test]
    fn ld_sp_n16() {
        let bus = GameboyBus::new(vec![0x31, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x0, &prefs);

        assert_eq!(result, Ok((3, "LD SP, $1234".to_string())))
    }

    #[test]
    fn ld_hld_a() {
        let bus = GameboyBus::new(vec![0x32]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HLD], A".to_string())))
    }

    #[test]
    fn inc_sp() {
        let bus = GameboyBus::new(vec![0x33]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC SP".to_string())))
    }

    #[test]
    fn inc_hlr() {
        let bus = GameboyBus::new(vec![0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC [HL]".to_string())))
    }

    #[test]
    fn dec_hlr() {
        let bus = GameboyBus::new(vec![0x35]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC [HL]".to_string())))
    }

    #[test]
    fn ld_hlr_n8() {
        let bus = GameboyBus::new(vec![0x36, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD [HL], $2A".to_string())))
    }

    #[test]
    fn scf() {
        let bus = GameboyBus::new(vec![0x37]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SCF".to_string())))
    }

    #[test]
    fn jr_c_i8() {
        let bus = GameboyBus::new(vec![0x38, 0xFC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "JR C, -4".to_string())))
    }

    #[test]
    fn add_hl_sp() {
        let bus = GameboyBus::new(vec![0x39]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD HL, SP".to_string())))
    }

    #[test]
    fn ld_a_hld() {
        let bus = GameboyBus::new(vec![0x3A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, [HLD]".to_string())))
    }

    #[test]
    fn dec_sp() {
        let bus = GameboyBus::new(vec![0x3B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC SP".to_string())))
    }

    #[test]
    fn inc_a() {
        let bus = GameboyBus::new(vec![0x3C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "INC A".to_string())))
    }

    #[test]
    fn dec_a() {
        let bus = GameboyBus::new(vec![0x3D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DEC A".to_string())))
    }

    #[test]
    fn ld_a_n8() {
        let bus = GameboyBus::new(vec![0x3E, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD A, $2A".to_string())))
    }

    #[test]
    fn ccf() {
        let bus = GameboyBus::new(vec![0x3F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CCF".to_string())))
    }

    // 0x4X
    #[test]
    fn ld_b_b() {
        let bus = GameboyBus::new(vec![0x40]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, B".to_string())))
    }

    #[test]
    fn ld_b_c() {
        let bus = GameboyBus::new(vec![0x41]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, C".to_string())))
    }

    #[test]
    fn ld_b_d() {
        let bus = GameboyBus::new(vec![0x42]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, D".to_string())))
    }

    #[test]
    fn ld_b_e() {
        let bus = GameboyBus::new(vec![0x43]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, E".to_string())))
    }

    #[test]
    fn ld_b_h() {
        let bus = GameboyBus::new(vec![0x44]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, H".to_string())))
    }

    #[test]
    fn ld_b_l() {
        let bus = GameboyBus::new(vec![0x45]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, L".to_string())))
    }

    #[test]
    fn ld_b_hlr() {
        let bus = GameboyBus::new(vec![0x46]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, [HL]".to_string())))
    }

    #[test]
    fn ld_b_a() {
        let bus = GameboyBus::new(vec![0x47]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD B, A".to_string())))
    }

    #[test]
    fn ld_c_b() {
        let bus = GameboyBus::new(vec![0x48]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, B".to_string())))
    }

    #[test]
    fn ld_c_c() {
        let bus = GameboyBus::new(vec![0x49]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, C".to_string())))
    }

    #[test]
    fn ld_c_d() {
        let bus = GameboyBus::new(vec![0x4a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, D".to_string())))
    }

    #[test]
    fn ld_c_e() {
        let bus = GameboyBus::new(vec![0x4b]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, E".to_string())))
    }

    #[test]
    fn ld_c_h() {
        let bus = GameboyBus::new(vec![0x4c]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, H".to_string())))
    }

    #[test]
    fn ld_c_l() {
        let bus = GameboyBus::new(vec![0x4d]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, L".to_string())))
    }

    #[test]
    fn ld_c_hlr() {
        let bus = GameboyBus::new(vec![0x4e]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, [HL]".to_string())))
    }

    #[test]
    fn ld_c_a() {
        let bus = GameboyBus::new(vec![0x4f]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD C, A".to_string())))
    }

    // 0x5X
    #[test]
    fn ld_d_b() {
        let bus = GameboyBus::new(vec![0x50]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, B".to_string())))
    }

    #[test]
    fn ld_d_c() {
        let bus = GameboyBus::new(vec![0x51]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, C".to_string())))
    }

    #[test]
    fn ld_d_d() {
        let bus = GameboyBus::new(vec![0x52]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, D".to_string())))
    }

    #[test]
    fn ld_d_e() {
        let bus = GameboyBus::new(vec![0x53]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, E".to_string())))
    }

    #[test]
    fn ld_d_h() {
        let bus = GameboyBus::new(vec![0x54]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, H".to_string())))
    }

    #[test]
    fn ld_d_l() {
        let bus = GameboyBus::new(vec![0x55]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, L".to_string())))
    }

    #[test]
    fn ld_d_hlr() {
        let bus = GameboyBus::new(vec![0x56]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, [HL]".to_string())))
    }

    #[test]
    fn ld_d_a() {
        let bus = GameboyBus::new(vec![0x57]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD D, A".to_string())))
    }

    #[test]
    fn ld_e_b() {
        let bus = GameboyBus::new(vec![0x58]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, B".to_string())))
    }

    #[test]
    fn ld_e_c() {
        let bus = GameboyBus::new(vec![0x59]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, C".to_string())))
    }

    #[test]
    fn ld_e_d() {
        let bus = GameboyBus::new(vec![0x5a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, D".to_string())))
    }

    #[test]
    fn ld_e_e() {
        let bus = GameboyBus::new(vec![0x5b]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, E".to_string())))
    }

    #[test]
    fn ld_e_h() {
        let bus = GameboyBus::new(vec![0x5c]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, H".to_string())))
    }

    #[test]
    fn ld_e_l() {
        let bus = GameboyBus::new(vec![0x5d]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, L".to_string())))
    }

    #[test]
    fn ld_e_hlr() {
        let bus = GameboyBus::new(vec![0x5e]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, [HL]".to_string())))
    }

    #[test]
    fn ld_e_a() {
        let bus = GameboyBus::new(vec![0x5f]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD E, A".to_string())))
    }

    // 0x6X
    #[test]
    fn ld_h_b() {
        let bus = GameboyBus::new(vec![0x60]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, B".to_string())))
    }

    #[test]
    fn ld_h_c() {
        let bus = GameboyBus::new(vec![0x61]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, C".to_string())))
    }

    #[test]
    fn ld_h_d() {
        let bus = GameboyBus::new(vec![0x62]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, D".to_string())))
    }

    #[test]
    fn ld_h_e() {
        let bus = GameboyBus::new(vec![0x63]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, E".to_string())))
    }

    #[test]
    fn ld_h_h() {
        let bus = GameboyBus::new(vec![0x64]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, H".to_string())))
    }

    #[test]
    fn ld_h_l() {
        let bus = GameboyBus::new(vec![0x65]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, L".to_string())))
    }

    #[test]
    fn ld_h_hlr() {
        let bus = GameboyBus::new(vec![0x66]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, [HL]".to_string())))
    }

    #[test]
    fn ld_h_a() {
        let bus = GameboyBus::new(vec![0x67]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD H, A".to_string())))
    }

    #[test]
    fn ld_l_b() {
        let bus = GameboyBus::new(vec![0x68]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, B".to_string())))
    }

    #[test]
    fn ld_l_c() {
        let bus = GameboyBus::new(vec![0x69]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, C".to_string())))
    }

    #[test]
    fn ld_l_d() {
        let bus = GameboyBus::new(vec![0x6a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, D".to_string())))
    }

    #[test]
    fn ld_l_e() {
        let bus = GameboyBus::new(vec![0x6b]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, E".to_string())))
    }

    #[test]
    fn ld_l_h() {
        let bus = GameboyBus::new(vec![0x6c]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, H".to_string())))
    }

    #[test]
    fn ld_l_l() {
        let bus = GameboyBus::new(vec![0x6d]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, L".to_string())))
    }

    #[test]
    fn ld_l_hlr() {
        let bus = GameboyBus::new(vec![0x6e]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, [HL]".to_string())))
    }

    #[test]
    fn ld_l_a() {
        let bus = GameboyBus::new(vec![0x6f]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD L, A".to_string())))
    }

    // 0x7X
    #[test]
    fn ld_hlr_b() {
        let bus = GameboyBus::new(vec![0x70]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], B".to_string())))
    }

    #[test]
    fn ld_hlr_c() {
        let bus = GameboyBus::new(vec![0x71]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], C".to_string())))
    }

    #[test]
    fn ld_hlr_d() {
        let bus = GameboyBus::new(vec![0x72]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], D".to_string())))
    }

    #[test]
    fn ld_hlr_e() {
        let bus = GameboyBus::new(vec![0x73]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], E".to_string())))
    }

    #[test]
    fn ld_hlr_h() {
        let bus = GameboyBus::new(vec![0x74]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], H".to_string())))
    }

    #[test]
    fn ld_hlr_l() {
        let bus = GameboyBus::new(vec![0x75]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], L".to_string())))
    }

    #[test]
    fn halt() {
        let bus = GameboyBus::new(vec![0x76]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "HALT".to_string())))
    }

    #[test]
    fn ld_hlr_a() {
        let bus = GameboyBus::new(vec![0x77]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD [HL], A".to_string())))
    }

    #[test]
    fn ld_a_b() {
        let bus = GameboyBus::new(vec![0x78]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, B".to_string())))
    }

    #[test]
    fn ld_a_c() {
        let bus = GameboyBus::new(vec![0x79]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, C".to_string())))
    }

    #[test]
    fn ld_a_d() {
        let bus = GameboyBus::new(vec![0x7a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, D".to_string())))
    }

    #[test]
    fn ld_a_e() {
        let bus = GameboyBus::new(vec![0x7b]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, E".to_string())))
    }

    #[test]
    fn ld_a_h() {
        let bus = GameboyBus::new(vec![0x7c]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, H".to_string())))
    }

    #[test]
    fn ld_a_l() {
        let bus = GameboyBus::new(vec![0x7d]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, L".to_string())))
    }

    #[test]
    fn ld_a_hlr() {
        let bus = GameboyBus::new(vec![0x7e]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, [HL]".to_string())))
    }

    #[test]
    fn ld_a_a() {
        let bus = GameboyBus::new(vec![0x7f]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD A, A".to_string())))
    }

    // 0x8X
    #[test]
    fn add_a_b() {
        let bus = GameboyBus::new(vec![0x80]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, B".to_string())))
    }

    #[test]
    fn add_a_c() {
        let bus = GameboyBus::new(vec![0x81]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, C".to_string())))
    }

    #[test]
    fn add_a_d() {
        let bus = GameboyBus::new(vec![0x82]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, D".to_string())))
    }

    #[test]
    fn add_a_e() {
        let bus = GameboyBus::new(vec![0x83]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, E".to_string())))
    }

    #[test]
    fn add_a_h() {
        let bus = GameboyBus::new(vec![0x84]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, H".to_string())))
    }

    #[test]
    fn add_a_l() {
        let bus = GameboyBus::new(vec![0x85]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, L".to_string())))
    }

    #[test]
    fn add_a_hlr() {
        let bus = GameboyBus::new(vec![0x86]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, [HL]".to_string())))
    }

    #[test]
    fn add_a_a() {
        let bus = GameboyBus::new(vec![0x87]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADD A, A".to_string())))
    }

    #[test]
    fn adc_a_b() {
        let bus = GameboyBus::new(vec![0x88]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, B".to_string())))
    }

    #[test]
    fn adc_a_c() {
        let bus = GameboyBus::new(vec![0x89]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, C".to_string())))
    }

    #[test]
    fn adc_a_d() {
        let bus = GameboyBus::new(vec![0x8a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, D".to_string())))
    }

    #[test]
    fn adc_a_e() {
        let bus = GameboyBus::new(vec![0x8b]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, E".to_string())))
    }

    #[test]
    fn adc_a_h() {
        let bus = GameboyBus::new(vec![0x8c]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, H".to_string())))
    }

    #[test]
    fn adc_a_l() {
        let bus = GameboyBus::new(vec![0x8d]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, L".to_string())))
    }

    #[test]
    fn adc_a_hlr() {
        let bus = GameboyBus::new(vec![0x8e]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, [HL]".to_string())))
    }

    #[test]
    fn adc_a_a() {
        let bus = GameboyBus::new(vec![0x8f]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "ADC A, A".to_string())))
    }

    // 0x9X
    #[test]
    fn sub_a_b() {
        let bus = GameboyBus::new(vec![0x90]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, B".to_string())))
    }

    #[test]
    fn sub_a_c() {
        let bus = GameboyBus::new(vec![0x91]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, C".to_string())))
    }

    #[test]
    fn sub_a_d() {
        let bus = GameboyBus::new(vec![0x92]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, D".to_string())))
    }

    #[test]
    fn sub_a_e() {
        let bus = GameboyBus::new(vec![0x93]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, E".to_string())))
    }

    #[test]
    fn sub_a_h() {
        let bus = GameboyBus::new(vec![0x94]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, H".to_string())))
    }

    #[test]
    fn sub_a_l() {
        let bus = GameboyBus::new(vec![0x95]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, L".to_string())))
    }

    #[test]
    fn sub_a_hlr() {
        let bus = GameboyBus::new(vec![0x96]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, [HL]".to_string())))
    }

    #[test]
    fn sub_a_a() {
        let bus = GameboyBus::new(vec![0x97]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SUB A, A".to_string())))
    }

    #[test]
    fn sbc_a_b() {
        let bus = GameboyBus::new(vec![0x98]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, B".to_string())))
    }

    #[test]
    fn sbc_a_c() {
        let bus = GameboyBus::new(vec![0x99]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, C".to_string())))
    }

    #[test]
    fn sbc_a_d() {
        let bus = GameboyBus::new(vec![0x9a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, D".to_string())))
    }

    #[test]
    fn sbc_a_e() {
        let bus = GameboyBus::new(vec![0x9b]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, E".to_string())))
    }

    #[test]
    fn sbc_a_h() {
        let bus = GameboyBus::new(vec![0x9c]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, H".to_string())))
    }

    #[test]
    fn sbc_a_l() {
        let bus = GameboyBus::new(vec![0x9d]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, L".to_string())))
    }

    #[test]
    fn sbc_a_hlr() {
        let bus = GameboyBus::new(vec![0x9e]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, [HL]".to_string())))
    }

    #[test]
    fn sbc_a_a() {
        let bus = GameboyBus::new(vec![0x9f]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "SBC A, A".to_string())))
    }

    // 0xAX
    #[test]
    fn and_a_b() {
        let bus = GameboyBus::new(vec![0xA0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, B".to_string())))
    }

    #[test]
    fn and_a_c() {
        let bus = GameboyBus::new(vec![0xA1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, C".to_string())))
    }

    #[test]
    fn and_a_d() {
        let bus = GameboyBus::new(vec![0xA2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, D".to_string())))
    }

    #[test]
    fn and_a_e() {
        let bus = GameboyBus::new(vec![0xA3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, E".to_string())))
    }

    #[test]
    fn and_a_h() {
        let bus = GameboyBus::new(vec![0xA4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, H".to_string())))
    }

    #[test]
    fn and_a_l() {
        let bus = GameboyBus::new(vec![0xA5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, L".to_string())))
    }

    #[test]
    fn and_a_hlr() {
        let bus = GameboyBus::new(vec![0xA6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, [HL]".to_string())))
    }

    #[test]
    fn and_a_a() {
        let bus = GameboyBus::new(vec![0xA7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "AND A, A".to_string())))
    }

    #[test]
    fn xor_a_b() {
        let bus = GameboyBus::new(vec![0xA8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, B".to_string())))
    }

    #[test]
    fn xor_a_c() {
        let bus = GameboyBus::new(vec![0xA9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, C".to_string())))
    }

    #[test]
    fn xor_a_d() {
        let bus = GameboyBus::new(vec![0xAa]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, D".to_string())))
    }

    #[test]
    fn xor_a_e() {
        let bus = GameboyBus::new(vec![0xAb]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, E".to_string())))
    }

    #[test]
    fn xor_a_h() {
        let bus = GameboyBus::new(vec![0xAc]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, H".to_string())))
    }

    #[test]
    fn xor_a_l() {
        let bus = GameboyBus::new(vec![0xAd]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, L".to_string())))
    }

    #[test]
    fn xor_a_hlr() {
        let bus = GameboyBus::new(vec![0xAe]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, [HL]".to_string())))
    }

    #[test]
    fn xor_a_a() {
        let bus = GameboyBus::new(vec![0xAf]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "XOR A, A".to_string())))
    }

    // 0xBX
    #[test]
    fn or_a_b() {
        let bus = GameboyBus::new(vec![0xB0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, B".to_string())))
    }

    #[test]
    fn or_a_c() {
        let bus = GameboyBus::new(vec![0xB1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, C".to_string())))
    }

    #[test]
    fn or_a_d() {
        let bus = GameboyBus::new(vec![0xB2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, D".to_string())))
    }

    #[test]
    fn or_a_e() {
        let bus = GameboyBus::new(vec![0xB3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, E".to_string())))
    }

    #[test]
    fn or_a_h() {
        let bus = GameboyBus::new(vec![0xB4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, H".to_string())))
    }

    #[test]
    fn or_a_l() {
        let bus = GameboyBus::new(vec![0xB5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, L".to_string())))
    }

    #[test]
    fn or_a_hlr() {
        let bus = GameboyBus::new(vec![0xB6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, [HL]".to_string())))
    }

    #[test]
    fn or_a_a() {
        let bus = GameboyBus::new(vec![0xB7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "OR A, A".to_string())))
    }

    #[test]
    fn cp_a_b() {
        let bus = GameboyBus::new(vec![0xB8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, B".to_string())))
    }

    #[test]
    fn cp_a_c() {
        let bus = GameboyBus::new(vec![0xB9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, C".to_string())))
    }

    #[test]
    fn cp_a_d() {
        let bus = GameboyBus::new(vec![0xBa]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, D".to_string())))
    }

    #[test]
    fn cp_a_e() {
        let bus = GameboyBus::new(vec![0xBb]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, E".to_string())))
    }

    #[test]
    fn cp_a_h() {
        let bus = GameboyBus::new(vec![0xBc]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, H".to_string())))
    }

    #[test]
    fn cp_a_l() {
        let bus = GameboyBus::new(vec![0xBd]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, L".to_string())))
    }

    #[test]
    fn cp_a_hlr() {
        let bus = GameboyBus::new(vec![0xBe]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, [HL]".to_string())))
    }

    #[test]
    fn cp_a_a() {
        let bus = GameboyBus::new(vec![0xBf]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "CP A, A".to_string())))
    }

    // 0xCX
    #[test]
    fn ret_nz() {
        let bus = GameboyBus::new(vec![0xC0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RET NZ".to_string())))
    }

    #[test]
    fn pop_bc() {
        let bus = GameboyBus::new(vec![0xC1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "POP BC".to_string())))
    }

    #[test]
    fn jp_nz_n16() {
        let bus = GameboyBus::new(vec![0xC2, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "JP NZ, $1234".to_string())))
    }

    #[test]
    fn jp_n16() {
        let bus = GameboyBus::new(vec![0xC3, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "JP $1234".to_string())))
    }

    #[test]
    fn call_nz_n16() {
        let bus = GameboyBus::new(vec![0xC4, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "CALL NZ, $1234".to_string())))
    }

    #[test]
    fn push_bc() {
        let bus = GameboyBus::new(vec![0xC5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "PUSH BC".to_string())))
    }

    #[test]
    fn add_a_n8() {
        let bus = GameboyBus::new(vec![0xC6, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "ADD A, $2A".to_string())))
    }

    #[test]
    fn rst_00() {
        let bus = GameboyBus::new(vec![0xC7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $00".to_string())))
    }

    #[test]
    fn ret_z() {
        let bus = GameboyBus::new(vec![0xC8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RET Z".to_string())))
    }

    #[test]
    fn ret() {
        let bus = GameboyBus::new(vec![0xC9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RET".to_string())))
    }

    #[test]
    fn jp_z_n16() {
        let bus = GameboyBus::new(vec![0xCA, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "JP Z, $1234".to_string())))
    }

    #[test]
    fn call_z_n16() {
        let bus = GameboyBus::new(vec![0xCC, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "CALL Z, $1234".to_string())))
    }

    #[test]
    fn call_n16() {
        let bus = GameboyBus::new(vec![0xCD, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "CALL $1234".to_string())))
    }

    #[test]
    fn adc_a_n8() {
        let bus = GameboyBus::new(vec![0xCE, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "ADC A, $2A".to_string())))
    }

    #[test]
    fn rst_08() {
        let bus = GameboyBus::new(vec![0xCF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $08".to_string())))
    }

    // 0xDX
    #[test]
    fn ret_nc() {
        let bus = GameboyBus::new(vec![0xD0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RET NC".to_string())))
    }

    #[test]
    fn pop_de() {
        let bus = GameboyBus::new(vec![0xD1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "POP DE".to_string())))
    }

    #[test]
    fn jp_nc_n16() {
        let bus = GameboyBus::new(vec![0xD2, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "JP NC, $1234".to_string())))
    }

    #[test]
    fn call_nc_n16() {
        let bus = GameboyBus::new(vec![0xD4, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "CALL NC, $1234".to_string())))
    }

    #[test]
    fn push_de() {
        let bus = GameboyBus::new(vec![0xD5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "PUSH DE".to_string())))
    }

    #[test]
    fn sub_a_n8() {
        let bus = GameboyBus::new(vec![0xD6, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SUB A, $2A".to_string())))
    }

    #[test]
    fn rst_10() {
        let bus = GameboyBus::new(vec![0xD7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $10".to_string())))
    }

    #[test]
    fn ret_c() {
        let bus = GameboyBus::new(vec![0xD8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RET C".to_string())))
    }

    #[test]
    fn reti() {
        let bus = GameboyBus::new(vec![0xD9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RETI".to_string())))
    }

    #[test]
    fn jp_c_n16() {
        let bus = GameboyBus::new(vec![0xDA, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "JP C, $1234".to_string())))
    }

    #[test]
    fn call_c_n16() {
        let bus = GameboyBus::new(vec![0xDC, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "CALL C, $1234".to_string())))
    }

    #[test]
    fn sbc_a_n8() {
        let bus = GameboyBus::new(vec![0xDE, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SBC A, $2A".to_string())))
    }

    #[test]
    fn rst_18() {
        let bus = GameboyBus::new(vec![0xDF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $18".to_string())))
    }

    // 0xEX
    #[test]
    fn ldh_n8_a() {
        let bus = GameboyBus::new(vec![0xE0, 0xDE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LDH [$FFDE], A".to_string())))
    }

    #[test]
    fn pop_hl() {
        let bus = GameboyBus::new(vec![0xE1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "POP HL".to_string())))
    }

    #[test]
    fn ldh_cr_a() {
        let bus = GameboyBus::new(vec![0xE2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LDH [C], A".to_string())))
    }

    #[test]
    fn push_hl() {
        let bus = GameboyBus::new(vec![0xE5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "PUSH HL".to_string())))
    }

    #[test]
    fn and_a_n8() {
        let bus = GameboyBus::new(vec![0xE6, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "AND A, $2A".to_string())))
    }

    #[test]
    fn rst_20() {
        let bus = GameboyBus::new(vec![0xE7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $20".to_string())))
    }

    #[test]
    fn add_sp_i8_neg() {
        let bus = GameboyBus::new(vec![0xE8, 0xFD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "ADD SP, -3".to_string())))
    }

    #[test]
    fn add_sp_i8_pos() {
        let bus = GameboyBus::new(vec![0xE8, 0x02]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "ADD SP, 2".to_string())))
    }

    #[test]
    fn jp_hl() {
        let bus = GameboyBus::new(vec![0xE9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "JP HL".to_string())))
    }

    #[test]
    fn ld_n16r_a() {
        let bus = GameboyBus::new(vec![0xEA, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "LD [$1234], A".to_string())))
    }

    #[test]
    fn xor_a_n8() {
        let bus = GameboyBus::new(vec![0xEE, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "XOR A, $2A".to_string())))
    }

    #[test]
    fn rst_28() {
        let bus = GameboyBus::new(vec![0xEF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $28".to_string())))
    }

    // 0xFX
    #[test]
    fn ldh_a_n8() {
        let bus = GameboyBus::new(vec![0xF0, 0xDE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LDH A, [$FFDE]".to_string())))
    }

    #[test]
    fn pop_af() {
        let bus = GameboyBus::new(vec![0xF1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "POP AF".to_string())))
    }

    #[test]
    fn ldh_a_cr() {
        let bus = GameboyBus::new(vec![0xF2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LDH A, [C]".to_string())))
    }

    #[test]
    fn di() {
        let bus = GameboyBus::new(vec![0xF3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "DI".to_string())))
    }

    #[test]
    fn push_af() {
        let bus = GameboyBus::new(vec![0xF5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "PUSH AF".to_string())))
    }

    #[test]
    fn or_a_n8() {
        let bus = GameboyBus::new(vec![0xF6, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "OR A, $2A".to_string())))
    }

    #[test]
    fn rst_30() {
        let bus = GameboyBus::new(vec![0xF7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $30".to_string())))
    }

    #[test]
    fn ld_sp_i8_neg() {
        let bus = GameboyBus::new(vec![0xF8, 0xFD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD HL, SP-3".to_string())))
    }

    #[test]
    fn ld_sp_i8_pos() {
        let bus = GameboyBus::new(vec![0xF8, 0x02]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "LD HL, SP+2".to_string())))
    }

    #[test]
    fn ld_sp_hl() {
        let bus = GameboyBus::new(vec![0xF9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "LD SP, HL".to_string())))
    }

    #[test]
    fn ld_a_n16r() {
        let bus = GameboyBus::new(vec![0xFA, 0x12, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((3, "LD A, [$1234]".to_string())))
    }

    #[test]
    fn ei() {
        let bus = GameboyBus::new(vec![0xFB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "EI".to_string())))
    }

    #[test]
    fn cp_a_n8() {
        let bus = GameboyBus::new(vec![0xFE, 0x2a]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "CP A, $2A".to_string())))
    }

    #[test]
    fn rst_38() {
        let bus = GameboyBus::new(vec![0xFF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((1, "RST $38".to_string())))
    }

    // 0xCB-prefix
    // 0x0X
    #[test]
    fn rlc_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x00]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC B".to_string())))
    }

    #[test]
    fn rlc_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x01]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC C".to_string())))
    }

    #[test]
    fn rlc_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x02]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC D".to_string())))
    }

    #[test]
    fn rlc_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x03]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC E".to_string())))
    }

    #[test]
    fn rlc_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x04]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC H".to_string())))
    }

    #[test]
    fn rlc_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x05]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC L".to_string())))
    }

    #[test]
    fn rlc_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x06]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC [HL]".to_string())))
    }

    #[test]
    fn rlc_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x07]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RLC A".to_string())))
    }

    #[test]
    fn rrc_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x08]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC B".to_string())))
    }

    #[test]
    fn rrc_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x09]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC C".to_string())))
    }

    #[test]
    fn rrc_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x0A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC D".to_string())))
    }

    #[test]
    fn rrc_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x0B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC E".to_string())))
    }

    #[test]
    fn rrc_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x0C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC H".to_string())))
    }

    #[test]
    fn rrc_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x0D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC L".to_string())))
    }

    #[test]
    fn rrc_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x0E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC [HL]".to_string())))
    }

    #[test]
    fn rrc_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x0F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RRC A".to_string())))
    }

    // 0x1X
    #[test]
    fn rl_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x10]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL B".to_string())))
    }

    #[test]
    fn rl_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x11]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL C".to_string())))
    }

    #[test]
    fn rl_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x12]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL D".to_string())))
    }

    #[test]
    fn rl_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x13]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL E".to_string())))
    }

    #[test]
    fn rl_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x14]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL H".to_string())))
    }

    #[test]
    fn rl_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x15]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL L".to_string())))
    }

    #[test]
    fn rl_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x16]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL [HL]".to_string())))
    }

    #[test]
    fn rl_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x17]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RL A".to_string())))
    }

    #[test]
    fn rr_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x18]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR B".to_string())))
    }

    #[test]
    fn rr_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x19]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR C".to_string())))
    }

    #[test]
    fn rr_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x1A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR D".to_string())))
    }

    #[test]
    fn rr_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x1B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR E".to_string())))
    }

    #[test]
    fn rr_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x1C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR H".to_string())))
    }

    #[test]
    fn rr_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x1D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR L".to_string())))
    }

    #[test]
    fn rr_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x1E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR [HL]".to_string())))
    }

    #[test]
    fn rr_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x1F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RR A".to_string())))
    }

    // 0x2X
    #[test]
    fn sla_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x20]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA B".to_string())))
    }

    #[test]
    fn sla_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x21]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA C".to_string())))
    }

    #[test]
    fn sla_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x22]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA D".to_string())))
    }

    #[test]
    fn sla_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x23]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA E".to_string())))
    }

    #[test]
    fn sla_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x24]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA H".to_string())))
    }

    #[test]
    fn sla_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x25]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA L".to_string())))
    }

    #[test]
    fn sla_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x26]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA [HL]".to_string())))
    }

    #[test]
    fn sla_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x27]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SLA A".to_string())))
    }

    #[test]
    fn sra_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x28]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA B".to_string())))
    }

    #[test]
    fn sra_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x29]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA C".to_string())))
    }

    #[test]
    fn sra_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x2A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA D".to_string())))
    }

    #[test]
    fn sra_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x2B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA E".to_string())))
    }

    #[test]
    fn sra_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x2C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA H".to_string())))
    }

    #[test]
    fn sra_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x2D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA L".to_string())))
    }

    #[test]
    fn sra_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x2E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA [HL]".to_string())))
    }

    #[test]
    fn sra_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x2F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRA A".to_string())))
    }

    // 0x3X
    #[test]
    fn swap_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x30]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP B".to_string())))
    }

    #[test]
    fn swap_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x31]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP C".to_string())))
    }

    #[test]
    fn swap_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x32]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP D".to_string())))
    }

    #[test]
    fn swap_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x33]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP E".to_string())))
    }

    #[test]
    fn swap_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x34]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP H".to_string())))
    }

    #[test]
    fn swap_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x35]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP L".to_string())))
    }

    #[test]
    fn swap_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x36]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP [HL]".to_string())))
    }

    #[test]
    fn swap_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x37]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SWAP A".to_string())))
    }

    #[test]
    fn srl_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x38]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL B".to_string())))
    }

    #[test]
    fn srl_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x39]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL C".to_string())))
    }

    #[test]
    fn srl_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x3A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL D".to_string())))
    }

    #[test]
    fn srl_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x3B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL E".to_string())))
    }

    #[test]
    fn srl_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x3C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL H".to_string())))
    }

    #[test]
    fn srl_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x3D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL L".to_string())))
    }

    #[test]
    fn srl_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x3E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL [HL]".to_string())))
    }

    #[test]
    fn srl_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x3F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SRL A".to_string())))
    }

    // 0x4X
    #[test]
    fn bit_0_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x40]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, B".to_string())))
    }

    #[test]
    fn bit_0_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x41]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, C".to_string())))
    }

    #[test]
    fn bit_0_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x42]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, D".to_string())))
    }

    #[test]
    fn bit_0_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x43]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, E".to_string())))
    }

    #[test]
    fn bit_0_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x44]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, H".to_string())))
    }

    #[test]
    fn bit_0_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x45]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, L".to_string())))
    }

    #[test]
    fn bit_0_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x46]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, [HL]".to_string())))
    }

    #[test]
    fn bit_0_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x47]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 0, A".to_string())))
    }

    #[test]
    fn bit_1_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x48]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, B".to_string())))
    }

    #[test]
    fn bit_1_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x49]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, C".to_string())))
    }

    #[test]
    fn bit_1_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x4A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, D".to_string())))
    }

    #[test]
    fn bit_1_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x4B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, E".to_string())))
    }

    #[test]
    fn bit_1_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x4C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, H".to_string())))
    }

    #[test]
    fn bit_1_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x4D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, L".to_string())))
    }

    #[test]
    fn bit_1_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x4E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, [HL]".to_string())))
    }

    #[test]
    fn bit_1_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x4F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 1, A".to_string())))
    }

    // 0x5X
    #[test]
    fn bit_2_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x50]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, B".to_string())))
    }

    #[test]
    fn bit_2_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x51]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, C".to_string())))
    }

    #[test]
    fn bit_2_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x52]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, D".to_string())))
    }

    #[test]
    fn bit_2_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x53]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, E".to_string())))
    }

    #[test]
    fn bit_2_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x54]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, H".to_string())))
    }

    #[test]
    fn bit_2_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x55]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, L".to_string())))
    }

    #[test]
    fn bit_2_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x56]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, [HL]".to_string())))
    }

    #[test]
    fn bit_2_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x57]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 2, A".to_string())))
    }

    #[test]
    fn bit_3_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x58]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, B".to_string())))
    }

    #[test]
    fn bit_3_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x59]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, C".to_string())))
    }

    #[test]
    fn bit_3_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x5A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, D".to_string())))
    }

    #[test]
    fn bit_3_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x5B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, E".to_string())))
    }

    #[test]
    fn bit_3_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x5C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, H".to_string())))
    }

    #[test]
    fn bit_3_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x5D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, L".to_string())))
    }

    #[test]
    fn bit_3_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x5E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, [HL]".to_string())))
    }

    #[test]
    fn bit_3_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x5F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 3, A".to_string())))
    }

    // 0x6X
    #[test]
    fn bit_4_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x60]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, B".to_string())))
    }

    #[test]
    fn bit_4_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x61]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, C".to_string())))
    }

    #[test]
    fn bit_4_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x62]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, D".to_string())))
    }

    #[test]
    fn bit_4_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x63]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, E".to_string())))
    }

    #[test]
    fn bit_4_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x64]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, H".to_string())))
    }

    #[test]
    fn bit_4_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x65]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, L".to_string())))
    }

    #[test]
    fn bit_4_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x66]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, [HL]".to_string())))
    }

    #[test]
    fn bit_4_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x67]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 4, A".to_string())))
    }

    #[test]
    fn bit_5_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x68]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, B".to_string())))
    }

    #[test]
    fn bit_5_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x69]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, C".to_string())))
    }

    #[test]
    fn bit_5_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x6A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, D".to_string())))
    }

    #[test]
    fn bit_5_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x6B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, E".to_string())))
    }

    #[test]
    fn bit_5_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x6C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, H".to_string())))
    }

    #[test]
    fn bit_5_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x6D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, L".to_string())))
    }

    #[test]
    fn bit_5_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x6E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, [HL]".to_string())))
    }

    #[test]
    fn bit_5_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x6F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 5, A".to_string())))
    }

    // 0x7X
    #[test]
    fn bit_6_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x70]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, B".to_string())))
    }

    #[test]
    fn bit_6_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x71]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, C".to_string())))
    }

    #[test]
    fn bit_6_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x72]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, D".to_string())))
    }

    #[test]
    fn bit_6_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x73]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, E".to_string())))
    }

    #[test]
    fn bit_6_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x74]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, H".to_string())))
    }

    #[test]
    fn bit_6_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x75]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, L".to_string())))
    }

    #[test]
    fn bit_6_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x76]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, [HL]".to_string())))
    }

    #[test]
    fn bit_6_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x77]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 6, A".to_string())))
    }

    #[test]
    fn bit_7_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x78]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, B".to_string())))
    }

    #[test]
    fn bit_7_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x79]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, C".to_string())))
    }

    #[test]
    fn bit_7_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x7A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, D".to_string())))
    }

    #[test]
    fn bit_7_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x7B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, E".to_string())))
    }

    #[test]
    fn bit_7_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x7C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, H".to_string())))
    }

    #[test]
    fn bit_7_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x7D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, L".to_string())))
    }

    #[test]
    fn bit_7_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x7E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, [HL]".to_string())))
    }

    #[test]
    fn bit_7_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x7F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "BIT 7, A".to_string())))
    }

    // 0x8X
    #[test]
    fn res_0_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x80]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, B".to_string())))
    }

    #[test]
    fn res_0_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x81]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, C".to_string())))
    }

    #[test]
    fn res_0_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x82]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, D".to_string())))
    }

    #[test]
    fn res_0_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x83]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, E".to_string())))
    }

    #[test]
    fn res_0_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x84]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, H".to_string())))
    }

    #[test]
    fn res_0_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x85]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, L".to_string())))
    }

    #[test]
    fn res_0_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x86]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, [HL]".to_string())))
    }

    #[test]
    fn res_0_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x87]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 0, A".to_string())))
    }

    #[test]
    fn res_1_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x88]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, B".to_string())))
    }

    #[test]
    fn res_1_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x89]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, C".to_string())))
    }

    #[test]
    fn res_1_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x8A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, D".to_string())))
    }

    #[test]
    fn res_1_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x8B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, E".to_string())))
    }

    #[test]
    fn res_1_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x8C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, H".to_string())))
    }

    #[test]
    fn res_1_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x8D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, L".to_string())))
    }

    #[test]
    fn res_1_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x8E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, [HL]".to_string())))
    }

    #[test]
    fn res_1_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x8F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 1, A".to_string())))
    }

    // 0x9X
    #[test]
    fn res_2_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x90]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, B".to_string())))
    }

    #[test]
    fn res_2_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x91]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, C".to_string())))
    }

    #[test]
    fn res_2_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x92]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, D".to_string())))
    }

    #[test]
    fn res_2_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x93]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, E".to_string())))
    }

    #[test]
    fn res_2_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x94]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, H".to_string())))
    }

    #[test]
    fn res_2_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x95]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, L".to_string())))
    }

    #[test]
    fn res_2_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x96]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, [HL]".to_string())))
    }

    #[test]
    fn res_2_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x97]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 2, A".to_string())))
    }

    #[test]
    fn res_3_b() {
        let bus = GameboyBus::new(vec![0xCB, 0x98]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, B".to_string())))
    }

    #[test]
    fn res_3_c() {
        let bus = GameboyBus::new(vec![0xCB, 0x99]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, C".to_string())))
    }

    #[test]
    fn res_3_d() {
        let bus = GameboyBus::new(vec![0xCB, 0x9A]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, D".to_string())))
    }

    #[test]
    fn res_3_e() {
        let bus = GameboyBus::new(vec![0xCB, 0x9B]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, E".to_string())))
    }

    #[test]
    fn res_3_h() {
        let bus = GameboyBus::new(vec![0xCB, 0x9C]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, H".to_string())))
    }

    #[test]
    fn res_3_l() {
        let bus = GameboyBus::new(vec![0xCB, 0x9D]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, L".to_string())))
    }

    #[test]
    fn res_3_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0x9E]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, [HL]".to_string())))
    }

    #[test]
    fn res_3_a() {
        let bus = GameboyBus::new(vec![0xCB, 0x9F]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 3, A".to_string())))
    }

    // 0xAX
    #[test]
    fn res_4_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xA0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, B".to_string())))
    }

    #[test]
    fn res_4_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xA1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, C".to_string())))
    }

    #[test]
    fn res_4_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xA2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, D".to_string())))
    }

    #[test]
    fn res_4_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xA3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, E".to_string())))
    }

    #[test]
    fn res_4_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xA4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, H".to_string())))
    }

    #[test]
    fn res_4_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xA5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, L".to_string())))
    }

    #[test]
    fn res_4_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xA6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, [HL]".to_string())))
    }

    #[test]
    fn res_4_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xA7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 4, A".to_string())))
    }

    #[test]
    fn res_5_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xA8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, B".to_string())))
    }

    #[test]
    fn res_5_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xA9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, C".to_string())))
    }

    #[test]
    fn res_5_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xAA]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, D".to_string())))
    }

    #[test]
    fn res_5_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xAB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, E".to_string())))
    }

    #[test]
    fn res_5_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xAC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, H".to_string())))
    }

    #[test]
    fn res_5_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xAD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, L".to_string())))
    }

    #[test]
    fn res_5_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xAE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, [HL]".to_string())))
    }

    #[test]
    fn res_5_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xAF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 5, A".to_string())))
    }

    // 0xBX
    #[test]
    fn res_6_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xB0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, B".to_string())))
    }

    #[test]
    fn res_6_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xB1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, C".to_string())))
    }

    #[test]
    fn res_6_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xB2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, D".to_string())))
    }

    #[test]
    fn res_6_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xB3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, E".to_string())))
    }

    #[test]
    fn res_6_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xB4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, H".to_string())))
    }

    #[test]
    fn res_6_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xB5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, L".to_string())))
    }

    #[test]
    fn res_6_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xB6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, [HL]".to_string())))
    }

    #[test]
    fn res_6_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xB7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 6, A".to_string())))
    }

    #[test]
    fn res_7_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xB8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, B".to_string())))
    }

    #[test]
    fn res_7_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xB9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, C".to_string())))
    }

    #[test]
    fn res_7_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xBA]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, D".to_string())))
    }

    #[test]
    fn res_7_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xBB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, E".to_string())))
    }

    #[test]
    fn res_7_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xBC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, H".to_string())))
    }

    #[test]
    fn res_7_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xBD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, L".to_string())))
    }

    #[test]
    fn res_7_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xBE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, [HL]".to_string())))
    }

    #[test]
    fn res_7_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xBF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "RES 7, A".to_string())))
    }

    // 0xCX
    #[test]
    fn set_0_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xC0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, B".to_string())))
    }

    #[test]
    fn set_0_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xC1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, C".to_string())))
    }

    #[test]
    fn set_0_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xC2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, D".to_string())))
    }

    #[test]
    fn set_0_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xC3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, E".to_string())))
    }

    #[test]
    fn set_0_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xC4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, H".to_string())))
    }

    #[test]
    fn set_0_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xC5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, L".to_string())))
    }

    #[test]
    fn set_0_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xC6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, [HL]".to_string())))
    }

    #[test]
    fn set_0_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xC7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 0, A".to_string())))
    }

    #[test]
    fn set_1_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xC8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, B".to_string())))
    }

    #[test]
    fn set_1_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xC9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, C".to_string())))
    }

    #[test]
    fn set_1_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xCA]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, D".to_string())))
    }

    #[test]
    fn set_1_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xCB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, E".to_string())))
    }

    #[test]
    fn set_1_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xCC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, H".to_string())))
    }

    #[test]
    fn set_1_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xCD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, L".to_string())))
    }

    #[test]
    fn set_1_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xCE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, [HL]".to_string())))
    }

    #[test]
    fn set_1_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xCF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 1, A".to_string())))
    }

    // 0xDX
    #[test]
    fn set_2_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xD0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, B".to_string())))
    }

    #[test]
    fn set_2_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xD1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, C".to_string())))
    }

    #[test]
    fn set_2_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xD2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, D".to_string())))
    }

    #[test]
    fn set_2_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xD3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, E".to_string())))
    }

    #[test]
    fn set_2_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xD4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, H".to_string())))
    }

    #[test]
    fn set_2_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xD5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, L".to_string())))
    }

    #[test]
    fn set_2_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xD6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, [HL]".to_string())))
    }

    #[test]
    fn set_2_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xD7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 2, A".to_string())))
    }

    #[test]
    fn set_3_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xD8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, B".to_string())))
    }

    #[test]
    fn set_3_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xD9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, C".to_string())))
    }

    #[test]
    fn set_3_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xDA]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, D".to_string())))
    }

    #[test]
    fn set_3_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xDB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, E".to_string())))
    }

    #[test]
    fn set_3_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xDC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, H".to_string())))
    }

    #[test]
    fn set_3_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xDD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, L".to_string())))
    }

    #[test]
    fn set_3_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xDE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, [HL]".to_string())))
    }

    #[test]
    fn set_3_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xDF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 3, A".to_string())))
    }

    // 0xEX
    #[test]
    fn set_4_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xE0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, B".to_string())))
    }

    #[test]
    fn set_4_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xE1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, C".to_string())))
    }

    #[test]
    fn set_4_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xE2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, D".to_string())))
    }

    #[test]
    fn set_4_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xE3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, E".to_string())))
    }

    #[test]
    fn set_4_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xE4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, H".to_string())))
    }

    #[test]
    fn set_4_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xE5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, L".to_string())))
    }

    #[test]
    fn set_4_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xE6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, [HL]".to_string())))
    }

    #[test]
    fn set_4_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xE7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 4, A".to_string())))
    }

    #[test]
    fn set_5_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xE8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, B".to_string())))
    }

    #[test]
    fn set_5_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xE9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, C".to_string())))
    }

    #[test]
    fn set_5_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xEA]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, D".to_string())))
    }

    #[test]
    fn set_5_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xEB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, E".to_string())))
    }

    #[test]
    fn set_5_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xEC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, H".to_string())))
    }

    #[test]
    fn set_5_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xED]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, L".to_string())))
    }

    #[test]
    fn set_5_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xEE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, [HL]".to_string())))
    }

    #[test]
    fn set_5_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xEF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 5, A".to_string())))
    }

    // 0xFX
    #[test]
    fn set_6_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xF0]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, B".to_string())))
    }

    #[test]
    fn set_6_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xF1]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, C".to_string())))
    }

    #[test]
    fn set_6_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xF2]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, D".to_string())))
    }

    #[test]
    fn set_6_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xF3]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, E".to_string())))
    }

    #[test]
    fn set_6_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xF4]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, H".to_string())))
    }

    #[test]
    fn set_6_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xF5]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, L".to_string())))
    }

    #[test]
    fn set_6_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xF6]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, [HL]".to_string())))
    }

    #[test]
    fn set_6_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xF7]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 6, A".to_string())))
    }

    #[test]
    fn set_7_b() {
        let bus = GameboyBus::new(vec![0xCB, 0xF8]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, B".to_string())))
    }

    #[test]
    fn set_7_c() {
        let bus = GameboyBus::new(vec![0xCB, 0xF9]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, C".to_string())))
    }

    #[test]
    fn set_7_d() {
        let bus = GameboyBus::new(vec![0xCB, 0xFA]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, D".to_string())))
    }

    #[test]
    fn set_7_e() {
        let bus = GameboyBus::new(vec![0xCB, 0xFB]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, E".to_string())))
    }

    #[test]
    fn set_7_h() {
        let bus = GameboyBus::new(vec![0xCB, 0xFC]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, H".to_string())))
    }

    #[test]
    fn set_7_l() {
        let bus = GameboyBus::new(vec![0xCB, 0xFD]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, L".to_string())))
    }

    #[test]
    fn set_7_hlr() {
        let bus = GameboyBus::new(vec![0xCB, 0xFE]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, [HL]".to_string())))
    }

    #[test]
    fn set_7_a() {
        let bus = GameboyBus::new(vec![0xCB, 0xFF]);
        let prefs = Preferences{upcase: true, comma_space: true};
        let result = disass(&bus, 0x00, &prefs);

        assert_eq!(result, Ok((2, "SET 7, A".to_string())))
    }
}
