// Auto-generated. Do not modify this file! Instead modify examples/override-interrupt.rs
//! Override the TIM7 (or any other) interrupt handler
//!
//! ``` rust,no_run
//! #![feature(asm)]
//! #![no_main]
//! #![no_std]
//!
//! #[macro_use]
//! extern crate cortex_m;
//! extern crate f3;
//!
//! use f3::delay;
//!
//! #[export_name = "main"]
//! pub extern "C" fn main() -> ! {
//!     // This function uses the TIM7 interrupt under the hood. After a second has
//!     // passed, the `_tim7` interrupt handler will be called and ...
//!     delay::ms(1_000);
//!
//!     loop {}
//! }
//!
//! #[export_name = "_tim7"]  // <-- Important! Note the underscore.
//! pub extern "C" fn my_tim7_interrupt_handler() {
//!     unsafe {
//!         // .. you'll reach THIS breakpoint!
//!         bkpt!();
//!     }
//!
//!     loop {}
//! }
//! ```
