#![no_main]
#![no_std]

extern crate embedded_sdmmc;
extern crate heapless;

use embedded_sdmmc::{
    TimeSource, Timestamp, VolumeIdx, Mode
};

use core::convert::TryInto;
use core::fmt::Write;
use heapless::String;

use panic_halt as _;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306, size::DisplaySize128x64};
use stm32f3xx_hal as hal;

use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m::asm;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use hal::pac;
use hal::spi::Spi;
use hal::prelude::*;


struct Clock;

impl TimeSource for Clock {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);


    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    // Configure I2C1
    let mut scl =
        gpiob
            .pb6
            .into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let mut sda =
        gpiob
            .pb7
            .into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

    scl.internal_pull_up(&mut gpiob.pupdr, true);
    sda.internal_pull_up(&mut gpiob.pupdr, true);

    let i2c = hal::i2c::I2c::new(
        dp.I2C1,
        (scl, sda),
        100.kHz().try_into().unwrap(),
        clocks,
        &mut rcc.apb1,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    let imgs = [
        include_bytes!("1.gray"),
        include_bytes!("2.gray"),
        include_bytes!("3.gray"),
        include_bytes!("4.gray"),
        include_bytes!("5.gray"),
        include_bytes!("6.gray"),
        include_bytes!("7.gray"),
        include_bytes!("8.gray"),
        include_bytes!("9.gray"),
        include_bytes!("10.gray"),
        include_bytes!("11.gray"),
        include_bytes!("12.gray"),
        include_bytes!("13.gray"),
        include_bytes!("14.gray"),
        include_bytes!("15.gray"),
        include_bytes!("16.gray"),
        include_bytes!("17.gray"),
        include_bytes!("18.gray"),
        include_bytes!("19.gray"),
        include_bytes!("20.gray"),
        include_bytes!("21.gray"),
        include_bytes!("22.gray"),
        include_bytes!("23.gray"),
        include_bytes!("24.gray"),
        include_bytes!("25.gray")
    ];

    //let text_style = MonoTextStyleBuilder::new()
    //    .font(&FONT_6X10)
    //    .text_color(BinaryColor::On)
    //    .build();

    //Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
    //    .draw(&mut display)
    //    .unwrap();

    //let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

    //// Configure pins for SPI
    //let sck = gpioc
    //    .pc10
    //    .into_af_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrh);
    //let miso = gpioc
    //    .pc11
    //    .into_af_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrh);
    //let mosi = gpioc
    //    .pc12
    //    .into_af_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrh);


    //let spi = Spi::new(dp.SPI3, (sck, miso, mosi), 3.MHz(), clocks, &mut rcc.apb1);

    //let chip_select = gpioc
    //    .pc4
    //    .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

    //let mut cont = embedded_sdmmc::Controller::new(
    //    embedded_sdmmc::SdMmcSpi::new(spi, chip_select),
    //    Clock
    //);

    //match cont.device().init() {
    //    Ok(_) => {
    //        let volume = cont.get_volume(VolumeIdx(0));

    //        if let Ok(mut volume) = volume {
    //            let root_dir = cont.open_root_dir(&volume).unwrap();
    //            let mut f = cont
    //                .open_file_in_dir(&mut volume, &root_dir, "borges.txt", Mode::ReadOnly)
    //                .unwrap();

    //            let mut s: String<32> = String::from("");
    //            let mut buffer = [0u8; 32];
    //            let num_read = cont.read(&volume, &mut f, &mut buffer).unwrap();
    //            for b in &buffer[0..num_read] {
    //                if *b == 10 {
    //                    break;
    //                }
    //                write!(s, "{}", *b as char).unwrap();
    //            }

    //            Text::with_baseline(s.as_mut_str(), Point::new(0, 36), text_style, Baseline::Top)
    //                .draw(&mut display)
    //                .unwrap();
    //        } else {
    //            Text::with_baseline("NUTS", Point::new(0, 36), text_style, Baseline::Top)
    //                .draw(&mut display)
    //                .unwrap();
    //        }
    //    }
    //    Err(_e) => {
    //        Text::with_baseline("Ouch", Point::new(0, 56), text_style, Baseline::Top)
    //        .draw(&mut display)
    //        .unwrap();
    //    }
    //};


    let mut curr_img = 0;
    loop {
        if curr_img == imgs.len() {curr_img = 0};
        let im = imgs[curr_img];
        display.clear();
        let raw: ImageRaw<BinaryColor> =ImageRaw::new(im, 128);
        let im = Image::new(&raw, Point::new(0, 0));
        im.draw(&mut display).unwrap();
        display.flush().unwrap();
        asm::delay(50000000);
        curr_img = curr_img + 1;
    }

}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
