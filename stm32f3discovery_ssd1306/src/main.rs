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
        include_bytes!("../../images/128x64/001.gray"),
        include_bytes!("../../images/128x64/002.gray"),
        include_bytes!("../../images/128x64/003.gray"),
        include_bytes!("../../images/128x64/004.gray"),
        include_bytes!("../../images/128x64/005.gray"),
        include_bytes!("../../images/128x64/006.gray"),
        include_bytes!("../../images/128x64/007.gray"),
        include_bytes!("../../images/128x64/008.gray"),
        include_bytes!("../../images/128x64/009.gray"),
        include_bytes!("../../images/128x64/010.gray"),
        include_bytes!("../../images/128x64/011.gray"),
        include_bytes!("../../images/128x64/012.gray"),
        include_bytes!("../../images/128x64/013.gray"),
        include_bytes!("../../images/128x64/014.gray"),
        include_bytes!("../../images/128x64/015.gray"),
        include_bytes!("../../images/128x64/016.gray"),
        include_bytes!("../../images/128x64/017.gray"),
        include_bytes!("../../images/128x64/018.gray"),
        include_bytes!("../../images/128x64/019.gray"),
        include_bytes!("../../images/128x64/020.gray"),
        include_bytes!("../../images/128x64/021.gray"),
        include_bytes!("../../images/128x64/022.gray"),
        include_bytes!("../../images/128x64/023.gray"),
        include_bytes!("../../images/128x64/024.gray"),
        include_bytes!("../../images/128x64/025.gray"),
        include_bytes!("../../images/128x64/026.gray"),
        include_bytes!("../../images/128x64/027.gray"),
        include_bytes!("../../images/128x64/028.gray"),
        include_bytes!("../../images/128x64/029.gray"),
        include_bytes!("../../images/128x64/030.gray"),
        include_bytes!("../../images/128x64/031.gray"),
        include_bytes!("../../images/128x64/032.gray"),
        include_bytes!("../../images/128x64/033.gray"),
        include_bytes!("../../images/128x64/034.gray"),
        include_bytes!("../../images/128x64/035.gray"),
        include_bytes!("../../images/128x64/036.gray"),
        include_bytes!("../../images/128x64/037.gray"),
        include_bytes!("../../images/128x64/038.gray"),
        include_bytes!("../../images/128x64/039.gray"),
        include_bytes!("../../images/128x64/040.gray"),
        include_bytes!("../../images/128x64/041.gray"),
        include_bytes!("../../images/128x64/042.gray"),
        include_bytes!("../../images/128x64/043.gray"),
        include_bytes!("../../images/128x64/044.gray"),
        include_bytes!("../../images/128x64/045.gray"),
        include_bytes!("../../images/128x64/046.gray"),
        include_bytes!("../../images/128x64/047.gray"),
        include_bytes!("../../images/128x64/048.gray"),
        include_bytes!("../../images/128x64/049.gray"),
        include_bytes!("../../images/128x64/050.gray"),
        include_bytes!("../../images/128x64/051.gray"),
        include_bytes!("../../images/128x64/052.gray"),
        include_bytes!("../../images/128x64/053.gray"),
        include_bytes!("../../images/128x64/054.gray"),
        include_bytes!("../../images/128x64/055.gray"),
        include_bytes!("../../images/128x64/056.gray"),
        include_bytes!("../../images/128x64/057.gray"),
        include_bytes!("../../images/128x64/058.gray"),
        include_bytes!("../../images/128x64/059.gray"),
        include_bytes!("../../images/128x64/060.gray"),
        include_bytes!("../../images/128x64/061.gray"),
        include_bytes!("../../images/128x64/062.gray"),
        include_bytes!("../../images/128x64/063.gray"),
        include_bytes!("../../images/128x64/064.gray"),
        include_bytes!("../../images/128x64/065.gray"),
        include_bytes!("../../images/128x64/066.gray"),
        include_bytes!("../../images/128x64/067.gray"),
        include_bytes!("../../images/128x64/068.gray"),
        include_bytes!("../../images/128x64/069.gray"),
        include_bytes!("../../images/128x64/070.gray"),
        include_bytes!("../../images/128x64/071.gray"),
        include_bytes!("../../images/128x64/072.gray"),
        include_bytes!("../../images/128x64/073.gray"),
        include_bytes!("../../images/128x64/074.gray"),
        include_bytes!("../../images/128x64/075.gray"),
        include_bytes!("../../images/128x64/076.gray"),
        include_bytes!("../../images/128x64/077.gray"),
        include_bytes!("../../images/128x64/078.gray"),
        include_bytes!("../../images/128x64/079.gray"),
        include_bytes!("../../images/128x64/080.gray"),
        include_bytes!("../../images/128x64/081.gray"),
        include_bytes!("../../images/128x64/082.gray"),
        include_bytes!("../../images/128x64/083.gray"),
        include_bytes!("../../images/128x64/084.gray"),
        include_bytes!("../../images/128x64/085.gray"),
        include_bytes!("../../images/128x64/086.gray"),
        include_bytes!("../../images/128x64/087.gray"),
        include_bytes!("../../images/128x64/088.gray"),
        include_bytes!("../../images/128x64/089.gray"),
        include_bytes!("../../images/128x64/090.gray"),
        include_bytes!("../../images/128x64/091.gray"),
        include_bytes!("../../images/128x64/092.gray"),
        include_bytes!("../../images/128x64/093.gray"),
        include_bytes!("../../images/128x64/094.gray"),
        include_bytes!("../../images/128x64/095.gray"),
        include_bytes!("../../images/128x64/096.gray"),
        include_bytes!("../../images/128x64/097.gray"),
        include_bytes!("../../images/128x64/098.gray"),
        include_bytes!("../../images/128x64/099.gray"),
        include_bytes!("../../images/128x64/100.gray"),
        include_bytes!("../../images/128x64/101.gray"),
        include_bytes!("../../images/128x64/102.gray"),
        include_bytes!("../../images/128x64/103.gray"),
        include_bytes!("../../images/128x64/104.gray"),
        include_bytes!("../../images/128x64/105.gray"),
        include_bytes!("../../images/128x64/106.gray"),
        include_bytes!("../../images/128x64/107.gray"),
        include_bytes!("../../images/128x64/108.gray"),
        include_bytes!("../../images/128x64/109.gray"),
        include_bytes!("../../images/128x64/100.gray"),
        include_bytes!("../../images/128x64/111.gray"),
        include_bytes!("../../images/128x64/112.gray"),
        include_bytes!("../../images/128x64/113.gray"),
        include_bytes!("../../images/128x64/114.gray"),
        include_bytes!("../../images/128x64/115.gray"),
        include_bytes!("../../images/128x64/116.gray"),
        include_bytes!("../../images/128x64/117.gray"),
        include_bytes!("../../images/128x64/118.gray"),
        include_bytes!("../../images/128x64/119.gray"),
        include_bytes!("../../images/128x64/120.gray"),
        include_bytes!("../../images/128x64/121.gray"),
        include_bytes!("../../images/128x64/122.gray"),
        include_bytes!("../../images/128x64/123.gray"),
        include_bytes!("../../images/128x64/124.gray"),
        include_bytes!("../../images/128x64/125.gray"),
        include_bytes!("../../images/128x64/126.gray"),
        include_bytes!("../../images/128x64/127.gray"),
        include_bytes!("../../images/128x64/128.gray"),
        include_bytes!("../../images/128x64/129.gray"),
        include_bytes!("../../images/128x64/130.gray"),
        include_bytes!("../../images/128x64/131.gray"),
        include_bytes!("../../images/128x64/132.gray"),
        include_bytes!("../../images/128x64/133.gray"),
        include_bytes!("../../images/128x64/134.gray"),
        include_bytes!("../../images/128x64/135.gray"),
        include_bytes!("../../images/128x64/136.gray"),
        include_bytes!("../../images/128x64/137.gray"),
        include_bytes!("../../images/128x64/138.gray"),
        include_bytes!("../../images/128x64/139.gray"),
        include_bytes!("../../images/128x64/140.gray"),
        include_bytes!("../../images/128x64/141.gray"),
        include_bytes!("../../images/128x64/142.gray"),
        include_bytes!("../../images/128x64/143.gray"),
        include_bytes!("../../images/128x64/144.gray"),
        include_bytes!("../../images/128x64/145.gray"),
        include_bytes!("../../images/128x64/146.gray"),
        include_bytes!("../../images/128x64/147.gray"),
        include_bytes!("../../images/128x64/148.gray"),
        include_bytes!("../../images/128x64/149.gray"),
        include_bytes!("../../images/128x64/150.gray"),
        include_bytes!("../../images/128x64/151.gray"),
        include_bytes!("../../images/128x64/152.gray"),
        include_bytes!("../../images/128x64/153.gray"),
        include_bytes!("../../images/128x64/154.gray"),
        include_bytes!("../../images/128x64/155.gray"),
        include_bytes!("../../images/128x64/156.gray"),
        include_bytes!("../../images/128x64/157.gray"),
        include_bytes!("../../images/128x64/158.gray"),
        include_bytes!("../../images/128x64/159.gray"),
        include_bytes!("../../images/128x64/150.gray"),
        include_bytes!("../../images/128x64/161.gray"),
        include_bytes!("../../images/128x64/162.gray"),
        include_bytes!("../../images/128x64/163.gray"),
        include_bytes!("../../images/128x64/164.gray"),
        include_bytes!("../../images/128x64/165.gray"),
        include_bytes!("../../images/128x64/166.gray"),
        include_bytes!("../../images/128x64/167.gray"),
        include_bytes!("../../images/128x64/168.gray"),
        include_bytes!("../../images/128x64/169.gray"),
        include_bytes!("../../images/128x64/170.gray"),
        include_bytes!("../../images/128x64/171.gray"),
        include_bytes!("../../images/128x64/172.gray"),
        include_bytes!("../../images/128x64/173.gray"),
        include_bytes!("../../images/128x64/174.gray"),
        include_bytes!("../../images/128x64/175.gray"),
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
