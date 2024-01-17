#![no_std]
#![no_main]

// 3F20_0008 fsel2 1<<3 turn pin one into an output
// 3F20_001c gpio1_set 1<<21 turns pin 21 on
// 3F20_0028 gpio1_clear 1<<21 turns pin 21 off

const GPIO_FSEL0: u32 = 0x3F20_0000;
const GPIO_FSEL1: u32 = 0x3F20_0004;
const GPIO_FSEL2: u32 = 0x3F20_0008;

const GPIO_SET0: u32 = 0x3F20_001c;
const GPIO_CLR0: u32 = 0x3F20_0028;

use core::arch::asm;
use core::panic::PanicInfo;

struct GPIO;

impl GPIO {
    pub fn set_output(pin: u32) {
        let reg = pin / 10;
        let register = match reg {
            0 => GPIO_FSEL0,
            1 => GPIO_FSEL1,
            2 => GPIO_FSEL2,
            _ => panic!("Something has gone terribly wrong"),
        };

        let mut val: u32;

        unsafe {
            val = core::ptr::read_volatile(register as *mut u32);
        }

        // create mask
        let mut mask: u32 = 0b111;

        // shift the mask to the right location
        let pinnum = pin % 10;
        mask = mask << pinnum * 3;

        // and in the NOT of the mask
        val = val & !(mask);

        // set OUR value
        val |= 1 << pinnum * 3;

        unsafe {
            core::ptr::write_volatile(register as *mut u32, val);
        }
    }

    pub fn set(pin: u32) {
        let bitpos: u32 = pin;

        let mut val: u32 = unsafe { core::ptr::read_volatile(GPIO_SET0 as *const u32) };

        val |= 1 << bitpos;

        unsafe { core::ptr::write_volatile(GPIO_SET0 as *mut u32, val) };
    }

    pub fn clear(pin: u32) {
        let bitpos: u32 = pin;

        let mut val: u32 = unsafe { core::ptr::read_volatile(GPIO_CLR0 as *const u32) };

        val |= 1 << bitpos;

        unsafe { core::ptr::write_volatile(GPIO_CLR0 as *mut u32, val) };
    }
}

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    GPIO::set_output(21);

    loop {
        // turn pin on
        GPIO::set(21);

        for _ in 1..50000 {
            unsafe{asm!("nop")};
        }

        // turn pin off
        GPIO::clear(21);

        for _ in 1..50000 {
            unsafe{asm!("nop")};
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
