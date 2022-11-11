#![no_main]
#![no_std]

use panic_halt as _;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306, size::DisplaySize128x64};
use stm32f4xx_hal as hal;

use core::{fmt::Write, borrow::BorrowMut};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m::asm;
use fugit::{Duration, ExtU32};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use hal::pac;
use hal::prelude::*;
use hal::timer::Event;
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

    let mut counter1 = dp.TIM1.counter_ms(&clocks);
    let mut counter2 = dp.TIM2.counter_ms(&clocks);
    let mut counter3 = dp.TIM3.counter_us(&clocks);

    let dur: Duration<u32, 1, 1000> = 20000.millis();

    counter1.start(dur).unwrap();

    let dur2: Duration<u32, 1, 1000> = 60.millis();
    let dur3: Duration<u32, 1, 1000000> = 60000.micros();
    counter2.start(dur2).unwrap();
    counter3.start(dur3).unwrap();

    // let x =
    //     match foo {
    //         Ok(_) => "ok",
    //         Err(err) =>
    //             if err == Disabled {
    //                 "Disabled"
    //             } else {
    //                 "WrongAutoReload"
    //             }
    //     };

    // if foo.is_err() {
    //     display.clear();
    //     Text::with_baseline(x, Point::zero(), text_style, Baseline::Top)
    //         .draw(&mut display)
    //         .unwrap();
    //     display.flush().unwrap();
    //     asm::delay(20000000);
    // }

    let mut curr_img = 0;
    let mut curr_img2 = 100;
    let mut curr_img3 = 150;
    loop {

        display.clear();

        // counter 1
        let mut counter1_msg: String<20> = String::from("");
        let count1 = counter1.now().ticks();
        write!(counter1_msg,"{}", count1).unwrap();

        Text::with_baseline(&counter1_msg, Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        // counter 2
        let mut counter2_msg: String<20> = String::from("");
        let count2 = counter2.now().ticks();
        write!(counter2_msg,"{}", count2).unwrap();

        Text::with_baseline(&counter2_msg, Point::new(0, 16), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        // counter 3
        let count3 = counter3.now().ticks();
        let mut counter3_msg: String<20> = String::from("");
        write!(counter3_msg,"{}", count3).unwrap();

        Text::with_baseline(&counter3_msg, Point::new(0, 32), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        // sums
        let mut sums_msg: String<20> = String::from("");
        if counter2.wait().is_ok() {
            write!(sums_msg,"Cool {}", curr_img2).unwrap();
        }
        else {
            write!(sums_msg,"{}", count3 - (1000 * count2)).unwrap();
        }

        Text::with_baseline(&sums_msg, Point::new(0, 48), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        //if curr_img == images::IMAGES.len() {curr_img = 0};
        //let im = images::IMAGES[curr_img];
        //display.clear();
        //let raw: ImageRaw<BinaryColor> =ImageRaw::new(im, 128);
        //let im = Image::new(&raw, Point::new(0, 0));
        //im.draw(&mut display).unwrap();
        //display.flush().unwrap();

        // display2 2
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

        asm::delay(10000000);
        curr_img = curr_img + 1;
        curr_img2 = curr_img2 + 1;
        curr_img3 = curr_img3 + 1;
    }

}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
