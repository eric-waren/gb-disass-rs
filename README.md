# `gb-disass-rs`

Rust library to disassemble Game Boy binary

## Interfaces

```rust
/// Trait to be implemented by the `disass` function caller. This allows the `disass` function to
/// access the binary Game Boy data.
pub trait MemoryBus {
    fn read_byte(&self, addr: u16) -> Option<u8>;
    fn read_word(&self, addr: u16) -> Option<u16>;
}

/// Display preferences for the `disass` function.
///
/// * `upcase`: return the textual representation as UPCASE letters (including hexadecimal)
/// * `comma_space`: add or a not a space after a comma with 2 operands (e.g. `ld a, b`)
pub struct Preferences {
    upcase: bool,
    comma_space: bool,
}

/// Return a textual representation of a Game Boy binary operation, compatible with the RGBDS syntax in a `String`.
///
/// # Example
/// ```ignore
/// let bus = GameboyBus::new(vec![0x01, 0x12, 0x34]);
/// let prefs = Preferences{upcase: true, comma_space: true};
/// let result = disass(&bus, 0x0, &prefs);
///
/// assert_eq!(result, Ok((3, "LD BC, $1234".to_string())))
/// ```
///
/// # Errors
///
/// * The operation needs one or two operands but an insufficient number is found
/// * The opcode isn't a valid Game Boy operation (unsupported)
///
/// # Result
///
/// Returns a tuple containing the number of bytes (as a u16) consumed and the textual representation in a
/// String.
///
/// The byte count number can be used to increment a PC register in an emulator.
pub fn disass(bus: &impl MemoryBus, addr: u16, prefs: &Preferences) -> Result<(u16, String), String>;
```

## Examples

```rust
use gb_disass_rs::{MemoryBus, Preferences, disass};

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

fn main() {
    let bus = GameboyBus::new(vec![0x01, 0x12, 0x34]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disass(&bus, 0x0, &prefs);

    assert_eq!(result, Ok((3, "LD BC, $1234".to_string())))
}
```

## LICENSE
```
Copyright 2026 Eric Waren

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the “Software”), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```
