#![no_std]
#![no_main]

use core::{
    ptr::{read_volatile, write_volatile},
    arch::asm
};

const GPIOD_BASE: u32 = 0x40020C00;
const PERIPHERALS_BASE: u32 = 0x40000000;
const AHB1_BASE: u32 = PERIPHERALS_BASE + 0x00020000;
const RCC_BASE: u32 = AHB1_BASE + 0x3800;

fn main() -> ! {
    unsafe {
        // Port D
        write_volatile((RCC_BASE + 0x30) as *mut u32, read_volatile((RCC_BASE + 0x30) as *mut u32) | (1<<3));
        // MODER
        write_volatile((GPIOD_BASE) as *mut u32, read_volatile((GPIOD_BASE) as *mut u32) & !(3<<(2*13)));
        write_volatile((GPIOD_BASE) as *mut u32, read_volatile((GPIOD_BASE) as *mut u32) | (1<<(2*13)));
        // OTYPER
        write_volatile((GPIOD_BASE + 0x04) as *mut u32, read_volatile((GPIOD_BASE + 0x04) as *mut u32) & !(1<<1));
        // ospeedr
        write_volatile((GPIOD_BASE + 0x08) as *mut u32, read_volatile((GPIOD_BASE + 0x08) as *mut u32) | 3<<(2*13));
        // PUPDR
        write_volatile((GPIOD_BASE + 0x0C) as *mut u32, read_volatile((GPIOD_BASE + 0x0C) as *mut u32) & !(3<<(2*13)));

        loop {

            // SET LED
            // write_volatile((GPIOD_BASE + 0x18) as *mut u32, read_volatile((GPIOD_BASE + 0x18) as *mut u32) | (1<<13));
            *((GPIOD_BASE + 0x18) as *mut u32) |= 1<<13;

            for _ in 1..200000 { asm!("NOP") };

            // CLEAR LED
            // write_volatile((GPIOD_BASE + 0x18) as *mut u32, read_volatile((GPIOD_BASE + 0x18) as *mut u32) | ((1<<13)<<16));
            *((GPIOD_BASE + 0x18) as *mut u32) |= (1<<13)<<16;

            for _ in 1..200000 { asm!("NOP") };
        }
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub fn reset_handler() -> ! {
    main()
}

fn infinite_loop() -> ! {
    loop {}
}

fn dummy_handler() -> ! {
    loop {}
}

#[link_section = ".stacktop"]
#[no_mangle]
static __stacktop: u32 = 0x20020000;

#[link_section = ".vector_table"]
#[no_mangle]
static __vector_table: [fn() -> !; 97] = [
    reset_handler, 
    infinite_loop, 
    infinite_loop,
    infinite_loop,
    infinite_loop,
    infinite_loop,
    dummy_handler,
    dummy_handler,
    dummy_handler,
    dummy_handler,
    infinite_loop,
    infinite_loop,
    dummy_handler,
    infinite_loop,
    infinite_loop,

      /* External Interrupts */
    infinite_loop,   /* Window WatchDog              */                                        
    infinite_loop,   /* PVD through EXTI Line detection */                        
    infinite_loop,   /* Tamper and TimeStamps through the EXTI line */            
    infinite_loop,   /* RTC Wakeup through the EXTI line */                      
    infinite_loop,   /* FLASH                        */                                          
    infinite_loop,   /* RCC                          */                                            
    infinite_loop,   /* EXTI Line0                   */                        
    infinite_loop,   /* EXTI Line1                   */                          
    infinite_loop,   /* EXTI Line2                   */                          
    infinite_loop,   /* EXTI Line3                   */                          
    infinite_loop,   /* EXTI Line4                   */                          
    infinite_loop,   /* DMA1 Stream 0                */                  
    infinite_loop,   /* DMA1 Stream 1                */                   
    infinite_loop,   /* DMA1 Stream 2                */                   
    infinite_loop,   /* DMA1 Stream 3                */                   
    infinite_loop,   /* DMA1 Stream 4                */                   
    infinite_loop,   /* DMA1 Stream 5                */                   
    infinite_loop,   /* DMA1 Stream 6                */                   
    infinite_loop,   /* ADC1, ADC2 and ADC3s         */                   
    infinite_loop,   /* CAN1 TX                      */                         
    infinite_loop,   /* CAN1 RX0                     */                          
    infinite_loop,   /* CAN1 RX1                     */                          
    infinite_loop,   /* CAN1 SCE                     */                          
    infinite_loop,   /* External Line[9:5]s          */                          
    infinite_loop,   /* TIM1 Break and TIM9          */         
    infinite_loop,   /* TIM1 Update and TIM10        */         
    infinite_loop,   /* TIM1 Trigger and Commutation and TIM11 */
    infinite_loop,   /* TIM1 Capture Compare         */                          
    infinite_loop,   /* TIM2                         */                   
    infinite_loop,   /* TIM3                         */                   
    infinite_loop,   /* TIM4                         */                   
    infinite_loop,   /* I2C1 Event                   */                          
    infinite_loop,   /* I2C1 Error                   */                          
    infinite_loop,   /* I2C2 Event                   */                          
    infinite_loop,   /* I2C2 Error                   */                            
    infinite_loop,   /* SPI1                         */                   
    infinite_loop,   /* SPI2                         */                   
    infinite_loop,   /* USART1                       */                   
    infinite_loop,   /* USART2                       */                   
    infinite_loop,   /* USART3                       */                   
    infinite_loop,   /* External Line[15:10]s        */                          
    infinite_loop,   /* RTC Alarm (A and B) through EXTI Line */                 
    infinite_loop,   /* USB OTG FS Wakeup through EXTI line */                       
    infinite_loop,   /* TIM8 Break and TIM12         */         
    infinite_loop,   /* TIM8 Update and TIM13        */         
    infinite_loop,   /* TIM8 Trigger and Commutation and TIM14 */
    infinite_loop,   /* TIM8 Capture Compare         */                          
    infinite_loop,   /* DMA1 Stream7                 */                          
    infinite_loop,   /* FSMC                         */                   
    infinite_loop,   /* SDIO                         */                   
    infinite_loop,   /* TIM5                         */                   
    infinite_loop,   /* SPI3                         */                   
    infinite_loop,   /* UART4                        */                   
    infinite_loop,   /* UART5                        */                   
    infinite_loop,   /* TIM6 and DAC1&2 underrun errors */                   
    infinite_loop,   /* TIM7                         */
    infinite_loop,   /* DMA2 Stream 0                */                   
    infinite_loop,   /* DMA2 Stream 1                */                   
    infinite_loop,   /* DMA2 Stream 2                */                   
    infinite_loop,   /* DMA2 Stream 3                */                   
    infinite_loop,   /* DMA2 Stream 4                */                   
    infinite_loop,   /* Ethernet                     */                   
    infinite_loop,   /* Ethernet Wakeup through EXTI line */                     
    infinite_loop,   /* CAN2 TX                      */                          
    infinite_loop,   /* CAN2 RX0                     */                          
    infinite_loop,   /* CAN2 RX1                     */                          
    infinite_loop,   /* CAN2 SCE                     */                          
    infinite_loop,   /* USB OTG FS                   */                   
    infinite_loop,   /* DMA2 Stream 5                */                   
    infinite_loop,   /* DMA2 Stream 6                */                   
    infinite_loop,   /* DMA2 Stream 7                */                   
    infinite_loop,   /* USART6                       */                    
    infinite_loop,   /* I2C3 event                   */                          
    infinite_loop,   /* I2C3 error                   */                          
    infinite_loop,   /* USB OTG HS End Point 1 Out   */                   
    infinite_loop,   /* USB OTG HS End Point 1 In    */                   
    infinite_loop,   /* USB OTG HS Wakeup through EXTI */                         
    infinite_loop,   /* USB OTG HS                   */                   
    infinite_loop,   /* DCMI                         */                   
    dummy_handler,   /* CRYP crypto                  */                   
    infinite_loop,   /* Hash and Rng                 */
    infinite_loop,   /* FPU                          */
];