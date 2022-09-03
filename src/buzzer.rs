use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const GPIO_ISO: u8 = 17;

fn buzz() -> Result<(), Box<dyn Error>> {
    println!("activating optoisolator on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_ISO)?.into_output();

    pin.set_high();
    thread::sleep(Duration::from_secs(1));
    pin.set_low();

    Ok(())
}