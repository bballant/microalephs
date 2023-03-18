#![no_main]
#![no_std]

use panic_halt as _;

use ssd1306::{
    mode::BufferedGraphicsMode, prelude::*,
    size::DisplaySize128x64,
    I2CDisplayInterface, Ssd1306,
};
use stm32f4xx_hal as hal;

use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};
use fugit::{Duration, ExtU32};
use hal::{i2c::I2c, pac, prelude::*};

mod images;

// https://stackoverflow.com/questions/58075821/rust-embedded-binary-size
#[inline]
fn my_draw<T: stm32f4xx_hal::i2c::Instance, U>(
    img: usize,
    display: &mut Ssd1306<
        I2CInterface<I2c<T, U>>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
) -> () {
    display.clear();
    let raw2: ImageRaw<BinaryColor> = ImageRaw::new(images::IMAGES[img], 128);
    let im = Image::new(&raw2, Point::new(0, 0));
    im.draw(display).unwrap();
    display.flush().unwrap();
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpiob = dp.GPIOB.split();

    // Configure I2C2
    let scl2 = gpiob.pb10.into_alternate_open_drain();

    let sda2 = gpiob.pb3.into_alternate_open_drain();

    let i2c2 = hal::i2c::I2c::new(
        dp.I2C2,
        (scl2, sda2),
        hal::i2c::Mode::standard(100.kHz()),
        &clocks,
    );

    let interface2 = I2CDisplayInterface::new(i2c2);
    let mut display2 = Ssd1306::new(interface2, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display2.init().unwrap();

    // Configure App Counter

    let mut app_counter = dp.TIM2.counter_ms(&clocks);
    // flip picture every 15 seconds
    let dur: Duration<u32, 1, 1000> = 14000.millis();
    app_counter.start(dur).unwrap();

    let mut app_counter2 = dp.TIM3.counter_ms(&clocks);
    // flip picture every 15 seconds
    let dur2: Duration<u32, 1, 1000> = 13000.millis();
    app_counter2.start(dur2).unwrap();

    let mut im1 = 0;
    let mut im2 = im1+7; // how many things can you remember?
    my_draw(im2, &mut display2);
    loop {

//        if app_counter.wait().is_ok() {
//            im1 = im1 + 1;
//            if im1 == images::IMAGES.len() {
//                im1 = 0
//            };
//            my_draw(im1, &mut display);
//        }

        if app_counter2.wait().is_ok() {
            im2 = im2 + 1;
            if im2 == images::IMAGES.len() {
                im2 = 0
            };
            my_draw(im2, &mut display2);
        }
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
