// simple file implementation

use alloc::string::String;

pub trait File {
    fn get_name(&self) -> String;
    fn get_size(&self) -> usize;
}

pub enum FileMode {
    Read,
    Write,
    ReadWrite,
}

pub struct FilePointer<T: File> {
    file: T,
    pos:  usize, // cursor
}

impl <T: File> FilePointer<T> {
    pub fn new(file: T, pos: usize,) -> FilePointer<T> {
        FilePointer {
            file: file,
            pos:  pos,
        }
    }
    pub fn get_pos(&self) -> usize {
        self.pos
    }

    pub fn advance_pointer(&mut self, amount: usize) {
        self.pos += amount
    }

    pub fn get_file(&self) -> &T {
        &self.file
    }
}

pub struct FileDescriptor<T: File> {
    id:      u16,
    mode:    FileMode,
    pointer: FilePointer<T>
}

impl <T: File> FileDescriptor<T> {
    pub fn new(id: u16, mode: FileMode, pointer: FilePointer<T>) -> FileDescriptor<T> {
        FileDescriptor {
            id:      id,
            mode:    mode,
            pointer: pointer,
        }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_pointer(&self) -> &FilePointer<T> {
        &self.pointer
    }

    pub fn get_pointer_mut(&mut self) -> &mut FilePointer<T> {
        &mut self.pointer
    }
}
