use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

/*
INPUT | remote (high when pressed)
grnd red 40
d+ black 36

OUTPUT | cable (high when pressed)
grnd black
d+ green
*/
const GPIO_REMOTE: u8 = 23;
const GPIO_SW0: u8 = 23;
const GPIO_SW1: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_REMOTE)?.into_input();
    let mut pin = Gpio::new()?.get(GPIO_SW0)?.into_output();
    let mut pin = Gpio::new()?.get(GPIO_SW1)?.into_output();

    pin.set_high();
    thread::sleep(Duration::from_millis(500));
    pin.set_low();

    Ok(())
}
