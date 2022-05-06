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
use std::ops::Deref;
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

    let mut trigger_pin = gpio.get(23)?.into_output();
    let mut echo_pin = gpio.get(24)?.into_input();

    trigger_pin.set_low();

    println!("init echo is_high: {}", echo_pin.is_high());

    let pulse: Arc<Mutex<Pulse>> = Arc::new(Mutex::new(Pulse::empty()));
    let pulse_cloned = pulse.clone();
    echo_pin.set_async_interrupt(Trigger::Both, move |level| {
        let instant = std::time::Instant::now();
        if level == Level::High {
            println!("echo raising: {:?}", instant);
            pulse_cloned.lock().unwrap().start = Some(instant);
        }
        else if level == Level::Low {
            println!("echo falling: {:?}", instant);
            pulse_cloned.lock().unwrap().end = Some(instant);
        }
    })?;

    // measure distance
    trigger_pin.set_high();
    thread::sleep(Duration::from_micros(10));
    trigger_pin.set_low();

    
    
    // Sleep for 10 seconds while the LED blinks.
    thread::sleep(Duration::from_secs(1));
    
    echo_pin.clear_async_interrupt()?;
    
    let pulse_length = pulse.lock().unwrap().length();
    
    if let Some(duration) = pulse_length {
        let distance = duration.as_secs_f64() * 17150_f64;
        println!("Distance: {} cm", distance);
    }
    else {
        println!("No distance measured");
    }

    // wait before running motors
    thread::sleep(Duration::from_secs(10));
    
    // Enable PWM channel 0 (BCM GPIO 18, physical pin 12) at 2 Hz with a 25% duty cycle.
    let pwm0 = Pwm::with_frequency(Channel::Pwm0, 100.0, 0.25, Polarity::Normal, true)?;
    let pwm1 = Pwm::with_frequency(Channel::Pwm1, 100.0, 0.25, Polarity::Normal, true)?;

    for i in 0..=100 {
        pwm0.set_frequency(100.0, i as f64 * 0.01f64)?;
        pwm1.set_frequency(100.0, i as f64 * 0.01f64)?;
        thread::sleep(Duration::from_millis(100));
    }
    for i in 0..=100 {
        pwm0.set_frequency(100.0, 1_f64 - (i as f64 * 0.01f64))?;
        pwm1.set_frequency(100.0, 1_f64 - (i as f64 * 0.01f64))?;
        thread::sleep(Duration::from_millis(100));
    }
        
    Ok(())

    // When the pwm variable goes out of scope, the PWM channel is automatically disabled.
    // You can manually disable the channel by calling the Pwm::disable() method.
}