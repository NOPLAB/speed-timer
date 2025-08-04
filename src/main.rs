use ag_lcd::LcdDisplay;
use esp_idf_svc::hal::{delay::Delay, i2c::{I2cConfig, I2cDriver}, prelude::Peripherals,  timer::*};
use port_expander::Pcf8574;
use esp_idf_svc::hal::prelude::*;

static mut TIMER_COUNTER: u64 = 0;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let p = Peripherals::take()?;

    let delay = Delay::new_default();

    let sda = p.pins.gpio12;
    let scl = p.pins.gpio13;

    let i2c_config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(p.i2c0, sda, scl, &i2c_config)?;
    let mut i2c_expander = Pcf8574::new(i2c, true, true, true);

    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new_pcf8574(&mut i2c_expander, delay).with_cursor(ag_lcd::Cursor::Off).with_lines(ag_lcd::Lines::TwoLines).build();

    lcd.clear();

    lcd.set_position(0, 1);
    lcd.print("Hello, world!");

    let timer0_config = config::Config::new().auto_reload(true);
    let mut timer0 = TimerDriver::new(p.timer00, &timer0_config)?;
    timer0.set_alarm(timer0.tick_hz() / 1000)?;

    unsafe {
        timer0.subscribe(move || {
            TIMER_COUNTER += 1;
        })?;
    }

    timer0.enable_interrupt()?;
    timer0.enable_alarm(true)?;
    timer0.enable(true)?;

    loop {
        lcd.clear();

        let cnt;
        unsafe {
            cnt = TIMER_COUNTER;
        }

        let seconds = cnt / 1000;
        let millis = cnt % 1000;

        let text = format!("Time: {}.{}", seconds, millis);

        lcd.print(&text);
    }
}
