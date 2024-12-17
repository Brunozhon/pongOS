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
   2. Windows: [Download this File](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)
3. Clone this GitHub repository.
```bash
git clone https://github.com/Brunozhon/pongOS && cd pongOS
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
   1. On QEMU:
   ```bash
   qemu-system-x86_64 -drive format=raw,file=target/x86_64-pongOS/debug/bootimage-pongOS.bin
   ```
   2. On a real machine:
      1. Plug a USB flash drive into your computer. **It has to be at least 8GB!**
      2. Overwrite your flash drive with the image, replacing `DRIVE` with the drive name:
      ```bash
      dd if=target/x86_64-blog_os/debug/bootimage-blog_os.bin of=/dev/DRIVE && sync
      ```
      3. Plug it in to a test computer, preferably running the x86_64 architecture and running Windows OS.
      4. Inside "Settings," press *System > Recovery* on Windows 11 and *Settings > Update & Security > Recovery* on Windows 10.
      5. Select the "Restart Now" button to the right of *Advanced Startup*.
      6. In the recovery environment, navigate to "Use a Device" and choose your USB flash drive.
      7. Your computer should reboot and boot into Pong OS!
8. Have fun playing Pong!