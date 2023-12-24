/* Code and comments from: https://os.phil-opp.com/freestanding-rust-binary/  */

/*  This is a bare metal implementation. This means that the final binary won't
    be linked to the standard library that interacts with the operating system,
    since there is not operating system (hence, bare metal).
    no_std tells the compiler not to link the standard library. */
#![no_std]

/*  Freestanding executable, no access to Rust runtime and crt0.
    We need to define our own entry point overwriting the crt0 one directly.
    no_main tells the compiler not to use the normal entry point chain. */
#![no_main]

/*  Add the message() method to the PanicInfo struct in order to retrieve the
    reason and print it to the screen when the panic_handler is triggered. */
#![feature(panic_info_message)]

/* ==== ENTRY POINT ========================================================= */
/*  All the code written here is underneath the .text._start section.
    The _start section is then placed above all else by the linker script. */
    core::arch::global_asm!(".section .text._start");

    mod lib_to_string;
    use crate::lib_to_string::print_string;
    use crate::lib_to_string::ToString;
    
/*  A main doesn’t make sense without an underlying runtime that calls it.
    We are overwriting the os entry point with our own _start function.
    The no_mangle attribute ensures that the compiler outputs the
    function with name _start and not some cryptic unique name symbol.
    Required since we need to tell the entry point name to the linker.
    "_start" is the default entry point name for most systems.
    We also have to mark the function as extern "C" to tell the compiler that
    it should use the C calling convention for this function.
    (https://en.wikipedia.org/wiki/X86_calling_conventions) */
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // DS, SS, SS registries are set to 0x1000, SP is set to 0 by the stage-1.
    // Since the Stack Pointer grows backwards, it will wrap around the segment.
    // The stack will still override our code if we reach 64kB.
    //! In this environment there are no "stack overflow" guards
    
    // Only works in --release...
    print!("u8: ", 8u8, " u16: ", 16u16, " u32: ", 32u32, " u64: ", 64u64);

    // Do nothing until the end of time
    loop {}
}

/* ==== PANIC HANDLER ======================================================= */
use core::panic::PanicInfo;

/*  panic_handler defines the method that is invoked when a panic occurs.
    In a no_std environment we need to define it ourselves. */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    // Print panic reason
    //print_s(_info.message().unwrap().as_str().expect("Panic!"));
    print!("Panic!");

    // Do nothing until the end of time
    loop {}
}





/* ==== TEST ================================================================ *
/*  Export print macro to print any ToString implementing type. */
#[macro_export]
macro_rules! print {
    ($($arg:expr),*) => {
        {
            $(
                let s = $arg.to_string();
                print_string(s);
                // for c in s.chars() { unsafe { _print_char(c as u8, 0); } }
            )*
        }
    };
}

/*  Define ToString trait so that we can implement a custom to_string function
    for each type we need to print with the print! macro.
    The trait is implement by defining a to_string method that returns a
    reference to a static string. */
trait ToString { fn to_string(&self) -> &'static str; }

/*  To implement the conversion to unsigned integer to string, we can use the
    same logic for each type, so a macro is used to generate the same impl
    for u8, u16, u32 and u64. */
#[macro_export]
macro_rules! to_string_impl_uint {
    // Match ty: type for which we are implementing the to_string method.
    // Match size: maximum size of the string representation of the number type.
    ($ty:ty, $size:expr) => {

        // Start code implementing ToString trait for the matched uint type
        impl ToString for $ty {
            fn to_string(&self) -> &'static str {

                // Static buffer to store ASCII representation without allocator
                static mut BUFFER: [u8; $size] = [0; $size];

                // Init buffer index to last char
                let mut i = $size - 1;

                // The number to print is self, the u8 we're writing the impl for
                let mut num = *self;

                unsafe {
                    loop {
                        // Get the rightmost number getting the /10 remainder
                        BUFFER[i] = b'0' + (num % 10) as u8;

                        // Actually divide to "shift" the number to the right
                        num /= 10;

                        // If there are no numbers left to print, exit
                        if num == 0 { break; }

                        // Change buffer index
                        i -= 1;
                    }

                    // Convert the u8 array to utf8 string (unchecked)
                    core::str::from_utf8_unchecked(&BUFFER[i..])
                }
            }
        }
    };
}

/*  Implement the ToString trait for the static &str type: return self. */
impl ToString for &'static str { fn to_string(&self) -> &'static str { *self } }

/*  Implement the ToString trait for u8 using the to_string_impl_uint macro. 
    Max size: 3 chars (0-255). */
to_string_impl_uint!(u8, 3);

/*  Implement the ToString trait for u16 using the to_string_impl_uint macro.
    Max size: 5 chars (0-65_535). */
to_string_impl_uint!(u16, 5);

/*  Implement the ToString trait for u32 using the to_string_impl_uint macro.
    Max size: 10 chars (0-4_294_967_295). */
to_string_impl_uint!(u32, 10);

/*  Implement the ToString trait for u64 using the to_string_impl_uint macro.
    Max size: 20 chars (0-18_446_744_073_709_551_615). */
to_string_impl_uint!(u64, 20);
*/