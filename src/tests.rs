use super::*;

// 0x0X
#[test]
fn nop() {
    let bus = SimpleBus::new(vec![0x00]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x0, &prefs);

    assert_eq!(result, Ok((1, "NOP".to_string())))
}

#[test]
fn ld_bc_n16() {
    let bus = SimpleBus::new(vec![0x01, 0xcd, 0xab]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x0, &prefs);

    assert_eq!(result, Ok((3, "LD BC, $ABCD".to_string())))
}

#[test]
fn ld_bc_missing_byte() {
    let bus = SimpleBus::new(vec![0x01, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x0, &prefs);

    assert!(result.is_err())
}

#[test]
fn ld_bc_missing_word() {
    let bus = SimpleBus::new(vec![0x01]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x0, &prefs);

    assert!(result.is_err())
}

#[test]
fn ld_bc_n16r_lower() {
    let bus = SimpleBus::new(vec![0x01, 0xcd, 0xab]);
    let prefs = Preferences{upcase: false, comma_space: true};
    let result = disassemble(&bus, 0x0, &prefs);

    assert_eq!(result, Ok((3, "ld bc, $abcd".to_string())))
}

#[test]
fn ld_bc_n16r_no_space() {
    let bus = SimpleBus::new(vec![0x01, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: false};
    let result = disassemble(&bus, 0x0, &prefs);

    assert_eq!(result, Ok((3, "LD BC,$1234".to_string())))
}

#[test]
fn ld_bc_n16r_lower_no_space() {
    let bus = SimpleBus::new(vec![0x01, 0x34, 0x12]);
    let prefs = Preferences{upcase: false, comma_space: false};
    let result = disassemble(&bus, 0x0, &prefs);

    assert_eq!(result, Ok((3, "ld bc,$1234".to_string())))
}

#[test]
fn ld_bcr_a() {
    let bus = SimpleBus::new(vec![0x02]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD [BC], A".to_string())))
}

#[test]
fn inc_bc() {
    let bus = SimpleBus::new(vec![0x03]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "INC BC".to_string())))
}

#[test]
fn inc_b() {
    let bus = SimpleBus::new(vec![0x04]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "INC B".to_string())))
}

#[test]
fn dec_b() {
    let bus = SimpleBus::new(vec![0x05]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DEC B".to_string())))
}

#[test]
fn ld_b_n8() {
    let bus = SimpleBus::new(vec![0x06, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "LD B, $2A".to_string())))
}

#[test]
fn rlca() {
    let bus = SimpleBus::new(vec![0x07]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RLCA".to_string())))
}

#[test]
fn ld_n16r_sp() {
    let bus = SimpleBus::new(vec![0x08, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((3, "LD [$1234], SP".to_string())))
}

#[test]
fn add_hl_bc() {
    let bus = SimpleBus::new(vec![0x09]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADD HL, BC".to_string())))
}

#[test]
fn ld_a_bcr() {
    let bus = SimpleBus::new(vec![0x0A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD A, [BC]".to_string())))
}

#[test]
fn dec_bc() {
    let bus = SimpleBus::new(vec![0x0B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DEC BC".to_string())))
}

#[test]
fn inc_c() {
    let bus = SimpleBus::new(vec![0x0C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "INC C".to_string())))
}

#[test]
fn dec_c() {
    let bus = SimpleBus::new(vec![0x0D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DEC C".to_string())))
}

#[test]
fn ld_c_n8() {
    let bus = SimpleBus::new(vec![0x0E, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "LD C, $2A".to_string())))
}

#[test]
fn rrca() {
    let bus = SimpleBus::new(vec![0x0F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RRCA".to_string())))
}

// 0x1X
#[test]
fn stop() {
    let bus = SimpleBus::new(vec![0x10]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x0, &prefs);

    assert_eq!(result, Ok((1, "STOP".to_string())))
}

#[test]
fn ld_de_n16() {
    let bus = SimpleBus::new(vec![0x11, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x0, &prefs);

    assert_eq!(result, Ok((3, "LD DE, $1234".to_string())))
}

#[test]
fn ld_der_a() {
    let bus = SimpleBus::new(vec![0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD [DE], A".to_string())))
}

#[test]
fn inc_de() {
    let bus = SimpleBus::new(vec![0x13]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "INC DE".to_string())))
}

#[test]
fn inc_d() {
    let bus = SimpleBus::new(vec![0x14]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "INC D".to_string())))
}

#[test]
fn dec_d() {
    let bus = SimpleBus::new(vec![0x15]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DEC D".to_string())))
}

#[test]
fn ld_d_n8() {
    let bus = SimpleBus::new(vec![0x16, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "LD D, $2A".to_string())))
}

#[test]
fn rla() {
    let bus = SimpleBus::new(vec![0x17]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RLA".to_string())))
}

#[test]
fn jr_i8() {
    let bus = SimpleBus::new(vec![0x18, 0xFC]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "JR -4".to_string())))
}

#[test]
fn add_hl_de() {
    let bus = SimpleBus::new(vec![0x19]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADD HL, DE".to_string())))
}

#[test]
fn ld_a_der() {
    let bus = SimpleBus::new(vec![0x1A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD A, [DE]".to_string())))
}

#[test]
fn dec_de() {
    let bus = SimpleBus::new(vec![0x1B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DEC DE".to_string())))
}

#[test]
fn inc_e() {
    let bus = SimpleBus::new(vec![0x1C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "INC E".to_string())))
}

#[test]
fn dec_e() {
    let bus = SimpleBus::new(vec![0x1D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DEC E".to_string())))
}

#[test]
fn ld_e_n8() {
    let bus = SimpleBus::new(vec![0x1E, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "LD E, $2A".to_string())))
}

#[test]
fn rra() {
    let bus = SimpleBus::new(vec![0x1F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RRA".to_string())))
}

// 0x2X
#[test]
fn jr_nz_i8() {
    let bus = SimpleBus::new(vec![0x20, 0xFC]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x0, &prefs);

    assert_eq!(result, Ok((2, "JR NZ, -4".to_string())))
}

#[test]
fn ld_hl_n16() {
    let bus = SimpleBus::new(vec![0x21, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x0, &prefs);

    assert_eq!(result, Ok((3, "LD HL, $1234".to_string())))
}

#[test]
fn ld_hli_a() {
    let bus = SimpleBus::new(vec![0x22]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD [HLI], A".to_string())))
}

#[test]
fn inc_hl() {
    let bus = SimpleBus::new(vec![0x23]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "INC HL".to_string())))
}

#[test]
fn inc_h() {
    let bus = SimpleBus::new(vec![0x24]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "INC H".to_string())))
}

#[test]
fn dec_h() {
    let bus = SimpleBus::new(vec![0x25]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DEC H".to_string())))
}

#[test]
fn ld_h_n8() {
    let bus = SimpleBus::new(vec![0x26, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "LD H, $2A".to_string())))
}

#[test]
fn daa() {
    let bus = SimpleBus::new(vec![0x27]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DAA".to_string())))
}

#[test]
fn jr_z_i8() {
    let bus = SimpleBus::new(vec![0x28, 0xFC]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "JR Z, -4".to_string())))
}

#[test]
fn add_hl_hl() {
    let bus = SimpleBus::new(vec![0x29]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADD HL, HL".to_string())))
}

#[test]
fn ld_a_hli() {
    let bus = SimpleBus::new(vec![0x2A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD A, [HLI]".to_string())))
}

#[test]
fn dec_hl() {
    let bus = SimpleBus::new(vec![0x2B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DEC HL".to_string())))
}

#[test]
fn inc_l() {
    let bus = SimpleBus::new(vec![0x2C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "INC L".to_string())))
}

#[test]
fn dec_l() {
    let bus = SimpleBus::new(vec![0x2D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DEC L".to_string())))
}

#[test]
fn ld_l_n8() {
    let bus = SimpleBus::new(vec![0x2E, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "LD L, $2A".to_string())))
}

#[test]
fn cpl() {
    let bus = SimpleBus::new(vec![0x2F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "CPL".to_string())))
}

// 0x3X
#[test]
fn jr_nc_i8() {
    let bus = SimpleBus::new(vec![0x30, 0xFC]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x0, &prefs);

    assert_eq!(result, Ok((2, "JR NC, -4".to_string())))
}

#[test]
fn ld_sp_n16() {
    let bus = SimpleBus::new(vec![0x31, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x0, &prefs);

    assert_eq!(result, Ok((3, "LD SP, $1234".to_string())))
}

#[test]
fn ld_hld_a() {
    let bus = SimpleBus::new(vec![0x32]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD [HLD], A".to_string())))
}

#[test]
fn inc_sp() {
    let bus = SimpleBus::new(vec![0x33]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "INC SP".to_string())))
}

#[test]
fn inc_hlr() {
    let bus = SimpleBus::new(vec![0x34]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "INC [HL]".to_string())))
}

#[test]
fn dec_hlr() {
    let bus = SimpleBus::new(vec![0x35]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DEC [HL]".to_string())))
}

#[test]
fn ld_hlr_n8() {
    let bus = SimpleBus::new(vec![0x36, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "LD [HL], $2A".to_string())))
}

#[test]
fn scf() {
    let bus = SimpleBus::new(vec![0x37]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SCF".to_string())))
}

#[test]
fn jr_c_i8() {
    let bus = SimpleBus::new(vec![0x38, 0xFC]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "JR C, -4".to_string())))
}

#[test]
fn add_hl_sp() {
    let bus = SimpleBus::new(vec![0x39]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADD HL, SP".to_string())))
}

#[test]
fn ld_a_hld() {
    let bus = SimpleBus::new(vec![0x3A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD A, [HLD]".to_string())))
}

#[test]
fn dec_sp() {
    let bus = SimpleBus::new(vec![0x3B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DEC SP".to_string())))
}

#[test]
fn inc_a() {
    let bus = SimpleBus::new(vec![0x3C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "INC A".to_string())))
}

#[test]
fn dec_a() {
    let bus = SimpleBus::new(vec![0x3D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DEC A".to_string())))
}

#[test]
fn ld_a_n8() {
    let bus = SimpleBus::new(vec![0x3E, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "LD A, $2A".to_string())))
}

#[test]
fn ccf() {
    let bus = SimpleBus::new(vec![0x3F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "CCF".to_string())))
}

// 0x4X
#[test]
fn ld_b_b() {
    let bus = SimpleBus::new(vec![0x40]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD B, B".to_string())))
}

#[test]
fn ld_b_c() {
    let bus = SimpleBus::new(vec![0x41]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD B, C".to_string())))
}

#[test]
fn ld_b_d() {
    let bus = SimpleBus::new(vec![0x42]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD B, D".to_string())))
}

#[test]
fn ld_b_e() {
    let bus = SimpleBus::new(vec![0x43]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD B, E".to_string())))
}

#[test]
fn ld_b_h() {
    let bus = SimpleBus::new(vec![0x44]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD B, H".to_string())))
}

#[test]
fn ld_b_l() {
    let bus = SimpleBus::new(vec![0x45]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD B, L".to_string())))
}

#[test]
fn ld_b_hlr() {
    let bus = SimpleBus::new(vec![0x46]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD B, [HL]".to_string())))
}

#[test]
fn ld_b_a() {
    let bus = SimpleBus::new(vec![0x47]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD B, A".to_string())))
}

#[test]
fn ld_c_b() {
    let bus = SimpleBus::new(vec![0x48]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD C, B".to_string())))
}

#[test]
fn ld_c_c() {
    let bus = SimpleBus::new(vec![0x49]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD C, C".to_string())))
}

#[test]
fn ld_c_d() {
    let bus = SimpleBus::new(vec![0x4a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD C, D".to_string())))
}

#[test]
fn ld_c_e() {
    let bus = SimpleBus::new(vec![0x4b]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD C, E".to_string())))
}

#[test]
fn ld_c_h() {
    let bus = SimpleBus::new(vec![0x4c]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD C, H".to_string())))
}

#[test]
fn ld_c_l() {
    let bus = SimpleBus::new(vec![0x4d]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD C, L".to_string())))
}

#[test]
fn ld_c_hlr() {
    let bus = SimpleBus::new(vec![0x4e]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD C, [HL]".to_string())))
}

#[test]
fn ld_c_a() {
    let bus = SimpleBus::new(vec![0x4f]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD C, A".to_string())))
}

// 0x5X
#[test]
fn ld_d_b() {
    let bus = SimpleBus::new(vec![0x50]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD D, B".to_string())))
}

#[test]
fn ld_d_c() {
    let bus = SimpleBus::new(vec![0x51]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD D, C".to_string())))
}

#[test]
fn ld_d_d() {
    let bus = SimpleBus::new(vec![0x52]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD D, D".to_string())))
}

#[test]
fn ld_d_e() {
    let bus = SimpleBus::new(vec![0x53]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD D, E".to_string())))
}

#[test]
fn ld_d_h() {
    let bus = SimpleBus::new(vec![0x54]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD D, H".to_string())))
}

#[test]
fn ld_d_l() {
    let bus = SimpleBus::new(vec![0x55]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD D, L".to_string())))
}

#[test]
fn ld_d_hlr() {
    let bus = SimpleBus::new(vec![0x56]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD D, [HL]".to_string())))
}

#[test]
fn ld_d_a() {
    let bus = SimpleBus::new(vec![0x57]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD D, A".to_string())))
}

#[test]
fn ld_e_b() {
    let bus = SimpleBus::new(vec![0x58]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD E, B".to_string())))
}

#[test]
fn ld_e_c() {
    let bus = SimpleBus::new(vec![0x59]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD E, C".to_string())))
}

#[test]
fn ld_e_d() {
    let bus = SimpleBus::new(vec![0x5a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD E, D".to_string())))
}

#[test]
fn ld_e_e() {
    let bus = SimpleBus::new(vec![0x5b]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD E, E".to_string())))
}

#[test]
fn ld_e_h() {
    let bus = SimpleBus::new(vec![0x5c]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD E, H".to_string())))
}

#[test]
fn ld_e_l() {
    let bus = SimpleBus::new(vec![0x5d]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD E, L".to_string())))
}

#[test]
fn ld_e_hlr() {
    let bus = SimpleBus::new(vec![0x5e]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD E, [HL]".to_string())))
}

#[test]
fn ld_e_a() {
    let bus = SimpleBus::new(vec![0x5f]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD E, A".to_string())))
}

// 0x6X
#[test]
fn ld_h_b() {
    let bus = SimpleBus::new(vec![0x60]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD H, B".to_string())))
}

#[test]
fn ld_h_c() {
    let bus = SimpleBus::new(vec![0x61]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD H, C".to_string())))
}

#[test]
fn ld_h_d() {
    let bus = SimpleBus::new(vec![0x62]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD H, D".to_string())))
}

#[test]
fn ld_h_e() {
    let bus = SimpleBus::new(vec![0x63]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD H, E".to_string())))
}

#[test]
fn ld_h_h() {
    let bus = SimpleBus::new(vec![0x64]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD H, H".to_string())))
}

#[test]
fn ld_h_l() {
    let bus = SimpleBus::new(vec![0x65]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD H, L".to_string())))
}

#[test]
fn ld_h_hlr() {
    let bus = SimpleBus::new(vec![0x66]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD H, [HL]".to_string())))
}

#[test]
fn ld_h_a() {
    let bus = SimpleBus::new(vec![0x67]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD H, A".to_string())))
}

#[test]
fn ld_l_b() {
    let bus = SimpleBus::new(vec![0x68]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD L, B".to_string())))
}

#[test]
fn ld_l_c() {
    let bus = SimpleBus::new(vec![0x69]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD L, C".to_string())))
}

#[test]
fn ld_l_d() {
    let bus = SimpleBus::new(vec![0x6a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD L, D".to_string())))
}

#[test]
fn ld_l_e() {
    let bus = SimpleBus::new(vec![0x6b]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD L, E".to_string())))
}

#[test]
fn ld_l_h() {
    let bus = SimpleBus::new(vec![0x6c]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD L, H".to_string())))
}

#[test]
fn ld_l_l() {
    let bus = SimpleBus::new(vec![0x6d]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD L, L".to_string())))
}

#[test]
fn ld_l_hlr() {
    let bus = SimpleBus::new(vec![0x6e]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD L, [HL]".to_string())))
}

#[test]
fn ld_l_a() {
    let bus = SimpleBus::new(vec![0x6f]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD L, A".to_string())))
}

// 0x7X
#[test]
fn ld_hlr_b() {
    let bus = SimpleBus::new(vec![0x70]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD [HL], B".to_string())))
}

#[test]
fn ld_hlr_c() {
    let bus = SimpleBus::new(vec![0x71]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD [HL], C".to_string())))
}

#[test]
fn ld_hlr_d() {
    let bus = SimpleBus::new(vec![0x72]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD [HL], D".to_string())))
}

#[test]
fn ld_hlr_e() {
    let bus = SimpleBus::new(vec![0x73]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD [HL], E".to_string())))
}

#[test]
fn ld_hlr_h() {
    let bus = SimpleBus::new(vec![0x74]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD [HL], H".to_string())))
}

#[test]
fn ld_hlr_l() {
    let bus = SimpleBus::new(vec![0x75]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD [HL], L".to_string())))
}

#[test]
fn halt() {
    let bus = SimpleBus::new(vec![0x76]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "HALT".to_string())))
}

#[test]
fn ld_hlr_a() {
    let bus = SimpleBus::new(vec![0x77]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD [HL], A".to_string())))
}

#[test]
fn ld_a_b() {
    let bus = SimpleBus::new(vec![0x78]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD A, B".to_string())))
}

#[test]
fn ld_a_c() {
    let bus = SimpleBus::new(vec![0x79]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD A, C".to_string())))
}

#[test]
fn ld_a_d() {
    let bus = SimpleBus::new(vec![0x7a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD A, D".to_string())))
}

#[test]
fn ld_a_e() {
    let bus = SimpleBus::new(vec![0x7b]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD A, E".to_string())))
}

#[test]
fn ld_a_h() {
    let bus = SimpleBus::new(vec![0x7c]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD A, H".to_string())))
}

#[test]
fn ld_a_l() {
    let bus = SimpleBus::new(vec![0x7d]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD A, L".to_string())))
}

#[test]
fn ld_a_hlr() {
    let bus = SimpleBus::new(vec![0x7e]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD A, [HL]".to_string())))
}

#[test]
fn ld_a_a() {
    let bus = SimpleBus::new(vec![0x7f]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD A, A".to_string())))
}

// 0x8X
#[test]
fn add_a_b() {
    let bus = SimpleBus::new(vec![0x80]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADD A, B".to_string())))
}

#[test]
fn add_a_c() {
    let bus = SimpleBus::new(vec![0x81]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADD A, C".to_string())))
}

#[test]
fn add_a_d() {
    let bus = SimpleBus::new(vec![0x82]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADD A, D".to_string())))
}

#[test]
fn add_a_e() {
    let bus = SimpleBus::new(vec![0x83]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADD A, E".to_string())))
}

#[test]
fn add_a_h() {
    let bus = SimpleBus::new(vec![0x84]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADD A, H".to_string())))
}

#[test]
fn add_a_l() {
    let bus = SimpleBus::new(vec![0x85]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADD A, L".to_string())))
}

#[test]
fn add_a_hlr() {
    let bus = SimpleBus::new(vec![0x86]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADD A, [HL]".to_string())))
}

#[test]
fn add_a_a() {
    let bus = SimpleBus::new(vec![0x87]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADD A, A".to_string())))
}

#[test]
fn adc_a_b() {
    let bus = SimpleBus::new(vec![0x88]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADC A, B".to_string())))
}

#[test]
fn adc_a_c() {
    let bus = SimpleBus::new(vec![0x89]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADC A, C".to_string())))
}

#[test]
fn adc_a_d() {
    let bus = SimpleBus::new(vec![0x8a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADC A, D".to_string())))
}

#[test]
fn adc_a_e() {
    let bus = SimpleBus::new(vec![0x8b]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADC A, E".to_string())))
}

#[test]
fn adc_a_h() {
    let bus = SimpleBus::new(vec![0x8c]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADC A, H".to_string())))
}

#[test]
fn adc_a_l() {
    let bus = SimpleBus::new(vec![0x8d]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADC A, L".to_string())))
}

#[test]
fn adc_a_hlr() {
    let bus = SimpleBus::new(vec![0x8e]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADC A, [HL]".to_string())))
}

#[test]
fn adc_a_a() {
    let bus = SimpleBus::new(vec![0x8f]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "ADC A, A".to_string())))
}

// 0x9X
#[test]
fn sub_a_b() {
    let bus = SimpleBus::new(vec![0x90]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SUB A, B".to_string())))
}

#[test]
fn sub_a_c() {
    let bus = SimpleBus::new(vec![0x91]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SUB A, C".to_string())))
}

#[test]
fn sub_a_d() {
    let bus = SimpleBus::new(vec![0x92]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SUB A, D".to_string())))
}

#[test]
fn sub_a_e() {
    let bus = SimpleBus::new(vec![0x93]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SUB A, E".to_string())))
}

#[test]
fn sub_a_h() {
    let bus = SimpleBus::new(vec![0x94]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SUB A, H".to_string())))
}

#[test]
fn sub_a_l() {
    let bus = SimpleBus::new(vec![0x95]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SUB A, L".to_string())))
}

#[test]
fn sub_a_hlr() {
    let bus = SimpleBus::new(vec![0x96]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SUB A, [HL]".to_string())))
}

#[test]
fn sub_a_a() {
    let bus = SimpleBus::new(vec![0x97]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SUB A, A".to_string())))
}

#[test]
fn sbc_a_b() {
    let bus = SimpleBus::new(vec![0x98]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SBC A, B".to_string())))
}

#[test]
fn sbc_a_c() {
    let bus = SimpleBus::new(vec![0x99]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SBC A, C".to_string())))
}

#[test]
fn sbc_a_d() {
    let bus = SimpleBus::new(vec![0x9a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SBC A, D".to_string())))
}

#[test]
fn sbc_a_e() {
    let bus = SimpleBus::new(vec![0x9b]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SBC A, E".to_string())))
}

#[test]
fn sbc_a_h() {
    let bus = SimpleBus::new(vec![0x9c]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SBC A, H".to_string())))
}

#[test]
fn sbc_a_l() {
    let bus = SimpleBus::new(vec![0x9d]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SBC A, L".to_string())))
}

#[test]
fn sbc_a_hlr() {
    let bus = SimpleBus::new(vec![0x9e]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SBC A, [HL]".to_string())))
}

#[test]
fn sbc_a_a() {
    let bus = SimpleBus::new(vec![0x9f]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "SBC A, A".to_string())))
}

// 0xAX
#[test]
fn and_a_b() {
    let bus = SimpleBus::new(vec![0xA0]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "AND A, B".to_string())))
}

#[test]
fn and_a_c() {
    let bus = SimpleBus::new(vec![0xA1]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "AND A, C".to_string())))
}

#[test]
fn and_a_d() {
    let bus = SimpleBus::new(vec![0xA2]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "AND A, D".to_string())))
}

#[test]
fn and_a_e() {
    let bus = SimpleBus::new(vec![0xA3]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "AND A, E".to_string())))
}

#[test]
fn and_a_h() {
    let bus = SimpleBus::new(vec![0xA4]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "AND A, H".to_string())))
}

#[test]
fn and_a_l() {
    let bus = SimpleBus::new(vec![0xA5]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "AND A, L".to_string())))
}

#[test]
fn and_a_hlr() {
    let bus = SimpleBus::new(vec![0xA6]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "AND A, [HL]".to_string())))
}

#[test]
fn and_a_a() {
    let bus = SimpleBus::new(vec![0xA7]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "AND A, A".to_string())))
}

#[test]
fn xor_a_b() {
    let bus = SimpleBus::new(vec![0xA8]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "XOR A, B".to_string())))
}

#[test]
fn xor_a_c() {
    let bus = SimpleBus::new(vec![0xA9]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "XOR A, C".to_string())))
}

#[test]
fn xor_a_d() {
    let bus = SimpleBus::new(vec![0xAa]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "XOR A, D".to_string())))
}

#[test]
fn xor_a_e() {
    let bus = SimpleBus::new(vec![0xAb]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "XOR A, E".to_string())))
}

#[test]
fn xor_a_h() {
    let bus = SimpleBus::new(vec![0xAc]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "XOR A, H".to_string())))
}

#[test]
fn xor_a_l() {
    let bus = SimpleBus::new(vec![0xAd]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "XOR A, L".to_string())))
}

#[test]
fn xor_a_hlr() {
    let bus = SimpleBus::new(vec![0xAe]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "XOR A, [HL]".to_string())))
}

#[test]
fn xor_a_a() {
    let bus = SimpleBus::new(vec![0xAf]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "XOR A, A".to_string())))
}

// 0xBX
#[test]
fn or_a_b() {
    let bus = SimpleBus::new(vec![0xB0]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "OR A, B".to_string())))
}

#[test]
fn or_a_c() {
    let bus = SimpleBus::new(vec![0xB1]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "OR A, C".to_string())))
}

#[test]
fn or_a_d() {
    let bus = SimpleBus::new(vec![0xB2]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "OR A, D".to_string())))
}

#[test]
fn or_a_e() {
    let bus = SimpleBus::new(vec![0xB3]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "OR A, E".to_string())))
}

#[test]
fn or_a_h() {
    let bus = SimpleBus::new(vec![0xB4]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "OR A, H".to_string())))
}

#[test]
fn or_a_l() {
    let bus = SimpleBus::new(vec![0xB5]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "OR A, L".to_string())))
}

#[test]
fn or_a_hlr() {
    let bus = SimpleBus::new(vec![0xB6]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "OR A, [HL]".to_string())))
}

#[test]
fn or_a_a() {
    let bus = SimpleBus::new(vec![0xB7]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "OR A, A".to_string())))
}

#[test]
fn cp_a_b() {
    let bus = SimpleBus::new(vec![0xB8]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "CP A, B".to_string())))
}

#[test]
fn cp_a_c() {
    let bus = SimpleBus::new(vec![0xB9]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "CP A, C".to_string())))
}

#[test]
fn cp_a_d() {
    let bus = SimpleBus::new(vec![0xBa]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "CP A, D".to_string())))
}

#[test]
fn cp_a_e() {
    let bus = SimpleBus::new(vec![0xBb]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "CP A, E".to_string())))
}

#[test]
fn cp_a_h() {
    let bus = SimpleBus::new(vec![0xBc]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "CP A, H".to_string())))
}

#[test]
fn cp_a_l() {
    let bus = SimpleBus::new(vec![0xBd]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "CP A, L".to_string())))
}

#[test]
fn cp_a_hlr() {
    let bus = SimpleBus::new(vec![0xBe]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "CP A, [HL]".to_string())))
}

#[test]
fn cp_a_a() {
    let bus = SimpleBus::new(vec![0xBf]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "CP A, A".to_string())))
}

// 0xCX
#[test]
fn ret_nz() {
    let bus = SimpleBus::new(vec![0xC0]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RET NZ".to_string())))
}

#[test]
fn pop_bc() {
    let bus = SimpleBus::new(vec![0xC1]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "POP BC".to_string())))
}

#[test]
fn jp_nz_n16() {
    let bus = SimpleBus::new(vec![0xC2, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((3, "JP NZ, $1234".to_string())))
}

#[test]
fn jp_n16() {
    let bus = SimpleBus::new(vec![0xC3, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((3, "JP $1234".to_string())))
}

#[test]
fn call_nz_n16() {
    let bus = SimpleBus::new(vec![0xC4, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((3, "CALL NZ, $1234".to_string())))
}

#[test]
fn push_bc() {
    let bus = SimpleBus::new(vec![0xC5]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "PUSH BC".to_string())))
}

#[test]
fn add_a_n8() {
    let bus = SimpleBus::new(vec![0xC6, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "ADD A, $2A".to_string())))
}

#[test]
fn rst_00() {
    let bus = SimpleBus::new(vec![0xC7]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RST $00".to_string())))
}

#[test]
fn ret_z() {
    let bus = SimpleBus::new(vec![0xC8]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RET Z".to_string())))
}

#[test]
fn ret() {
    let bus = SimpleBus::new(vec![0xC9]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RET".to_string())))
}

#[test]
fn jp_z_n16() {
    let bus = SimpleBus::new(vec![0xCA, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((3, "JP Z, $1234".to_string())))
}

#[test]
fn call_z_n16() {
    let bus = SimpleBus::new(vec![0xCC, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((3, "CALL Z, $1234".to_string())))
}

#[test]
fn call_n16() {
    let bus = SimpleBus::new(vec![0xCD, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((3, "CALL $1234".to_string())))
}

#[test]
fn adc_a_n8() {
    let bus = SimpleBus::new(vec![0xCE, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "ADC A, $2A".to_string())))
}

#[test]
fn rst_08() {
    let bus = SimpleBus::new(vec![0xCF]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RST $08".to_string())))
}

// 0xDX
#[test]
fn ret_nc() {
    let bus = SimpleBus::new(vec![0xD0]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RET NC".to_string())))
}

#[test]
fn pop_de() {
    let bus = SimpleBus::new(vec![0xD1]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "POP DE".to_string())))
}

#[test]
fn jp_nc_n16() {
    let bus = SimpleBus::new(vec![0xD2, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((3, "JP NC, $1234".to_string())))
}

#[test]
fn call_nc_n16() {
    let bus = SimpleBus::new(vec![0xD4, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((3, "CALL NC, $1234".to_string())))
}

#[test]
fn push_de() {
    let bus = SimpleBus::new(vec![0xD5]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "PUSH DE".to_string())))
}

#[test]
fn sub_a_n8() {
    let bus = SimpleBus::new(vec![0xD6, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SUB A, $2A".to_string())))
}

#[test]
fn rst_10() {
    let bus = SimpleBus::new(vec![0xD7]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RST $10".to_string())))
}

#[test]
fn ret_c() {
    let bus = SimpleBus::new(vec![0xD8]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RET C".to_string())))
}

#[test]
fn reti() {
    let bus = SimpleBus::new(vec![0xD9]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RETI".to_string())))
}

#[test]
fn jp_c_n16() {
    let bus = SimpleBus::new(vec![0xDA, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((3, "JP C, $1234".to_string())))
}

#[test]
fn call_c_n16() {
    let bus = SimpleBus::new(vec![0xDC, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((3, "CALL C, $1234".to_string())))
}

#[test]
fn sbc_a_n8() {
    let bus = SimpleBus::new(vec![0xDE, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SBC A, $2A".to_string())))
}

#[test]
fn rst_18() {
    let bus = SimpleBus::new(vec![0xDF]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RST $18".to_string())))
}

// 0xEX
#[test]
fn ldh_n8_a() {
    let bus = SimpleBus::new(vec![0xE0, 0xDE]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "LDH [$FFDE], A".to_string())))
}

#[test]
fn pop_hl() {
    let bus = SimpleBus::new(vec![0xE1]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "POP HL".to_string())))
}

#[test]
fn ldh_cr_a() {
    let bus = SimpleBus::new(vec![0xE2]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LDH [C], A".to_string())))
}

#[test]
fn push_hl() {
    let bus = SimpleBus::new(vec![0xE5]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "PUSH HL".to_string())))
}

#[test]
fn and_a_n8() {
    let bus = SimpleBus::new(vec![0xE6, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "AND A, $2A".to_string())))
}

#[test]
fn rst_20() {
    let bus = SimpleBus::new(vec![0xE7]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RST $20".to_string())))
}

#[test]
fn add_sp_i8_neg() {
    let bus = SimpleBus::new(vec![0xE8, 0xFD]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "ADD SP, -3".to_string())))
}

#[test]
fn add_sp_i8_pos() {
    let bus = SimpleBus::new(vec![0xE8, 0x02]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "ADD SP, 2".to_string())))
}

#[test]
fn jp_hl() {
    let bus = SimpleBus::new(vec![0xE9]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "JP HL".to_string())))
}

#[test]
fn ld_n16r_a() {
    let bus = SimpleBus::new(vec![0xEA, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((3, "LD [$1234], A".to_string())))
}

#[test]
fn xor_a_n8() {
    let bus = SimpleBus::new(vec![0xEE, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "XOR A, $2A".to_string())))
}

#[test]
fn rst_28() {
    let bus = SimpleBus::new(vec![0xEF]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RST $28".to_string())))
}

// 0xFX
#[test]
fn ldh_a_n8() {
    let bus = SimpleBus::new(vec![0xF0, 0xDE]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "LDH A, [$FFDE]".to_string())))
}

#[test]
fn pop_af() {
    let bus = SimpleBus::new(vec![0xF1]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "POP AF".to_string())))
}

#[test]
fn ldh_a_cr() {
    let bus = SimpleBus::new(vec![0xF2]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LDH A, [C]".to_string())))
}

#[test]
fn di() {
    let bus = SimpleBus::new(vec![0xF3]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "DI".to_string())))
}

#[test]
fn push_af() {
    let bus = SimpleBus::new(vec![0xF5]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "PUSH AF".to_string())))
}

#[test]
fn or_a_n8() {
    let bus = SimpleBus::new(vec![0xF6, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "OR A, $2A".to_string())))
}

#[test]
fn rst_30() {
    let bus = SimpleBus::new(vec![0xF7]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RST $30".to_string())))
}

#[test]
fn ld_sp_i8_neg() {
    let bus = SimpleBus::new(vec![0xF8, 0xFD]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "LD HL, SP-3".to_string())))
}

#[test]
fn ld_sp_i8_pos() {
    let bus = SimpleBus::new(vec![0xF8, 0x02]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "LD HL, SP+2".to_string())))
}

#[test]
fn ld_sp_hl() {
    let bus = SimpleBus::new(vec![0xF9]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "LD SP, HL".to_string())))
}

#[test]
fn ld_a_n16r() {
    let bus = SimpleBus::new(vec![0xFA, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((3, "LD A, [$1234]".to_string())))
}

#[test]
fn ei() {
    let bus = SimpleBus::new(vec![0xFB]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "EI".to_string())))
}

#[test]
fn cp_a_n8() {
    let bus = SimpleBus::new(vec![0xFE, 0x2a]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "CP A, $2A".to_string())))
}

#[test]
fn rst_38() {
    let bus = SimpleBus::new(vec![0xFF]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((1, "RST $38".to_string())))
}

// 0xCB-prefix
// 0x0X
#[test]
fn rlc_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x00]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RLC B".to_string())))
}

#[test]
fn rlc_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x01]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RLC C".to_string())))
}

#[test]
fn rlc_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x02]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RLC D".to_string())))
}

#[test]
fn rlc_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x03]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RLC E".to_string())))
}

#[test]
fn rlc_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x04]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RLC H".to_string())))
}

#[test]
fn rlc_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x05]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RLC L".to_string())))
}

#[test]
fn rlc_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x06]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RLC [HL]".to_string())))
}

#[test]
fn rlc_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x07]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RLC A".to_string())))
}

#[test]
fn rrc_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x08]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RRC B".to_string())))
}

#[test]
fn rrc_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x09]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RRC C".to_string())))
}

#[test]
fn rrc_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x0A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RRC D".to_string())))
}

#[test]
fn rrc_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x0B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RRC E".to_string())))
}

#[test]
fn rrc_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x0C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RRC H".to_string())))
}

#[test]
fn rrc_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x0D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RRC L".to_string())))
}

#[test]
fn rrc_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x0E]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RRC [HL]".to_string())))
}

#[test]
fn rrc_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x0F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RRC A".to_string())))
}

// 0x1X
#[test]
fn rl_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x10]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RL B".to_string())))
}

#[test]
fn rl_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x11]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RL C".to_string())))
}

#[test]
fn rl_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RL D".to_string())))
}

#[test]
fn rl_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x13]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RL E".to_string())))
}

#[test]
fn rl_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x14]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RL H".to_string())))
}

#[test]
fn rl_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x15]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RL L".to_string())))
}

#[test]
fn rl_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x16]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RL [HL]".to_string())))
}

#[test]
fn rl_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x17]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RL A".to_string())))
}

#[test]
fn rr_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x18]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RR B".to_string())))
}

#[test]
fn rr_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x19]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RR C".to_string())))
}

#[test]
fn rr_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x1A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RR D".to_string())))
}

#[test]
fn rr_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x1B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RR E".to_string())))
}

#[test]
fn rr_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x1C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RR H".to_string())))
}

#[test]
fn rr_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x1D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RR L".to_string())))
}

#[test]
fn rr_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x1E]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RR [HL]".to_string())))
}

#[test]
fn rr_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x1F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RR A".to_string())))
}

// 0x2X
#[test]
fn sla_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x20]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SLA B".to_string())))
}

#[test]
fn sla_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x21]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SLA C".to_string())))
}

#[test]
fn sla_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x22]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SLA D".to_string())))
}

#[test]
fn sla_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x23]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SLA E".to_string())))
}

#[test]
fn sla_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x24]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SLA H".to_string())))
}

#[test]
fn sla_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x25]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SLA L".to_string())))
}

#[test]
fn sla_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x26]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SLA [HL]".to_string())))
}

#[test]
fn sla_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x27]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SLA A".to_string())))
}

#[test]
fn sra_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x28]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRA B".to_string())))
}

#[test]
fn sra_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x29]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRA C".to_string())))
}

#[test]
fn sra_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x2A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRA D".to_string())))
}

#[test]
fn sra_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x2B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRA E".to_string())))
}

#[test]
fn sra_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x2C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRA H".to_string())))
}

#[test]
fn sra_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x2D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRA L".to_string())))
}

#[test]
fn sra_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x2E]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRA [HL]".to_string())))
}

#[test]
fn sra_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x2F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRA A".to_string())))
}

// 0x3X
#[test]
fn swap_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x30]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SWAP B".to_string())))
}

#[test]
fn swap_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x31]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SWAP C".to_string())))
}

#[test]
fn swap_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x32]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SWAP D".to_string())))
}

#[test]
fn swap_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x33]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SWAP E".to_string())))
}

#[test]
fn swap_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x34]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SWAP H".to_string())))
}

#[test]
fn swap_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x35]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SWAP L".to_string())))
}

#[test]
fn swap_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x36]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SWAP [HL]".to_string())))
}

#[test]
fn swap_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x37]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SWAP A".to_string())))
}

#[test]
fn srl_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x38]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRL B".to_string())))
}

#[test]
fn srl_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x39]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRL C".to_string())))
}

#[test]
fn srl_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x3A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRL D".to_string())))
}

#[test]
fn srl_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x3B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRL E".to_string())))
}

#[test]
fn srl_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x3C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRL H".to_string())))
}

#[test]
fn srl_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x3D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRL L".to_string())))
}

#[test]
fn srl_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x3E]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRL [HL]".to_string())))
}

#[test]
fn srl_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x3F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SRL A".to_string())))
}

// 0x4X
#[test]
fn bit_0_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x40]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 0, B".to_string())))
}

#[test]
fn bit_0_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x41]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 0, C".to_string())))
}

#[test]
fn bit_0_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x42]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 0, D".to_string())))
}

#[test]
fn bit_0_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x43]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 0, E".to_string())))
}

#[test]
fn bit_0_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x44]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 0, H".to_string())))
}

#[test]
fn bit_0_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x45]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 0, L".to_string())))
}

#[test]
fn bit_0_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x46]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 0, [HL]".to_string())))
}

#[test]
fn bit_0_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x47]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 0, A".to_string())))
}

#[test]
fn bit_1_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x48]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 1, B".to_string())))
}

#[test]
fn bit_1_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x49]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 1, C".to_string())))
}

#[test]
fn bit_1_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x4A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 1, D".to_string())))
}

#[test]
fn bit_1_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x4B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 1, E".to_string())))
}

#[test]
fn bit_1_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x4C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 1, H".to_string())))
}

#[test]
fn bit_1_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x4D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 1, L".to_string())))
}

#[test]
fn bit_1_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x4E]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 1, [HL]".to_string())))
}

#[test]
fn bit_1_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x4F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 1, A".to_string())))
}

// 0x5X
#[test]
fn bit_2_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x50]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 2, B".to_string())))
}

#[test]
fn bit_2_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x51]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 2, C".to_string())))
}

#[test]
fn bit_2_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x52]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 2, D".to_string())))
}

#[test]
fn bit_2_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x53]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 2, E".to_string())))
}

#[test]
fn bit_2_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x54]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 2, H".to_string())))
}

#[test]
fn bit_2_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x55]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 2, L".to_string())))
}

#[test]
fn bit_2_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x56]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 2, [HL]".to_string())))
}

#[test]
fn bit_2_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x57]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 2, A".to_string())))
}

#[test]
fn bit_3_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x58]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 3, B".to_string())))
}

#[test]
fn bit_3_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x59]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 3, C".to_string())))
}

#[test]
fn bit_3_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x5A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 3, D".to_string())))
}

#[test]
fn bit_3_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x5B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 3, E".to_string())))
}

#[test]
fn bit_3_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x5C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 3, H".to_string())))
}

#[test]
fn bit_3_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x5D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 3, L".to_string())))
}

#[test]
fn bit_3_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x5E]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 3, [HL]".to_string())))
}

#[test]
fn bit_3_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x5F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 3, A".to_string())))
}

// 0x6X
#[test]
fn bit_4_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x60]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 4, B".to_string())))
}

#[test]
fn bit_4_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x61]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 4, C".to_string())))
}

#[test]
fn bit_4_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x62]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 4, D".to_string())))
}

#[test]
fn bit_4_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x63]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 4, E".to_string())))
}

#[test]
fn bit_4_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x64]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 4, H".to_string())))
}

#[test]
fn bit_4_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x65]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 4, L".to_string())))
}

#[test]
fn bit_4_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x66]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 4, [HL]".to_string())))
}

#[test]
fn bit_4_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x67]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 4, A".to_string())))
}

#[test]
fn bit_5_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x68]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 5, B".to_string())))
}

#[test]
fn bit_5_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x69]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 5, C".to_string())))
}

#[test]
fn bit_5_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x6A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 5, D".to_string())))
}

#[test]
fn bit_5_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x6B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 5, E".to_string())))
}

#[test]
fn bit_5_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x6C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 5, H".to_string())))
}

#[test]
fn bit_5_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x6D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 5, L".to_string())))
}

#[test]
fn bit_5_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x6E]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 5, [HL]".to_string())))
}

#[test]
fn bit_5_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x6F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 5, A".to_string())))
}

// 0x7X
#[test]
fn bit_6_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x70]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 6, B".to_string())))
}

#[test]
fn bit_6_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x71]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 6, C".to_string())))
}

#[test]
fn bit_6_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x72]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 6, D".to_string())))
}

#[test]
fn bit_6_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x73]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 6, E".to_string())))
}

#[test]
fn bit_6_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x74]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 6, H".to_string())))
}

#[test]
fn bit_6_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x75]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 6, L".to_string())))
}

#[test]
fn bit_6_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x76]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 6, [HL]".to_string())))
}

#[test]
fn bit_6_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x77]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 6, A".to_string())))
}

#[test]
fn bit_7_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x78]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 7, B".to_string())))
}

#[test]
fn bit_7_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x79]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 7, C".to_string())))
}

#[test]
fn bit_7_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x7A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 7, D".to_string())))
}

#[test]
fn bit_7_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x7B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 7, E".to_string())))
}

#[test]
fn bit_7_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x7C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 7, H".to_string())))
}

#[test]
fn bit_7_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x7D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 7, L".to_string())))
}

#[test]
fn bit_7_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x7E]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 7, [HL]".to_string())))
}

#[test]
fn bit_7_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x7F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "BIT 7, A".to_string())))
}

// 0x8X
#[test]
fn res_0_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x80]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 0, B".to_string())))
}

#[test]
fn res_0_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x81]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 0, C".to_string())))
}

#[test]
fn res_0_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x82]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 0, D".to_string())))
}

#[test]
fn res_0_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x83]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 0, E".to_string())))
}

#[test]
fn res_0_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x84]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 0, H".to_string())))
}

#[test]
fn res_0_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x85]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 0, L".to_string())))
}

#[test]
fn res_0_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x86]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 0, [HL]".to_string())))
}

#[test]
fn res_0_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x87]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 0, A".to_string())))
}

#[test]
fn res_1_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x88]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 1, B".to_string())))
}

#[test]
fn res_1_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x89]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 1, C".to_string())))
}

#[test]
fn res_1_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x8A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 1, D".to_string())))
}

#[test]
fn res_1_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x8B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 1, E".to_string())))
}

#[test]
fn res_1_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x8C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 1, H".to_string())))
}

#[test]
fn res_1_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x8D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 1, L".to_string())))
}

#[test]
fn res_1_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x8E]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 1, [HL]".to_string())))
}

#[test]
fn res_1_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x8F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 1, A".to_string())))
}

// 0x9X
#[test]
fn res_2_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x90]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 2, B".to_string())))
}

#[test]
fn res_2_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x91]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 2, C".to_string())))
}

#[test]
fn res_2_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x92]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 2, D".to_string())))
}

#[test]
fn res_2_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x93]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 2, E".to_string())))
}

#[test]
fn res_2_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x94]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 2, H".to_string())))
}

#[test]
fn res_2_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x95]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 2, L".to_string())))
}

#[test]
fn res_2_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x96]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 2, [HL]".to_string())))
}

#[test]
fn res_2_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x97]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 2, A".to_string())))
}

#[test]
fn res_3_b() {
    let bus = SimpleBus::new(vec![0xCB, 0x98]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 3, B".to_string())))
}

#[test]
fn res_3_c() {
    let bus = SimpleBus::new(vec![0xCB, 0x99]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 3, C".to_string())))
}

#[test]
fn res_3_d() {
    let bus = SimpleBus::new(vec![0xCB, 0x9A]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 3, D".to_string())))
}

#[test]
fn res_3_e() {
    let bus = SimpleBus::new(vec![0xCB, 0x9B]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 3, E".to_string())))
}

#[test]
fn res_3_h() {
    let bus = SimpleBus::new(vec![0xCB, 0x9C]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 3, H".to_string())))
}

#[test]
fn res_3_l() {
    let bus = SimpleBus::new(vec![0xCB, 0x9D]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 3, L".to_string())))
}

#[test]
fn res_3_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0x9E]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 3, [HL]".to_string())))
}

#[test]
fn res_3_a() {
    let bus = SimpleBus::new(vec![0xCB, 0x9F]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 3, A".to_string())))
}

// 0xAX
#[test]
fn res_4_b() {
    let bus = SimpleBus::new(vec![0xCB, 0xA0]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 4, B".to_string())))
}

#[test]
fn res_4_c() {
    let bus = SimpleBus::new(vec![0xCB, 0xA1]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 4, C".to_string())))
}

#[test]
fn res_4_d() {
    let bus = SimpleBus::new(vec![0xCB, 0xA2]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 4, D".to_string())))
}

#[test]
fn res_4_e() {
    let bus = SimpleBus::new(vec![0xCB, 0xA3]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 4, E".to_string())))
}

#[test]
fn res_4_h() {
    let bus = SimpleBus::new(vec![0xCB, 0xA4]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 4, H".to_string())))
}

#[test]
fn res_4_l() {
    let bus = SimpleBus::new(vec![0xCB, 0xA5]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 4, L".to_string())))
}

#[test]
fn res_4_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0xA6]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 4, [HL]".to_string())))
}

#[test]
fn res_4_a() {
    let bus = SimpleBus::new(vec![0xCB, 0xA7]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 4, A".to_string())))
}

#[test]
fn res_5_b() {
    let bus = SimpleBus::new(vec![0xCB, 0xA8]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 5, B".to_string())))
}

#[test]
fn res_5_c() {
    let bus = SimpleBus::new(vec![0xCB, 0xA9]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 5, C".to_string())))
}

#[test]
fn res_5_d() {
    let bus = SimpleBus::new(vec![0xCB, 0xAA]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 5, D".to_string())))
}

#[test]
fn res_5_e() {
    let bus = SimpleBus::new(vec![0xCB, 0xAB]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 5, E".to_string())))
}

#[test]
fn res_5_h() {
    let bus = SimpleBus::new(vec![0xCB, 0xAC]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 5, H".to_string())))
}

#[test]
fn res_5_l() {
    let bus = SimpleBus::new(vec![0xCB, 0xAD]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 5, L".to_string())))
}

#[test]
fn res_5_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0xAE]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 5, [HL]".to_string())))
}

#[test]
fn res_5_a() {
    let bus = SimpleBus::new(vec![0xCB, 0xAF]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 5, A".to_string())))
}

// 0xBX
#[test]
fn res_6_b() {
    let bus = SimpleBus::new(vec![0xCB, 0xB0]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 6, B".to_string())))
}

#[test]
fn res_6_c() {
    let bus = SimpleBus::new(vec![0xCB, 0xB1]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 6, C".to_string())))
}

#[test]
fn res_6_d() {
    let bus = SimpleBus::new(vec![0xCB, 0xB2]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 6, D".to_string())))
}

#[test]
fn res_6_e() {
    let bus = SimpleBus::new(vec![0xCB, 0xB3]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 6, E".to_string())))
}

#[test]
fn res_6_h() {
    let bus = SimpleBus::new(vec![0xCB, 0xB4]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 6, H".to_string())))
}

#[test]
fn res_6_l() {
    let bus = SimpleBus::new(vec![0xCB, 0xB5]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 6, L".to_string())))
}

#[test]
fn res_6_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0xB6]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 6, [HL]".to_string())))
}

#[test]
fn res_6_a() {
    let bus = SimpleBus::new(vec![0xCB, 0xB7]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 6, A".to_string())))
}

#[test]
fn res_7_b() {
    let bus = SimpleBus::new(vec![0xCB, 0xB8]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 7, B".to_string())))
}

#[test]
fn res_7_c() {
    let bus = SimpleBus::new(vec![0xCB, 0xB9]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 7, C".to_string())))
}

#[test]
fn res_7_d() {
    let bus = SimpleBus::new(vec![0xCB, 0xBA]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 7, D".to_string())))
}

#[test]
fn res_7_e() {
    let bus = SimpleBus::new(vec![0xCB, 0xBB]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 7, E".to_string())))
}

#[test]
fn res_7_h() {
    let bus = SimpleBus::new(vec![0xCB, 0xBC]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 7, H".to_string())))
}

#[test]
fn res_7_l() {
    let bus = SimpleBus::new(vec![0xCB, 0xBD]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 7, L".to_string())))
}

#[test]
fn res_7_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0xBE]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 7, [HL]".to_string())))
}

#[test]
fn res_7_a() {
    let bus = SimpleBus::new(vec![0xCB, 0xBF]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "RES 7, A".to_string())))
}

// 0xCX
#[test]
fn set_0_b() {
    let bus = SimpleBus::new(vec![0xCB, 0xC0]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 0, B".to_string())))
}

#[test]
fn set_0_c() {
    let bus = SimpleBus::new(vec![0xCB, 0xC1]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 0, C".to_string())))
}

#[test]
fn set_0_d() {
    let bus = SimpleBus::new(vec![0xCB, 0xC2]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 0, D".to_string())))
}

#[test]
fn set_0_e() {
    let bus = SimpleBus::new(vec![0xCB, 0xC3]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 0, E".to_string())))
}

#[test]
fn set_0_h() {
    let bus = SimpleBus::new(vec![0xCB, 0xC4]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 0, H".to_string())))
}

#[test]
fn set_0_l() {
    let bus = SimpleBus::new(vec![0xCB, 0xC5]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 0, L".to_string())))
}

#[test]
fn set_0_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0xC6]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 0, [HL]".to_string())))
}

#[test]
fn set_0_a() {
    let bus = SimpleBus::new(vec![0xCB, 0xC7]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 0, A".to_string())))
}

#[test]
fn set_1_b() {
    let bus = SimpleBus::new(vec![0xCB, 0xC8]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 1, B".to_string())))
}

#[test]
fn set_1_c() {
    let bus = SimpleBus::new(vec![0xCB, 0xC9]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 1, C".to_string())))
}

#[test]
fn set_1_d() {
    let bus = SimpleBus::new(vec![0xCB, 0xCA]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 1, D".to_string())))
}

#[test]
fn set_1_e() {
    let bus = SimpleBus::new(vec![0xCB, 0xCB]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 1, E".to_string())))
}

#[test]
fn set_1_h() {
    let bus = SimpleBus::new(vec![0xCB, 0xCC]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 1, H".to_string())))
}

#[test]
fn set_1_l() {
    let bus = SimpleBus::new(vec![0xCB, 0xCD]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 1, L".to_string())))
}

#[test]
fn set_1_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0xCE]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 1, [HL]".to_string())))
}

#[test]
fn set_1_a() {
    let bus = SimpleBus::new(vec![0xCB, 0xCF]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 1, A".to_string())))
}

// 0xDX
#[test]
fn set_2_b() {
    let bus = SimpleBus::new(vec![0xCB, 0xD0]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 2, B".to_string())))
}

#[test]
fn set_2_c() {
    let bus = SimpleBus::new(vec![0xCB, 0xD1]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 2, C".to_string())))
}

#[test]
fn set_2_d() {
    let bus = SimpleBus::new(vec![0xCB, 0xD2]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 2, D".to_string())))
}

#[test]
fn set_2_e() {
    let bus = SimpleBus::new(vec![0xCB, 0xD3]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 2, E".to_string())))
}

#[test]
fn set_2_h() {
    let bus = SimpleBus::new(vec![0xCB, 0xD4]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 2, H".to_string())))
}

#[test]
fn set_2_l() {
    let bus = SimpleBus::new(vec![0xCB, 0xD5]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 2, L".to_string())))
}

#[test]
fn set_2_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0xD6]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 2, [HL]".to_string())))
}

#[test]
fn set_2_a() {
    let bus = SimpleBus::new(vec![0xCB, 0xD7]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 2, A".to_string())))
}

#[test]
fn set_3_b() {
    let bus = SimpleBus::new(vec![0xCB, 0xD8]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 3, B".to_string())))
}

#[test]
fn set_3_c() {
    let bus = SimpleBus::new(vec![0xCB, 0xD9]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 3, C".to_string())))
}

#[test]
fn set_3_d() {
    let bus = SimpleBus::new(vec![0xCB, 0xDA]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 3, D".to_string())))
}

#[test]
fn set_3_e() {
    let bus = SimpleBus::new(vec![0xCB, 0xDB]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 3, E".to_string())))
}

#[test]
fn set_3_h() {
    let bus = SimpleBus::new(vec![0xCB, 0xDC]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 3, H".to_string())))
}

#[test]
fn set_3_l() {
    let bus = SimpleBus::new(vec![0xCB, 0xDD]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 3, L".to_string())))
}

#[test]
fn set_3_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0xDE]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 3, [HL]".to_string())))
}

#[test]
fn set_3_a() {
    let bus = SimpleBus::new(vec![0xCB, 0xDF]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 3, A".to_string())))
}

// 0xEX
#[test]
fn set_4_b() {
    let bus = SimpleBus::new(vec![0xCB, 0xE0]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 4, B".to_string())))
}

#[test]
fn set_4_c() {
    let bus = SimpleBus::new(vec![0xCB, 0xE1]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 4, C".to_string())))
}

#[test]
fn set_4_d() {
    let bus = SimpleBus::new(vec![0xCB, 0xE2]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 4, D".to_string())))
}

#[test]
fn set_4_e() {
    let bus = SimpleBus::new(vec![0xCB, 0xE3]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 4, E".to_string())))
}

#[test]
fn set_4_h() {
    let bus = SimpleBus::new(vec![0xCB, 0xE4]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 4, H".to_string())))
}

#[test]
fn set_4_l() {
    let bus = SimpleBus::new(vec![0xCB, 0xE5]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 4, L".to_string())))
}

#[test]
fn set_4_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0xE6]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 4, [HL]".to_string())))
}

#[test]
fn set_4_a() {
    let bus = SimpleBus::new(vec![0xCB, 0xE7]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 4, A".to_string())))
}

#[test]
fn set_5_b() {
    let bus = SimpleBus::new(vec![0xCB, 0xE8]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 5, B".to_string())))
}

#[test]
fn set_5_c() {
    let bus = SimpleBus::new(vec![0xCB, 0xE9]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 5, C".to_string())))
}

#[test]
fn set_5_d() {
    let bus = SimpleBus::new(vec![0xCB, 0xEA]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 5, D".to_string())))
}

#[test]
fn set_5_e() {
    let bus = SimpleBus::new(vec![0xCB, 0xEB]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 5, E".to_string())))
}

#[test]
fn set_5_h() {
    let bus = SimpleBus::new(vec![0xCB, 0xEC]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 5, H".to_string())))
}

#[test]
fn set_5_l() {
    let bus = SimpleBus::new(vec![0xCB, 0xED]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 5, L".to_string())))
}

#[test]
fn set_5_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0xEE]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 5, [HL]".to_string())))
}

#[test]
fn set_5_a() {
    let bus = SimpleBus::new(vec![0xCB, 0xEF]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 5, A".to_string())))
}

// 0xFX
#[test]
fn set_6_b() {
    let bus = SimpleBus::new(vec![0xCB, 0xF0]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 6, B".to_string())))
}

#[test]
fn set_6_c() {
    let bus = SimpleBus::new(vec![0xCB, 0xF1]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 6, C".to_string())))
}

#[test]
fn set_6_d() {
    let bus = SimpleBus::new(vec![0xCB, 0xF2]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 6, D".to_string())))
}

#[test]
fn set_6_e() {
    let bus = SimpleBus::new(vec![0xCB, 0xF3]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 6, E".to_string())))
}

#[test]
fn set_6_h() {
    let bus = SimpleBus::new(vec![0xCB, 0xF4]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 6, H".to_string())))
}

#[test]
fn set_6_l() {
    let bus = SimpleBus::new(vec![0xCB, 0xF5]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 6, L".to_string())))
}

#[test]
fn set_6_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0xF6]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 6, [HL]".to_string())))
}

#[test]
fn set_6_a() {
    let bus = SimpleBus::new(vec![0xCB, 0xF7]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 6, A".to_string())))
}

#[test]
fn set_7_b() {
    let bus = SimpleBus::new(vec![0xCB, 0xF8]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 7, B".to_string())))
}

#[test]
fn set_7_c() {
    let bus = SimpleBus::new(vec![0xCB, 0xF9]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 7, C".to_string())))
}

#[test]
fn set_7_d() {
    let bus = SimpleBus::new(vec![0xCB, 0xFA]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 7, D".to_string())))
}

#[test]
fn set_7_e() {
    let bus = SimpleBus::new(vec![0xCB, 0xFB]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 7, E".to_string())))
}

#[test]
fn set_7_h() {
    let bus = SimpleBus::new(vec![0xCB, 0xFC]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 7, H".to_string())))
}

#[test]
fn set_7_l() {
    let bus = SimpleBus::new(vec![0xCB, 0xFD]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 7, L".to_string())))
}

#[test]
fn set_7_hlr() {
    let bus = SimpleBus::new(vec![0xCB, 0xFE]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 7, [HL]".to_string())))
}

#[test]
fn set_7_a() {
    let bus = SimpleBus::new(vec![0xCB, 0xFF]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x00, &prefs);

    assert_eq!(result, Ok((2, "SET 7, A".to_string())))
}
