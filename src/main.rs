// pwm_blinkled.rs - Blinks an LED using hardware PWM.
//
// Remember to add a resistor of an appropriate value in series, to prevent
// exceeding the maximum current rating of the GPIO pin and the LED.
//
// Interrupting the process by pressing Ctrl-C causes the application to exit
// immediately without disabling the PWM channel. Check out the
// gpio_blinkled_signals.rs example to learn how to properly handle incoming
// signals to prevent an abnormal termination.

use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::gpio::{Trigger, Level};

struct Pulse {
    start: Option<std::time::Instant>,
    end: Option<std::time::Instant>,
}

impl Pulse {
    fn empty() -> Self {
        Pulse { start: None, end: None }
    }

    fn length(&self) -> Option<std::time::Duration> {
        match (self.start, self.end) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            _ => None
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let gpio = rppal::gpio::Gpio::new()?;

    let mut forward1_pin = gpio.get(22)?.into_output();
    let mut backward1_pin = gpio.get(23)?.into_output();
    let mut forward2_pin = gpio.get(25)?.into_output();
    let mut backward2_pin = gpio.get(24)?.into_output();

    forward1_pin.set_high();
    backward1_pin.set_low();

    forward2_pin.set_high();
    backward2_pin.set_low();

    
    // Enable PWM channel 0 (BCM GPIO 18, physical pin 12) at 2 Hz with a 25% duty cycle.
    let pwm0 = Pwm::with_frequency(Channel::Pwm0, 100.0, 1.0, Polarity::Normal, false)?;
    let pwm1 = Pwm::with_frequency(Channel::Pwm1, 100.0, 1.0, Polarity::Normal, false)?;
    
    // wait before running motors
    thread::sleep(Duration::from_secs(1));

    pwm0.enable()?;
    pwm1.enable()?;

    for i in 25..=100 {
        pwm0.set_frequency(100.0, i as f64 * 0.01f64)?;
        pwm1.set_frequency(100.0, i as f64 * 0.01f64)?;
        thread::sleep(Duration::from_millis(100));
    }
    
    // for i in 0..=100 {
    //     pwm0.set_frequency(100.0, 1_f64 - (i as f64 * 0.01f64))?;
    //     pwm1.set_frequency(100.0, 1_f64 - (i as f64 * 0.01f64))?;
    //     thread::sleep(Duration::from_millis(100));
    // }
        
    Ok(())

    // When the pwm variable goes out of scope, the PWM channel is automatically disabled.
    // You can manually disable the channel by calling the Pwm::disable() method.
}