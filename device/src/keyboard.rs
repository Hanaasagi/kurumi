// PS/2 keyboard scancode
// http://www.computer-engineering.org/ps2keyboard/scancodes1.html

use io::inb;
use spin::Mutex;

// Scancodes range 0x01 ... 0x0E
const ASCII_PART_1: &'static [u8; 14] = b"\x1B1234567890-=\x08";
// Scancodes range 0x0F ... 0x1C
const ASCII_PART_2: &'static [u8; 14] = b"\tqwertyuiop[]\n";
// Scancodes range 0x1E ... 0x29
const ASCII_PART_3: &'static [u8; 12] = b"asdfghjkl;'`";
// Scancodes range 0x2C ... 0x35
const ASCII_PART_4: &'static [u8; 10] = b"zxcvbnm,./";

#[derive(Copy, Clone)]
struct Scancode(u8);

impl Scancode {

    fn to_ascii(&self) -> Option<u8> {
        let code = self.0 as usize;
        match code {
            0x01 ... 0x0e => Some(ASCII_PART_1[code - 0x01]),
            0x0f ... 0x1c => Some(ASCII_PART_2[code - 0x0f]),
            0x1e ... 0x29 => Some(ASCII_PART_3[code - 0x1e]),
            0x2c ... 0x35 => Some(ASCII_PART_4[code - 0x2c]),
            0x2b          => Some(b'\\'),
            0x39          => Some(b' '), // SPACE
            _             => None,
        }
    }
}

// PS/2 keyboard state
struct Keyboard {
    scancode: Scancode,
    state: Modifiers
}

impl Keyboard {

    #[inline]
    fn read_scancode(&mut self) {
        self.scancode = Scancode(unsafe {inb(0x60)});
    }

    #[inline]
    fn update(&mut self) {
        self.state.update(self.scancode);
    }

    fn read_char(&mut self) -> Option<char>{
        self.read_scancode();
        self.update();
        self.scancode.to_ascii().map(|ascii| {
            self.state.modify(ascii) as char
        })
    }
}

bitflags! {

    struct Modifiers: u8 {
        const L_SHIFT  = 0b_1000_0000;
        const R_SHIFT  = 0b_0100_0000;
        const R_CTRL   = 0b_0010_0000;
        const L_CTRL   = 0b_0001_0000;
        const R_ALT    = 0b_0000_1000;
        const L_ALT    = 0b_0000_0100;
        const CAPSLOCK = 0b_0000_0010;
        const NUMLOCK  = 0b_0000_0001;
    }
}

impl Modifiers {

    const fn new() -> Self {
        Modifiers { bits: 0x00 }
    }

    // Returns true if either shift key is pressed.
    #[inline]
    fn is_shifted(&self) -> bool {
        self.contains(Self::L_SHIFT) || self.contains(Self::R_SHIFT)
    }

    // Returns true if the keyboard's state is currently uppercase.
    #[inline]
    fn is_uppercase(&self) -> bool {
        self.is_shifted() ^ self.contains(Self::CAPSLOCK)
    }

    fn update(&mut self, scancode: Scancode) {
        match scancode {
            Scancode(0x1D) => self.insert(Self::L_CTRL),
            Scancode(0x2A) => self.insert(Self::L_SHIFT),
            Scancode(0x36) => self.insert(Self::R_SHIFT),
            Scancode(0x38) => self.insert(Self::L_ALT),
            // Caps lock toggles on leading edge
            Scancode(0x3A) => self.toggle(Self::CAPSLOCK),
            Scancode(0x9D) => self.remove(Self::L_CTRL),
            Scancode(0xAA) => self.remove(Self::L_SHIFT),
            Scancode(0xB6) => self.remove(Self::R_SHIFT),
            Scancode(0xB8) => self.remove(Self::L_ALT),
            _              => {},
        }
    }

    // Apply the keyboard's modifiers to an ASCII scancode.
    fn modify(&self, ascii: u8) -> u8 {
        match ascii {
            b'a' ... b'z' if self.is_uppercase() => ascii - b'a' + b'A',
            b'1' ... b'9' if self.is_shifted()   => ascii - b'1' + b'!',
            b'0' if self.is_shifted()            => b')',
            _                                    => ascii
        }
    }
}

static KEYBOARD: Mutex<Keyboard> = Mutex::new(Keyboard {
    scancode: Scancode(0x00),
    state: Modifiers::new()
});

pub fn read_char() -> Option<char> {
    KEYBOARD.lock().read_char()
}
