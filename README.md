# Pong OS

An OS written in Rust that only runs Pong.

## Installation

1. Install QEMU.
   1. MacOS:
   ```bash
   # Homebrew
   brew install qemu
   # MacPorts
   sudo port install qemu
   ```
   2. Windows
   ```bash
   # 64-bit Windows 8.1 and above
   pacman -S mingw-w64-ucrt-x86_64-qemu
   ```
   3. Ubuntu/Debian
   ```bash
   # Full system emulation
   apt-get install qemu-system
   # Emulating Linux binaries
   apt-get install qemu-user-static
   ```
   4. From source:
   ```bash
   wget https://download.qemu.org/qemu-9.2.0.tar.xz
   tar xvJf qemu-9.2.0.tar.xz
   cd qemu-9.2.0
   ./configure
   make
   ```
2. Install Rust, Rustup, and Cargo.
   1. MacOS/Linux/Unix-based OS
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   2. Windows: [Download this File](rustup-init.exe)
3. Clone this GitHub repository.
```bash
git clone https://github.com/Brunozhon/pongOS
```
4. Override the Rust compiler to use Rust nightly for this directory:
```bash
rustup override set nightly
```
5. Install a crate called `bootimage`.
```bash
cargo install bootimage
```
6. Compile this project.
```bash
cargo bootimage
```
7. Run this project.
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-pongOS/debug/bootimage-pongOS.bin
```
8. Have fun playing Pong!