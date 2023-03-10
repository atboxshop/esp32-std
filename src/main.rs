use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main()
{
    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio9).unwrap();


    loop {
        led.set_high().unwrap();
        // we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(500);
        led.set_low().unwrap();
        FreeRtos::delay_ms(500);
        println!("Hello World! 123");
   }
}