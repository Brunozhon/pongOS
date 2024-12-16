#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod vga_buffer;
mod interrupts;
mod gdt;

use core::panic::PanicInfo;
use core::ptr::write;
use lazy_static::lazy_static;
use crate::vga_buffer::{write_byte, Color, ColorCode};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    write_byte(0, 0, b':', *RED);
    write_byte(1, 0, b'(', *RED);
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

lazy_static! {
    static ref WHITE: ColorCode = ColorCode::new(Color::White, Color::Black);
    static ref RED: ColorCode = ColorCode::new(Color::LightRed, Color::Black);
    static ref PINK: ColorCode = ColorCode::new(Color::Pink, Color::Black);
    static ref MAGENTA: ColorCode = ColorCode::new(Color::Magenta, Color::Black);
}

static mut ticks: u64 = 0;

static mut paddleA: usize = 17;
static mut aScore: u8 = 0;

static mut paddleB: usize = 17;
static mut bScore: u8 = 0;

static mut ballX: usize = 39;
static mut ballY: usize = 17;
static mut dx: bool = true;
static mut dy: bool = true;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    loop {
        x86_64::instructions::hlt();
    }
}

unsafe fn key_pressed(key: char) {
    if key == 'w' {
        write_byte(paddleA + 2, 0, b' ', *WHITE);
        paddleA -= 1;
        if paddleA < 2 { paddleA = 2; }
    } else if key == 's' {
        write_byte(paddleA - 2, 0, b' ', *WHITE);
        paddleA += 1;
        if paddleA > 22 { paddleA = 22; }
    } else if key == 'i' {
        write_byte(paddleB + 2, 79, b' ', *WHITE);
        paddleB -= 1;
        if paddleB < 2 { paddleB = 2; }
    } else if key == 'k' {
        write_byte(paddleB - 2, 79, b' ', *WHITE);
        paddleB += 1;
        if paddleB > 22 { paddleB = 22; }
    }
}

fn char_from_score(score: u8) -> u8 {
    match score {
        0 => b'0',
        1 => b'1',
        2 => b'2',
        3 => b'3',
        4 => b'4',
        5 => b'5',
        6 => b'6',
        7 => b'7',
        8 => b'8',
        _ => b'9'
    }
}

unsafe fn update() {
    write_byte(0, 37, b'P', *PINK);
    write_byte(0, 38, b'o', *PINK);
    write_byte(0, 39, b'n', *PINK);
    write_byte(0, 40, b'g', *PINK);
    write_byte(0, 41, b'O', *MAGENTA);
    write_byte(0, 42, b'S', *MAGENTA);

    write_byte(paddleA - 2, 0, b'#', *WHITE);
    write_byte(paddleA - 1, 0, b'#', *WHITE);
    write_byte(paddleA, 0, b'#', *WHITE);
    write_byte(paddleA + 1, 0, b'#', *WHITE);
    write_byte(paddleA + 2, 0, b'#', *WHITE);

    write_byte(paddleB - 2, 79, b'#', *WHITE);
    write_byte(paddleB - 1, 79, b'#', *WHITE);
    write_byte(paddleB, 79, b'#', *WHITE);
    write_byte(paddleB + 1, 79, b'#', *WHITE);
    write_byte(paddleB + 2, 79, b'#', *WHITE);

    write_byte(ballY, ballX, b' ', *WHITE);

    if ballX + 1 > 78 && ticks % 2 == 0 &&
        (ballY <= paddleB + 2 && ballY >= paddleB - 2) {
        dx = !dx;
    } else if ballX < 2 && ticks % 2 == 0 &&
        (ballY <= paddleA + 2 && ballY >= paddleA - 2) {
        dx = !dx;
    } else if ballX < 1 || ballX + 1 > 79 {
        if ballX < 1 {
            bScore = bScore.saturating_add_signed(1);
        } else {
            aScore = aScore.saturating_add_signed(1);
        }
        ballX = 39;
        ballY = 12;
    }

    if (ballY + 1 > 24 || ballY < 1) && ticks % 2 == 0 {
        dy = !dy;
    }

    if dy && ticks % 2 == 0 {
        ballY += 1;
    } else if ticks % 2 == 0 {
        ballY -= 1;
    }

    if dx {
        ballX += 1;
    } else {
        ballX -= 1;
    }

    write_byte(ballY, ballX, b'O', *RED);

    write_byte(0, 35, char_from_score(aScore), *WHITE);
    write_byte(0, 44, char_from_score(bScore), *WHITE);

    ticks += 1;
}