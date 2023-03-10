use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::thread::sleep;
use std::time::Duration;
use esp_idf_hal::adc::*;
use esp_idf_hal::gpio::*;
use esp_idf_hal::adc::config::Config;
use esp_idf_hal::peripherals::Peripherals;



fn main()
{
    let peripherals = Peripherals::take().unwrap();

    let mut adc = AdcDriver::new(peripherals.adc2, &Config::new().calibration(true)).unwrap();
    let mut adc_pin: esp_idf_hal::adc::AdcChannelDriver<'_, Gpio13, Atten11dB<_>> =
        AdcChannelDriver::new(peripherals.pins.gpio13).unwrap();       

    loop {
        sleep(Duration::from_millis(20));
        let value1 = adc.read(&mut adc_pin).unwrap();
        let value2:f32 = value1.into();
        println!("ADC value: {} vols", value2/1000.00);
   }
}