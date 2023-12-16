//todo:非标准库工程
#![no_std]
//todo:无主函数
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use cortex_m_rt::entry;
//todo:引入stm32的hal库crate,pac指外设访问层
use stm32h7xx_hal::{pac, prelude::*};
#[entry]
fn main() -> ! {
    //todo:使用单一外设使用constrain，使用多种外设如gpioa，gpiob等使用split

    // asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    //todo:获取cortex-M的核心
    let cp = cortex_m::Peripherals::take().unwrap();
    //todo:通过pac层的接口来获取mcu上的外设
    let dp = pac::Peripherals::take().unwrap();

    // //todo:设置power
    let pwr = dp.PWR.constrain(); //为了使用hal库
    let pwrcfg = pwr.freeze(); //保存电源配置

    //todo:设置rcc
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    //todo:获取gpioc外设,blue_led
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    //todo:设置blue_led引脚为推挽输出模式
    let mut blue_led = gpioc.pc1.into_push_pull_output();

    //todo:通过Cortex-m crate提供的一个system tick接口来实现简单延时
    let mut delay = cp.SYST.delay(ccdr.clocks);
    delay.delay_ms(2000_u16);
    loop {
        // your code goes here
        //todo:翻转led电平
        blue_led.toggle();
        delay.delay_ms(100_u16);
    }
}
