#![no_main]
#![no_std]

use panic_halt as _;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306, size::DisplaySize128x64};
use stm32f4xx_hal as hal;

use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m::asm;
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};
use hal::pac;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    // LED
    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_low();

    let gpiob = dp.GPIOB.split();

    // Configure I2C1
    let scl =
        gpiob
            .pb6
            .into_alternate_open_drain();
    let sda =
        gpiob
            .pb7
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

    let imgs = [
        include_bytes!("../../images/64x128r90/001.gray"),
        include_bytes!("../../images/64x128r90/002.gray"),
        include_bytes!("../../images/64x128r90/003.gray"),
        include_bytes!("../../images/64x128r90/004.gray"),
        include_bytes!("../../images/64x128r90/005.gray"),
        include_bytes!("../../images/64x128r90/006.gray"),
        include_bytes!("../../images/64x128r90/007.gray"),
        include_bytes!("../../images/64x128r90/008.gray"),
        include_bytes!("../../images/64x128r90/009.gray"),
        include_bytes!("../../images/64x128r90/010.gray"),
        include_bytes!("../../images/64x128r90/011.gray"),
        include_bytes!("../../images/64x128r90/012.gray"),
        include_bytes!("../../images/64x128r90/013.gray"),
        include_bytes!("../../images/64x128r90/014.gray"),
        include_bytes!("../../images/64x128r90/015.gray"),
        include_bytes!("../../images/64x128r90/016.gray"),
        include_bytes!("../../images/64x128r90/017.gray"),
        include_bytes!("../../images/64x128r90/018.gray"),
        include_bytes!("../../images/64x128r90/019.gray"),
        include_bytes!("../../images/64x128r90/020.gray"),
        include_bytes!("../../images/64x128r90/021.gray"),
        include_bytes!("../../images/64x128r90/022.gray"),
        include_bytes!("../../images/64x128r90/023.gray"),
        include_bytes!("../../images/64x128r90/024.gray"),
        include_bytes!("../../images/64x128r90/025.gray"),
        include_bytes!("../../images/64x128r90/026.gray"),
        include_bytes!("../../images/64x128r90/027.gray"),
        include_bytes!("../../images/64x128r90/028.gray"),
        include_bytes!("../../images/64x128r90/029.gray"),
        include_bytes!("../../images/64x128r90/030.gray"),
        include_bytes!("../../images/64x128r90/031.gray"),
        include_bytes!("../../images/64x128r90/032.gray"),
        include_bytes!("../../images/64x128r90/033.gray"),
        include_bytes!("../../images/64x128r90/034.gray"),
        include_bytes!("../../images/64x128r90/035.gray"),
        include_bytes!("../../images/64x128r90/036.gray"),
        include_bytes!("../../images/64x128r90/037.gray"),
        include_bytes!("../../images/64x128r90/038.gray"),
        include_bytes!("../../images/64x128r90/039.gray"),
        include_bytes!("../../images/64x128r90/040.gray"),
        include_bytes!("../../images/64x128r90/041.gray"),
        include_bytes!("../../images/64x128r90/042.gray"),
        include_bytes!("../../images/64x128r90/043.gray"),
        include_bytes!("../../images/64x128r90/044.gray"),
        include_bytes!("../../images/64x128r90/045.gray"),
        include_bytes!("../../images/64x128r90/046.gray"),
        include_bytes!("../../images/64x128r90/047.gray"),
        include_bytes!("../../images/64x128r90/048.gray"),
        include_bytes!("../../images/64x128r90/049.gray"),
        include_bytes!("../../images/64x128r90/050.gray"),
        include_bytes!("../../images/64x128r90/051.gray"),
        include_bytes!("../../images/64x128r90/052.gray"),
        include_bytes!("../../images/64x128r90/053.gray"),
        include_bytes!("../../images/64x128r90/054.gray"),
        include_bytes!("../../images/64x128r90/055.gray"),
        include_bytes!("../../images/64x128r90/056.gray"),
        include_bytes!("../../images/64x128r90/057.gray"),
        include_bytes!("../../images/64x128r90/058.gray"),
        include_bytes!("../../images/64x128r90/059.gray"),
        include_bytes!("../../images/64x128r90/060.gray"),
        include_bytes!("../../images/64x128r90/061.gray"),
        include_bytes!("../../images/64x128r90/062.gray"),
        include_bytes!("../../images/64x128r90/063.gray"),
        include_bytes!("../../images/64x128r90/064.gray"),
        include_bytes!("../../images/64x128r90/065.gray"),
        include_bytes!("../../images/64x128r90/066.gray"),
        include_bytes!("../../images/64x128r90/067.gray"),
        include_bytes!("../../images/64x128r90/068.gray"),
        include_bytes!("../../images/64x128r90/069.gray"),
        include_bytes!("../../images/64x128r90/070.gray"),
        include_bytes!("../../images/64x128r90/071.gray"),
        include_bytes!("../../images/64x128r90/072.gray"),
        include_bytes!("../../images/64x128r90/073.gray"),
        include_bytes!("../../images/64x128r90/074.gray"),
        include_bytes!("../../images/64x128r90/075.gray"),
        include_bytes!("../../images/64x128r90/076.gray"),
        include_bytes!("../../images/64x128r90/077.gray"),
        include_bytes!("../../images/64x128r90/078.gray"),
        include_bytes!("../../images/64x128r90/079.gray"),
        include_bytes!("../../images/64x128r90/080.gray"),
        include_bytes!("../../images/64x128r90/081.gray"),
        include_bytes!("../../images/64x128r90/082.gray"),
        include_bytes!("../../images/64x128r90/083.gray"),
        include_bytes!("../../images/64x128r90/084.gray"),
        include_bytes!("../../images/64x128r90/085.gray"),
        include_bytes!("../../images/64x128r90/086.gray"),
        include_bytes!("../../images/64x128r90/087.gray"),
        include_bytes!("../../images/64x128r90/088.gray"),
        include_bytes!("../../images/64x128r90/089.gray"),
        include_bytes!("../../images/64x128r90/090.gray"),
        include_bytes!("../../images/64x128r90/091.gray"),
        include_bytes!("../../images/64x128r90/092.gray"),
        include_bytes!("../../images/64x128r90/093.gray"),
        include_bytes!("../../images/64x128r90/094.gray"),
        include_bytes!("../../images/64x128r90/095.gray"),
        include_bytes!("../../images/64x128r90/096.gray"),
        include_bytes!("../../images/64x128r90/097.gray"),
        include_bytes!("../../images/64x128r90/098.gray"),
        include_bytes!("../../images/64x128r90/099.gray"),
        include_bytes!("../../images/64x128r90/100.gray"),
        include_bytes!("../../images/64x128r90/101.gray"),
        include_bytes!("../../images/64x128r90/102.gray"),
        include_bytes!("../../images/64x128r90/103.gray"),
        include_bytes!("../../images/64x128r90/104.gray"),
        include_bytes!("../../images/64x128r90/105.gray"),
        include_bytes!("../../images/64x128r90/106.gray"),
        include_bytes!("../../images/64x128r90/107.gray"),
        include_bytes!("../../images/64x128r90/108.gray"),
        include_bytes!("../../images/64x128r90/109.gray"),
        include_bytes!("../../images/64x128r90/100.gray"),
        include_bytes!("../../images/64x128r90/111.gray"),
        include_bytes!("../../images/64x128r90/112.gray"),
        include_bytes!("../../images/64x128r90/113.gray"),
        include_bytes!("../../images/64x128r90/114.gray"),
        include_bytes!("../../images/64x128r90/115.gray"),
        include_bytes!("../../images/64x128r90/116.gray"),
        include_bytes!("../../images/64x128r90/117.gray"),
        include_bytes!("../../images/64x128r90/118.gray"),
        include_bytes!("../../images/64x128r90/119.gray"),
        include_bytes!("../../images/64x128r90/120.gray"),
        include_bytes!("../../images/64x128r90/121.gray"),
        include_bytes!("../../images/64x128r90/122.gray"),
        include_bytes!("../../images/64x128r90/123.gray"),
        include_bytes!("../../images/64x128r90/124.gray"),
        include_bytes!("../../images/64x128r90/125.gray"),
        include_bytes!("../../images/64x128r90/126.gray"),
        include_bytes!("../../images/64x128r90/127.gray"),
        include_bytes!("../../images/64x128r90/128.gray"),
        include_bytes!("../../images/64x128r90/129.gray"),
        include_bytes!("../../images/64x128r90/130.gray"),
        include_bytes!("../../images/64x128r90/131.gray"),
        include_bytes!("../../images/64x128r90/132.gray"),
        include_bytes!("../../images/64x128r90/133.gray"),
        include_bytes!("../../images/64x128r90/134.gray"),
        include_bytes!("../../images/64x128r90/135.gray"),
        include_bytes!("../../images/64x128r90/136.gray"),
        include_bytes!("../../images/64x128r90/137.gray"),
        include_bytes!("../../images/64x128r90/138.gray"),
        include_bytes!("../../images/64x128r90/139.gray"),
        include_bytes!("../../images/64x128r90/140.gray"),
        include_bytes!("../../images/64x128r90/141.gray"),
        include_bytes!("../../images/64x128r90/142.gray"),
        include_bytes!("../../images/64x128r90/143.gray"),
        include_bytes!("../../images/64x128r90/144.gray"),
        include_bytes!("../../images/64x128r90/145.gray"),
        include_bytes!("../../images/64x128r90/146.gray"),
        include_bytes!("../../images/64x128r90/147.gray"),
        include_bytes!("../../images/64x128r90/148.gray"),
        include_bytes!("../../images/64x128r90/149.gray"),
        include_bytes!("../../images/64x128r90/150.gray"),
        include_bytes!("../../images/64x128r90/151.gray"),
        include_bytes!("../../images/64x128r90/152.gray"),
        include_bytes!("../../images/64x128r90/153.gray"),
        include_bytes!("../../images/64x128r90/154.gray"),
        include_bytes!("../../images/64x128r90/155.gray"),
        include_bytes!("../../images/64x128r90/156.gray"),
        include_bytes!("../../images/64x128r90/157.gray"),
        include_bytes!("../../images/64x128r90/158.gray"),
        include_bytes!("../../images/64x128r90/159.gray"),
        include_bytes!("../../images/64x128r90/150.gray"),
        include_bytes!("../../images/64x128r90/161.gray"),
        include_bytes!("../../images/64x128r90/162.gray"),
        include_bytes!("../../images/64x128r90/163.gray"),
        include_bytes!("../../images/64x128r90/164.gray"),
        include_bytes!("../../images/64x128r90/165.gray"),
        include_bytes!("../../images/64x128r90/166.gray"),
        include_bytes!("../../images/64x128r90/167.gray"),
        include_bytes!("../../images/64x128r90/168.gray"),
        include_bytes!("../../images/64x128r90/169.gray"),
        include_bytes!("../../images/64x128r90/170.gray"),
        include_bytes!("../../images/64x128r90/171.gray"),
        include_bytes!("../../images/64x128r90/172.gray"),
        include_bytes!("../../images/64x128r90/173.gray"),
    ];

    let mut curr_img = 0;
    loop {
        if curr_img == imgs.len() {curr_img = 0};
        let im = imgs[curr_img];
        // display.clear();
        // let raw: ImageRaw<BinaryColor> =ImageRaw::new(im, 128);
        // let im = Image::new(&raw, Point::new(0, 0));
        // im.draw(&mut display).unwrap();
        // display.flush().unwrap();
        asm::delay(2_1111_111);
        curr_img = curr_img + 1;
        led.toggle();
    }

}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
