use ag_lcd::LcdDisplay;
use esp_idf_svc::hal::{delay::Delay, i2c::{I2cConfig, I2cDriver}, prelude::Peripherals,  timer::*};
use port_expander::Pcf8574;
use esp_idf_svc::hal::prelude::*;
use esp32_nimble::{uuid128, BLEAdvertisementData, BLEDevice, NimbleProperties};

static mut TIMER_COUNTER: u64 = 0;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

let ble_device = BLEDevice::take();
  let ble_advertising = ble_device.get_advertising();

  let server = ble_device.get_server();
  server.on_connect(|server, desc| {
    ::log::info!("Client connected: {:?}", desc);

    server
      .update_conn_params(desc.conn_handle(), 24, 48, 0, 60)
      .unwrap();

    if server.connected_count() < (esp_idf_svc::sys::CONFIG_BT_NIMBLE_MAX_CONNECTIONS as _) {
      ::log::info!("Multi-connect support: start advertising");
      ble_advertising.lock().start().unwrap();
    }
  });

  server.on_disconnect(|_desc, reason| {
    ::log::info!("Client disconnected ({:?})", reason);
  });

  let service = server.create_service(uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa"));

  // A static characteristic.
  let static_characteristic = service.lock().create_characteristic(
    uuid128!("d4e0e0d0-1a2b-11e9-ab14-d663bd873d93"),
    NimbleProperties::READ,
  );
  static_characteristic
    .lock()
    .set_value("Hello, world!".as_bytes());

  // A characteristic that notifies every second.
  let notifying_characteristic = service.lock().create_characteristic(
    uuid128!("a3c87500-8ed3-4bdf-8a39-a01bebede295"),
    NimbleProperties::READ | NimbleProperties::NOTIFY,
  );
  notifying_characteristic.lock().set_value(b"Initial value.");

  // A writable characteristic.
  let writable_characteristic = service.lock().create_characteristic(
    uuid128!("3c9a3f00-8ed3-4bdf-8a39-a01bebede295"),
    NimbleProperties::READ | NimbleProperties::WRITE,
  );
  writable_characteristic
    .lock()
    .on_read(move |_, _| {
      ::log::info!("Read from writable characteristic.");
    })
    .on_write(|args| {
      ::log::info!(
        "Wrote to writable characteristic: {:?} -> {:?}",
        args.current_data(),
        args.recv_data()
      );
    });

  ble_advertising.lock().set_data(
    BLEAdvertisementData::new()
      .name("ESP32-GATT-Server")
      .add_service_uuid(uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa")),
  )?;
  ble_advertising.lock().start()?;

  server.ble_gatts_show_local();

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

 notifying_characteristic
      .lock()

      .set_value(&cnt.to_le_bytes())
      .notify();
    }
}
