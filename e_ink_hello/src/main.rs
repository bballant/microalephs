// This code taken from the example at
// https://github.com/wezm/ssd16750
extern crate linux_embedded_hal;
use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::Delay;
use linux_embedded_hal::{Pin, Spidev};

extern crate ssd1675;
use ssd1675::{Builder, Color, Dimensions, Display, GraphicDisplay, Rotation};

// Graphics
#[macro_use]
extern crate embedded_graphics;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;

// Font
extern crate profont;
use profont::{PROFONT_12_POINT, PROFONT_14_POINT, PROFONT_24_POINT, PROFONT_9_POINT};

use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, io};

// Activate SPI, GPIO in raspi-config needs to be run with sudo because of some sysfs_gpio
// permission problems and follow-up timing problems
// see https://github.com/rust-embedded/rust-sysfs-gpio/issues/5 and follow-up issues

//const ROWS: u16 = 212;
//const COLS: u8 = 104;
const ROWS: u16 = 250;
const COLS: u8 = 120;

// LUT bit from the adafruit python code
// https://github.com/adafruit/Adafruit_CircuitPython_EPD/blob/main/adafruit_epd/ssd1675.py#L46
#[rustfmt::skip]
const LUT: [u8; 70] = [
  0b10000000, 0b1100000, 0b1000000,  0b0,    0b0, 0b0, 0b0
, 0b10000,    0b1100000, 0b100000,   0b0,    0b0, 0b0, 0b0
, 0b10000000, 0b1100000, 0b1000000,  0b0,    0b0, 0b0, 0b0
, 0b10000,    0b1100000, 0b100000,   0b0,    0b0, 0b0, 0b0
, 0b0,  0b0,  0b0,  0b0,  0b0,  0b0,    0b0
, 0b11, 0b11, 0b0,  0b0,  0b10, 0b1001, 0b1001
, 0b0,  0b0,  0b10, 0b11, 0b11, 0b0,    0b0
, 0b10, 0b0,  0b0,  0b0,  0b0,  0b0,    0b0
, 0b0,  0b0,  0b0,  0b0,  0b0,  0b0,    0b0
, 0b0,  0b0,  0b0,  0b0,  0b0,  0b0,    0b0
];

fn main() -> Result<(), std::io::Error> {
    // Configure SPI
    let mut spi = Spidev::open("/dev/spidev0.0").expect("SPI device");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(4_000_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("SPI configuration");

    // https://pinout.xyz/pinout/inky_phat
    // Configure Digital I/O Pins

    let busy = Pin::new(17); // BCM17
    busy.export().expect("busy export");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction");

    let dc = Pin::new(22); // BCM22
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");

    let reset = Pin::new(27); // BCM27
    reset.export().expect("reset export");
    while !reset.is_exported() {}
    reset
        .set_direction(Direction::Out)
        .expect("reset Direction");
    reset.set_value(1).expect("reset Value set to 1");

    let cs = Pin::new(0); // BCM8
    cs.export().expect("cs export");
    while !cs.is_exported() {}
    cs.set_direction(Direction::Out).expect("CS Direction");
    cs.set_value(1).expect("CS Value set to 1");

    println!("Pins configured");

    // Initialise display controller
    let mut delay = Delay {};

    let controller = ssd1675::Interface::new(spi, cs, busy, dc, reset);

    let mut black_buffer = [0u8; ROWS as usize * COLS as usize / 8];
    let mut red_buffer = [0u8; ROWS as usize * COLS as usize / 8];
    let config = Builder::new()
        .dimensions(Dimensions {
            rows: ROWS,
            cols: COLS,
        })
        .rotation(Rotation::Rotate270)
        .lut(&LUT)
        .build()
        .expect("invalid configuration");
    let display = Display::new(controller, config);
    let mut display = GraphicDisplay::new(display, &mut black_buffer, &mut red_buffer);

    // Main loop. Displays CPU temperature, uname, and uptime every minute with a red Raspberry Pi
    // header.
    loop {
        display.reset(&mut delay).expect("error resetting display");
        println!("Reset and initialised");
        let one_minute = Duration::from_secs(60);

        display.clear(Color::White);
        println!("Clear");

        Text::new(
            "Raspberry Pi",
            Point::new(1, 4),
            MonoTextStyle::new(&PROFONT_24_POINT, Color::Black),
        )
        .draw(&mut display)
        .expect("error drawing text");

        if let Ok(cpu_temp) = read_cpu_temp() {
            Text::new(
                "CPU Temp:",
                Point::new(1, 30),
                MonoTextStyle::new(&PROFONT_14_POINT, Color::Black),
            )
            .draw(&mut display)
            .expect("error drawing text");
            Text::new(
                &format!("{:.1}°C", cpu_temp),
                Point::new(95, 34),
                MonoTextStyle::new(&PROFONT_12_POINT, Color::Black),
            )
            .draw(&mut display)
            .expect("error drawing text");
        }

        if let Some(uptime) = read_uptime() {
            Text::new(
                uptime.trim(),
                Point::new(1, 93),
                MonoTextStyle::new(&PROFONT_9_POINT, Color::Black),
            )
            .draw(&mut display)
            .expect("error drawing text");
        }

        if let Some(uname) = read_uname() {
            Text::new(
                uname.trim(),
                Point::new(1, 84),
                MonoTextStyle::new(&PROFONT_9_POINT, Color::Black),
            )
            .draw(&mut display)
            .expect("error drawing text");
        }

        display.update(&mut delay).expect("error updating display");
        println!("Update...");

        println!("Finished - going to sleep");
        display.deep_sleep()?;

        sleep(one_minute);
    }
}

fn read_cpu_temp() -> Result<f64, io::Error> {
    fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?
        .trim()
        .parse::<i32>()
        .map(|temp| temp as f64 / 1000.)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

fn read_uptime() -> Option<String> {
    Command::new("uptime")
        .arg("-p")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
}

fn read_uname() -> Option<String> {
    Command::new("uname")
        .arg("-smr")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
}
