# kurumi (くるみ)

kurumi is a toy os implemented in Rust. It is an experimental project.

![Imgur](https://i.imgur.com/seoYuqw.gif)

### Progress

- [X] Boot
- [X] vga output
- [X] interrupt
- [X] keyboard
- [ ] tty
- [ ] context
- [ ] system call
- [X] memory(follow blog_os)
- [X] file system(FAT32)
- [ ] console
- [ ] process

### Build
It depend on Rust nightly, Xargo, nasm, xorriso, qemu.

In debian
```
$ apt-get install nasm \
    binutils           \
    grub-common        \
    xorriso            \
    grub-pc-bin        \
    qemu
$ cargo install xargo
$ rustup component add rust-src
```

### Run

```
$ make iso
$ make run
```

### Reference
[Linux内核设计与实现](https://book.douban.com/subject/6097773/)  
[Linux内核0.11完全注释](https://github.com/loveveryday/linux0.11)  
[30天自制操作系统](https://book.douban.com/subject/11530329/)  
[Stanford CS140e - Operating Systems](https://web.stanford.edu/class/cs140e/)  
[Writing an OS in Rust](https://os.phil-opp.com/)  
[Redox-kernel](https://github.com/redox-os/kernel)  
