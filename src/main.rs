use std::error::Error;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, Trigger};

const GPIO_REMOTE: u8 = 20;
const GPIO_SW0: u8 = 19;
const GPIO_SW1: u8 = 27;

fn main() -> Result<(), Box<dyn Error>> {
    let mut remote_pin = Gpio::new()?.get(GPIO_REMOTE)?.into_input_pullup();
    let mut sw0_pin = Gpio::new()?.get(GPIO_SW0)?.into_output();
    let mut sw1_pin = Gpio::new()?.get(GPIO_SW1)?.into_output();

    let running = Arc::new(AtomicBool::new(true));

    let trigger = Trigger::Both;
    let remote_interrupt = remote_pin.set_async_interrupt(trigger, move |x| match x {
        rppal::gpio::Level::Low => {
            println!("pressed");
            sw0_pin.toggle();
            sw1_pin.toggle();
        }
        rppal::gpio::Level::High => {
            println!("depressed");
        }
    });
    if let Err(e) = remote_interrupt {
        panic!("{e:?}")
    }

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
