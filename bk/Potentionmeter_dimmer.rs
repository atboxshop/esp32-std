use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::thread::sleep;
use std::time::Duration;
use esp_idf_hal::prelude::*;
use esp_idf_hal::adc::*;
use esp_idf_hal::gpio::*;
use esp_idf_hal::ledc::*;
use esp_idf_hal::adc::config::Config;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::ledc::config::TimerConfig;


fn main()
{
    let peripherals = Peripherals::take().unwrap();

    let mut adc = AdcDriver::new(peripherals.adc2, &Config::new().calibration(true)).unwrap();
    let mut adc_pin: esp_idf_hal::adc::AdcChannelDriver<'_, Gpio12, Atten11dB<_>> =
        AdcChannelDriver::new(peripherals.pins.gpio12).unwrap();       
    let freq_conf = TimerConfig::new().frequency(120.Hz().into());
    let mut led = LedcDriver::new(peripherals.ledc.channel0,
        LedcTimerDriver::new(peripherals.ledc.timer0,
            &freq_conf).unwrap(),peripherals.pins.gpio14,).unwrap(); 
    loop {
        sleep(Duration::from_millis(20));
        let value1 = adc.read(&mut adc_pin).unwrap();
        let value2:f32 = value1.into(); //convert u16 to f32
        let value3:f32 = value2*(255.0/3054.0);//max value of pwn divided by max value of adc
        let value4:u32 = value3.round() as u32;
        println!("ADC value: {} vols", value2/1000.00);
        println!("{}",value4);
        led.set_duty(value4).unwrap();
   }
}