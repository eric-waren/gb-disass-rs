# `gb-disass-rs`

Rust library to disassemble Game Boy binary

## Install

Via CLI

```sh
cargo add gb-disass-rs
```

Via `Cargo.toml`

```toml
[dependencies]
gb-disass-rs = "*"
```

## Interfaces

These are the usable exported constructs. For more information on their usage, please read the [documentation](https://docs.rs/gb-disass-rs/latest/gb_disass_rs/)

```rust
pub enum Mnemonic;
pub enum Operand;

pub type Operation = (Mnemonic, Vec<Operand>);

pub trait MemoryBus {
    fn read_byte(&self, addr: u16) -> Option<u8>;
    fn read_word(&self, addr: u16) -> Option<u16>;
}

pub struct Preferences {
    upcase: bool,
    comma_space: bool,
}

pub struct SimpleBus {
    data: Vec<u8>,
}

pub fn decode(bus: &impl MemoryBus, addr: u16) -> Result<Operation, String>;
pub fn render(operation: &Operation, prefs: &Preferences) -> Result<String, std::fmt::Error>;
pub fn next_operation_offset(operation: &Operation) -> u16;
pub fn disassemble(bus: &impl MemoryBus, addr: u16, prefs: &Preferences) -> Operation;
```

## Examples

```rust
use gb_disass_rs::{SimpleBus, Preferences, disassemble};

fn main() {
    let bus = SimpleBus::new(vec![0x01, 0x34, 0x12]);
    let prefs = Preferences{upcase: true, comma_space: true};
    let result = disassemble(&bus, 0x0, &prefs);

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
