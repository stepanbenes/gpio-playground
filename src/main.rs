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

use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::gpio::{Trigger};

fn main() -> Result<(), Box<dyn Error>> {

    let gpio = rppal::gpio::Gpio::new()?;

    let mut trigger_pin = gpio.get(23)?.into_output();
    let mut echo_pin = gpio.get(24)?.into_input();

    println!("init echo is_high: {}", echo_pin.is_high());

    echo_pin.set_async_interrupt(Trigger::Both, |level| println!("echo: {}", level))?;

    //trigger_pin.set_low();
    
    // Enable PWM channel 0 (BCM GPIO 18, physical pin 12) at 2 Hz with a 25% duty cycle.
    let pwm = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true)?;
        
    // Sleep for 2 seconds while the LED blinks.
    thread::sleep(Duration::from_secs(10));
    
    trigger_pin.set_high();
    trigger_pin.set_low();

    for i in 0..2000 {
        pwm.set_frequency(100.0, (i % 100) as f64 * 0.01f64)?;
        thread::sleep(Duration::from_millis(10));
    }



    Ok(())

    // When the pwm variable goes out of scope, the PWM channel is automatically disabled.
    // You can manually disable the channel by calling the Pwm::disable() method.
}