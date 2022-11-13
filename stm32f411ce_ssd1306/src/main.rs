#![no_main]
#![no_std]

use panic_halt as _;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306, mode::BufferedGraphicsMode, size::DisplaySize128x64};
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
use hal::{i2c::I2c, pac};
use hal::prelude::*;
use heapless::String;

mod images;

// https://stackoverflow.com/questions/58075821/rust-embedded-binary-size
#[inline]
fn my_draw<T: stm32f4xx_hal::i2c::Instance, U>(
    img: usize,
    target: &mut Ssd1306<I2CInterface<I2c<T, U>>,
                            DisplaySize128x64,
                            BufferedGraphicsMode<DisplaySize128x64>
                            >) -> ()
{
    target.clear();
    let raw2: ImageRaw<BinaryColor> =ImageRaw::new(images::IMAGES[img], 128);
    let im = Image::new(&raw2, Point::new(0,0));
    im.draw(target).unwrap();
    target.flush().unwrap();
}

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

    // Configure App Counter

    let mut app_counter = dp.TIM2.counter_ms(&clocks);
    // flip picture every 15 seconds
    let dur: Duration<u32, 1, 1000> = 15000.millis();
    app_counter.start(dur).unwrap();

    let mut im1 = 0;
    let mut im2 = 150;
    my_draw(im1, &mut display2);
    my_draw(im2, &mut display3);
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

            im1 = im1 + 1;
            im2 = im2 + 1;

            if im1 == images::IMAGES.len() {im1 = 0};
            if im2 == images::IMAGES.len() {im2 = 0};

            write!(sums_msg,"flipping {}, {}", im1, im2).unwrap();

            my_draw(im1, &mut display2);
            my_draw(im2, &mut display3);

        }
        else {
            write!(sums_msg,"showing {}, {}", im1, im2).unwrap();
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
