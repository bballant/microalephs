#![no_main]
#![no_std]

use panic_halt as _;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306, size::DisplaySize128x64};
use stm32f4xx_hal as hal;

use core::{fmt::Write};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use fugit::{Duration, ExtU32};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use hal::pac;
use hal::rcc;
use hal::prelude::*;
use heapless::String;

mod images;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpiob = dp.GPIOB.split();
    let gpioa = dp.GPIOA.split();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();


    // Configure three displays on I2C //

    // Configure I2C1
    let scl =
        gpiob
            .pb8
            .into_alternate_open_drain();

    let sda =
        gpiob
            .pb9
            .into_alternate_open_drain();

    let i2c = hal::i2c::I2c::new(
        dp.I2C1,
        (scl, sda),
        hal::i2c::Mode::standard(100.kHz()),
        &clocks
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    // Configure I2C2
    let scl2 =
        gpiob
            .pb10
            .into_alternate_open_drain();

    let sda2 =
        gpiob
            .pb3
            .into_alternate_open_drain();

    let i2c2 = hal::i2c::I2c::new(
        dp.I2C2,
        (scl2, sda2),
        hal::i2c::Mode::standard(100.kHz()),
        &clocks
    );

    let interface2 = I2CDisplayInterface::new(i2c2);
    let mut display2 = Ssd1306::new(interface2, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display2.init().unwrap();

    // Configure I2C3
    let scl3 =
        gpioa
            .pa8
            .into_alternate_open_drain();

    let sda3 =
        gpiob
            .pb4
            .into_alternate_open_drain();

    let i2c3 = hal::i2c::I2c::new(
        dp.I2C3,
        (scl3, sda3),
        hal::i2c::Mode::standard(100.kHz()),
        &clocks
    );

    let interface3 = I2CDisplayInterface::new(i2c3);
    let mut display3 = Ssd1306::new(interface3, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display3.init().unwrap();

    fn rando_g(tim3: pac::TIM3, tim4: pac::TIM4, clocks: rcc::Clocks) -> u32 {
        let mut counter_a = tim3.counter_ms(&clocks);
        let mut counter_b = tim4.counter_us(&clocks);
        counter_a.start(20000.millis()).unwrap();
        counter_b.start(6501.micros()).unwrap();
        nb::block!(counter_a.wait()).unwrap();
        let ret_val = counter_b.now().ticks();
        counter_a.cancel().unwrap();
        counter_b.cancel().unwrap();
        ret_val
    }

    let rand_n = rando_g(dp.TIM3, dp.TIM4, clocks);
    //let rand_n = 0;

    // Configure App Counter

    let mut app_counter = dp.TIM2.counter_ms(&clocks);
    // flip picture every 15 seconds
    let dur: Duration<u32, 1, 1000> = 15000.millis();
    app_counter.start(dur).unwrap();

    let mut curr_img2 = 0;
    let mut curr_img3 = 150;
    loop {

        display.clear();

        // counter 1
        let mut app_counter_msg: String<20> = String::from("");
        let app_ticks = app_counter.now().ticks();
        write!(app_counter_msg,"{}", app_ticks).unwrap();

        Text::with_baseline(&app_counter_msg, Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        let mut sums_msg: String<20> = String::from("");
        if app_counter.wait().is_ok() {

            write!(sums_msg,"Cool {}... ok {}", curr_img2, rand_n).unwrap();

            // display2
            if curr_img2 == images::IMAGES.len() {curr_img2 = 0};
            let im2 = images::IMAGES[curr_img2];
            display2.clear();
            let raw2: ImageRaw<BinaryColor> =ImageRaw::new(im2, 128);
            let im2 = Image::new(&raw2, Point::new(0, 0));
            im2.draw(&mut display2).unwrap();
            display2.flush().unwrap();

            // display 3
            if curr_img3 == images::IMAGES.len() {curr_img3 = 0};
            let im3 = images::IMAGES[curr_img3];
            display3.clear();
            let raw3: ImageRaw<BinaryColor> =ImageRaw::new(im3, 128);
            let im3 = Image::new(&raw3, Point::new(0, 0));
            im3.draw(&mut display3).unwrap();
            display3.flush().unwrap();

            curr_img2 = curr_img2 + 1;
            curr_img3 = curr_img3 + 1;
        }
        else {
            write!(sums_msg,"Just_ {}... ok {}", curr_img2, rand_n).unwrap();
        }

        Text::with_baseline(&sums_msg, Point::new(0, 48), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();
    }

}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
