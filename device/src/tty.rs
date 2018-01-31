// tty is a device used to user's input & output
// input come form keyboard
// output to screen via vga.c

extern crate spin;

use spin::Mutex;

const NTTY_BUF: u32 = 512;

#[allow(non_camel_case_types)]
pub struct TTY_Buf {
    nread:  u32,
    nwrite: u32,
    buf:    [char; NTTY_BUF as usize],
}

impl TTY_Buf {

    const fn new() -> Self {
        TTY_Buf {
            nread:  0,
            nwrite: 0,
            buf:    ['\0'; NTTY_BUF as usize],
        }
    }

    fn check_full(&self) {
        if self.nwrite == self.nread + NTTY_BUF {
            panic!();
        }
    }

    pub fn input(&mut self, ch: char) {
        self.check_full();
        let pos = (self.nwrite  % NTTY_BUF) as usize;
        self.nwrite += 1;
        self.buf[pos] = ch;
        self.read();
    }

    fn read(&mut self) {
        for i in self.nread..self.nwrite {
            let ch = self.buf[i as usize];
            kprint!("{}", ch);
        }
        self.nread = self.nwrite;
    }
}

pub static TTY_BUF: Mutex<TTY_Buf> = Mutex::new(TTY_Buf::new());
