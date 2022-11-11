#![no_main]
#![no_std]

use panic_halt as _;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306, size::DisplaySize128x64};
use stm32f4xx_hal as hal;

use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m::asm;
use fugit::{Duration, ExtU32};
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};
use hal::pac;
use hal::prelude::*;

const IMAGES: [&[u8; 1024]; 195]  =
    [
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
        include_bytes!("../../images/128x64/176.gray"),
        include_bytes!("../../images/128x64/177.gray"),
        include_bytes!("../../images/128x64/178.gray"),
        include_bytes!("../../images/128x64/179.gray"),
        include_bytes!("../../images/128x64/180.gray"),
        include_bytes!("../../images/128x64/181.gray"),
        include_bytes!("../../images/128x64/182.gray"),
        include_bytes!("../../images/128x64/183.gray"),
        include_bytes!("../../images/128x64/184.gray"),
        include_bytes!("../../images/128x64/185.gray"),
        include_bytes!("../../images/128x64/186.gray"),
        include_bytes!("../../images/128x64/187.gray"),
        include_bytes!("../../images/128x64/188.gray"),
        include_bytes!("../../images/128x64/189.gray"),
        include_bytes!("../../images/128x64/190.gray"),
        include_bytes!("../../images/128x64/191.gray"),
        include_bytes!("../../images/128x64/192.gray"),
        include_bytes!("../../images/128x64/193.gray"),
        include_bytes!("../../images/128x64/194.gray"),
        include_bytes!("../../images/128x64/195.gray"),
        ];

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpiob = dp.GPIOB.split();
    let gpioa = dp.GPIOA.split();

    let mut counter1 = dp.TIM1.counter_ms(&clocks);
    let mut counter2 = dp.TIM2.counter_ms(&clocks);
    let mut counter3 = dp.TIM3.counter_ms(&clocks);

    let mut dur: Duration<u32, 1, 1000> = 2001.millis();

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

    counter1.start(dur).unwrap();
    let mut curr_img = 0;
    let mut curr_img2 = 100;
    let mut curr_img3 = 150;
    loop {

        //if curr_img == IMAGES.len() {curr_img = 0};
        //let im = IMAGES[curr_img];
        //display.clear();
        //let raw: ImageRaw<BinaryColor> =ImageRaw::new(im, 128);
        //let im = Image::new(&raw, Point::new(0, 0));
        //im.draw(&mut display).unwrap();
        //display.flush().unwrap();

        // display2 2
        if curr_img2 == IMAGES.len() {curr_img2 = 0};
        let im2 = IMAGES[curr_img2];
        display2.clear();
        let raw2: ImageRaw<BinaryColor> =ImageRaw::new(im2, 128);
        let im2 = Image::new(&raw2, Point::new(0, 0));
        im2.draw(&mut display2).unwrap();
        display2.flush().unwrap();

        // display 3
        if curr_img3 == IMAGES.len() {curr_img3 = 0};
        let im3 = IMAGES[curr_img3];
        display3.clear();
        let raw3: ImageRaw<BinaryColor> =ImageRaw::new(im3, 128);
        let im3 = Image::new(&raw3, Point::new(0, 0));
        im3.draw(&mut display3).unwrap();
        display3.flush().unwrap();

        asm::delay(100000000);
        curr_img = curr_img + 1;
        curr_img2 = curr_img2 + 1;
        curr_img3 = curr_img3 + 1;
    }

}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
