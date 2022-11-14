#![no_main]
#![no_std]

use panic_halt as _;

use stm32f4xx_hal as hal;

use core::fmt::Write;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::Rgb565,
    prelude::*,
};
use fugit::{Duration, ExtU32};
use hal::{pac, prelude::*};
use heapless::String;
use hal::spi::{Mode, Phase, Polarity, Spi, NoMiso};
use st7735_lcd::Orientation;


mod images;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpioa = dp.GPIOA.split();


    // SPI1
    let sck = gpioa.pa5.into_alternate();
    let mosi = gpioa.pa7.into_alternate().internal_pull_up(true);

    let rst = gpioa.pa1.into_push_pull_output();
    let dc = gpioa.pa0.into_push_pull_output();

    let spi = Spi::new(
        dp.SPI1,
        (sck, NoMiso {}, mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        16.MHz().into(),
        &clocks,
    );

    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 160, 128);

    let mut delay = cp.SYST.delay(&clocks);
    disp.init(&mut delay).unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();
    //disp.clear(Rgb565::BLACK);
    disp.clear(Rgb565::RED).unwrap();
    disp.set_offset(0, 25);

    // draw ferris
    let image_raw: ImageRaw<Rgb565> =
        ImageRaw::new(include_bytes!("../../images/ferris.raw"), 86);
    let image = Image::new(&image_raw, Point::new(34, 8));

    image.draw(&mut disp).unwrap();
    // Configure App Counter

    let mut app_counter = dp.TIM2.counter_ms(&clocks);
    // flip picture every 15 seconds
    let dur: Duration<u32, 1, 1000> = 15000.millis();
    app_counter.start(dur).unwrap();

    let mut im1 = 0;
    let mut im2 = 150;
    loop {

        let mut sums_msg: String<20> = String::from("");
        if app_counter.wait().is_ok() {
            im1 = im1 + 1;
            im2 = im2 + 1;

            if im1 == images::IMAGES.len() {
                im1 = 0
            };
            if im2 == images::IMAGES.len() {
                im2 = 0
            };

            write!(sums_msg, "flipping {}, {}", im1, im2).unwrap();

        } else {
            write!(sums_msg, "showing {}, {}", im1, im2).unwrap();
        }

    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
