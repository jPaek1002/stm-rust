#[allow(unused)]
macro_rules! foreach_flash_region {
    ($($pat:tt => $code:tt;)*) => {
        macro_rules! __foreach_flash_region_inner {
            $(($pat) => $code;)*
            ($_:tt) => {}
        }
        __foreach_flash_region_inner!((Bank1Region,4,1024));
    };
}#[allow(unused)]
macro_rules! foreach_interrupt {
    ($($pat:tt => $code:tt;)*) => {
        macro_rules! __foreach_interrupt_inner {
            $(($pat) => $code;)*
            ($_:tt) => {}
        }
        __foreach_interrupt_inner!((ADC,adc,ADC,GLOBAL,ADC1));
        __foreach_interrupt_inner!((DMA1,bdma,DMA,CH1,DMA1_CHANNEL1));
        __foreach_interrupt_inner!((DMA1,bdma,DMA,CH2,DMA1_CHANNEL2_3));
        __foreach_interrupt_inner!((DMA1,bdma,DMA,CH3,DMA1_CHANNEL2_3));
        __foreach_interrupt_inner!((DMA1,bdma,DMA,CH4,DMA1_CHANNEL4_5));
        __foreach_interrupt_inner!((DMA1,bdma,DMA,CH5,DMA1_CHANNEL4_5));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI0,EXTI0_1));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI1,EXTI0_1));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI10,EXTI4_15));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI11,EXTI4_15));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI12,EXTI4_15));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI13,EXTI4_15));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI14,EXTI4_15));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI15,EXTI4_15));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI2,EXTI2_3));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI3,EXTI2_3));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI4,EXTI4_15));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI5,EXTI4_15));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI6,EXTI4_15));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI7,EXTI4_15));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI8,EXTI4_15));
        __foreach_interrupt_inner!((EXTI,exti,EXTI,EXTI9,EXTI4_15));
        __foreach_interrupt_inner!((FLASH,flash,FLASH,GLOBAL,FLASH));
        __foreach_interrupt_inner!((I2C1,i2c,I2C,ER,I2C1));
        __foreach_interrupt_inner!((I2C1,i2c,I2C,EV,I2C1));
        __foreach_interrupt_inner!((RCC,rcc,RCC,GLOBAL,RCC));
        __foreach_interrupt_inner!((RTC,rtc,RTC,ALARM,RTC));
        __foreach_interrupt_inner!((RTC,rtc,RTC,SSRU,RTC));
        __foreach_interrupt_inner!((RTC,rtc,RTC,STAMP,RTC));
        __foreach_interrupt_inner!((RTC,rtc,RTC,TAMP,RTC));
        __foreach_interrupt_inner!((RTC,rtc,RTC,WKUP,RTC));
        __foreach_interrupt_inner!((SPI1,spi,SPI,GLOBAL,SPI1));
        __foreach_interrupt_inner!((TIM1,timer,TIM_ADV,BRK,TIM1_BRK_UP_TRG_COM));
        __foreach_interrupt_inner!((TIM1,timer,TIM_ADV,CC,TIM1_CC));
        __foreach_interrupt_inner!((TIM1,timer,TIM_ADV,COM,TIM1_BRK_UP_TRG_COM));
        __foreach_interrupt_inner!((TIM1,timer,TIM_ADV,TRG,TIM1_BRK_UP_TRG_COM));
        __foreach_interrupt_inner!((TIM1,timer,TIM_ADV,UP,TIM1_BRK_UP_TRG_COM));
        __foreach_interrupt_inner!((TIM14,timer,TIM_GP16,BRK,TIM14));
        __foreach_interrupt_inner!((TIM14,timer,TIM_GP16,CC,TIM14));
        __foreach_interrupt_inner!((TIM14,timer,TIM_GP16,COM,TIM14));
        __foreach_interrupt_inner!((TIM14,timer,TIM_GP16,TRG,TIM14));
        __foreach_interrupt_inner!((TIM14,timer,TIM_GP16,UP,TIM14));
        __foreach_interrupt_inner!((TIM16,timer,TIM_GP16,BRK,TIM16));
        __foreach_interrupt_inner!((TIM16,timer,TIM_GP16,CC,TIM16));
        __foreach_interrupt_inner!((TIM16,timer,TIM_GP16,COM,TIM16));
        __foreach_interrupt_inner!((TIM16,timer,TIM_GP16,TRG,TIM16));
        __foreach_interrupt_inner!((TIM16,timer,TIM_GP16,UP,TIM16));
        __foreach_interrupt_inner!((TIM17,timer,TIM_GP16,BRK,TIM17));
        __foreach_interrupt_inner!((TIM17,timer,TIM_GP16,CC,TIM17));
        __foreach_interrupt_inner!((TIM17,timer,TIM_GP16,COM,TIM17));
        __foreach_interrupt_inner!((TIM17,timer,TIM_GP16,TRG,TIM17));
        __foreach_interrupt_inner!((TIM17,timer,TIM_GP16,UP,TIM17));
        __foreach_interrupt_inner!((TIM2,timer,TIM_GP32,BRK,TIM2));
        __foreach_interrupt_inner!((TIM2,timer,TIM_GP32,CC,TIM2));
        __foreach_interrupt_inner!((TIM2,timer,TIM_GP32,COM,TIM2));
        __foreach_interrupt_inner!((TIM2,timer,TIM_GP32,TRG,TIM2));
        __foreach_interrupt_inner!((TIM2,timer,TIM_GP32,UP,TIM2));
        __foreach_interrupt_inner!((TIM3,timer,TIM_GP16,BRK,TIM3));
        __foreach_interrupt_inner!((TIM3,timer,TIM_GP16,CC,TIM3));
        __foreach_interrupt_inner!((TIM3,timer,TIM_GP16,COM,TIM3));
        __foreach_interrupt_inner!((TIM3,timer,TIM_GP16,TRG,TIM3));
        __foreach_interrupt_inner!((TIM3,timer,TIM_GP16,UP,TIM3));
        __foreach_interrupt_inner!((USART1,usart,USART,GLOBAL,USART1));
        __foreach_interrupt_inner!((WWDG,wwdg,WWDG,GLOBAL,WWDG));
        __foreach_interrupt_inner!((WWDG,wwdg,WWDG,RST,WWDG));
        __foreach_interrupt_inner!((WWDG));
        __foreach_interrupt_inner!((PVD));
        __foreach_interrupt_inner!((RTC));
        __foreach_interrupt_inner!((FLASH));
        __foreach_interrupt_inner!((RCC));
        __foreach_interrupt_inner!((EXTI0_1));
        __foreach_interrupt_inner!((EXTI,EXTI0_1));
        __foreach_interrupt_inner!((EXTI2_3));
        __foreach_interrupt_inner!((EXTI,EXTI2_3));
        __foreach_interrupt_inner!((EXTI4_15));
        __foreach_interrupt_inner!((EXTI,EXTI4_15));
        __foreach_interrupt_inner!((DMA1_CHANNEL1));
        __foreach_interrupt_inner!((DMA1_CHANNEL2_3));
        __foreach_interrupt_inner!((DMA1_CHANNEL4_5));
        __foreach_interrupt_inner!((ADC1));
        __foreach_interrupt_inner!((TIM1_BRK_UP_TRG_COM));
        __foreach_interrupt_inner!((TIM1_CC));
        __foreach_interrupt_inner!((TIM2));
        __foreach_interrupt_inner!((TIM3));
        __foreach_interrupt_inner!((TIM14));
        __foreach_interrupt_inner!((TIM16));
        __foreach_interrupt_inner!((TIM17));
        __foreach_interrupt_inner!((I2C1));
        __foreach_interrupt_inner!((SPI1));
        __foreach_interrupt_inner!((USART1));
    };
}#[allow(unused)]
macro_rules! foreach_peripheral {
    ($($pat:tt => $code:tt;)*) => {
        macro_rules! __foreach_peripheral_inner {
            $(($pat) => $code;)*
            ($_:tt) => {}
        }
        __foreach_peripheral_inner!((adc,ADC));
        __foreach_peripheral_inner!((crc,CRC));
        __foreach_peripheral_inner!((dbgmcu,DBGMCU));
        __foreach_peripheral_inner!((bdma,DMA1));
        __foreach_peripheral_inner!((exti,EXTI));
        __foreach_peripheral_inner!((flash,FLASH));
        __foreach_peripheral_inner!((gpio,GPIOA));
        __foreach_peripheral_inner!((gpio,GPIOB));
        __foreach_peripheral_inner!((gpio,GPIOC));
        __foreach_peripheral_inner!((gpio,GPIOF));
        __foreach_peripheral_inner!((i2c,I2C1));
        __foreach_peripheral_inner!((iwdg,IWDG));
        __foreach_peripheral_inner!((rcc,RCC));
        __foreach_peripheral_inner!((rtc,RTC));
        __foreach_peripheral_inner!((spi,SPI1));
        __foreach_peripheral_inner!((syscfg,SYSCFG));
        __foreach_peripheral_inner!((timer,TIM1));
        __foreach_peripheral_inner!((timer,TIM14));
        __foreach_peripheral_inner!((timer,TIM16));
        __foreach_peripheral_inner!((timer,TIM17));
        __foreach_peripheral_inner!((timer,TIM2));
        __foreach_peripheral_inner!((timer,TIM3));
        __foreach_peripheral_inner!((uid,UID));
        __foreach_peripheral_inner!((usart,USART1));
        __foreach_peripheral_inner!((wwdg,WWDG));
    };
}#[allow(unused)]
macro_rules! foreach_pin {
    ($($pat:tt => $code:tt;)*) => {
        macro_rules! __foreach_pin_inner {
            $(($pat) => $code;)*
            ($_:tt) => {}
        }
        __foreach_pin_inner!((PA0,GPIOA,0,0,EXTI0));
        __foreach_pin_inner!((PA1,GPIOA,0,1,EXTI1));
        __foreach_pin_inner!((PA2,GPIOA,0,2,EXTI2));
        __foreach_pin_inner!((PA3,GPIOA,0,3,EXTI3));
        __foreach_pin_inner!((PA4,GPIOA,0,4,EXTI4));
        __foreach_pin_inner!((PA5,GPIOA,0,5,EXTI5));
        __foreach_pin_inner!((PA6,GPIOA,0,6,EXTI6));
        __foreach_pin_inner!((PA7,GPIOA,0,7,EXTI7));
        __foreach_pin_inner!((PA8,GPIOA,0,8,EXTI8));
        __foreach_pin_inner!((PA9,GPIOA,0,9,EXTI9));
        __foreach_pin_inner!((PA10,GPIOA,0,10,EXTI10));
        __foreach_pin_inner!((PA11,GPIOA,0,11,EXTI11));
        __foreach_pin_inner!((PA12,GPIOA,0,12,EXTI12));
        __foreach_pin_inner!((PA13,GPIOA,0,13,EXTI13));
        __foreach_pin_inner!((PA14,GPIOA,0,14,EXTI14));
        __foreach_pin_inner!((PA15,GPIOA,0,15,EXTI15));
        __foreach_pin_inner!((PB0,GPIOB,1,0,EXTI0));
        __foreach_pin_inner!((PB1,GPIOB,1,1,EXTI1));
        __foreach_pin_inner!((PB2,GPIOB,1,2,EXTI2));
        __foreach_pin_inner!((PB3,GPIOB,1,3,EXTI3));
        __foreach_pin_inner!((PB4,GPIOB,1,4,EXTI4));
        __foreach_pin_inner!((PB5,GPIOB,1,5,EXTI5));
        __foreach_pin_inner!((PB6,GPIOB,1,6,EXTI6));
        __foreach_pin_inner!((PB7,GPIOB,1,7,EXTI7));
        __foreach_pin_inner!((PB8,GPIOB,1,8,EXTI8));
        __foreach_pin_inner!((PB9,GPIOB,1,9,EXTI9));
        __foreach_pin_inner!((PB10,GPIOB,1,10,EXTI10));
        __foreach_pin_inner!((PB11,GPIOB,1,11,EXTI11));
        __foreach_pin_inner!((PB12,GPIOB,1,12,EXTI12));
        __foreach_pin_inner!((PB13,GPIOB,1,13,EXTI13));
        __foreach_pin_inner!((PB14,GPIOB,1,14,EXTI14));
        __foreach_pin_inner!((PB15,GPIOB,1,15,EXTI15));
        __foreach_pin_inner!((PC0,GPIOC,2,0,EXTI0));
        __foreach_pin_inner!((PC1,GPIOC,2,1,EXTI1));
        __foreach_pin_inner!((PC2,GPIOC,2,2,EXTI2));
        __foreach_pin_inner!((PC3,GPIOC,2,3,EXTI3));
        __foreach_pin_inner!((PC4,GPIOC,2,4,EXTI4));
        __foreach_pin_inner!((PC5,GPIOC,2,5,EXTI5));
        __foreach_pin_inner!((PC6,GPIOC,2,6,EXTI6));
        __foreach_pin_inner!((PC7,GPIOC,2,7,EXTI7));
        __foreach_pin_inner!((PC8,GPIOC,2,8,EXTI8));
        __foreach_pin_inner!((PC9,GPIOC,2,9,EXTI9));
        __foreach_pin_inner!((PC10,GPIOC,2,10,EXTI10));
        __foreach_pin_inner!((PC11,GPIOC,2,11,EXTI11));
        __foreach_pin_inner!((PC12,GPIOC,2,12,EXTI12));
        __foreach_pin_inner!((PC13,GPIOC,2,13,EXTI13));
        __foreach_pin_inner!((PC14,GPIOC,2,14,EXTI14));
        __foreach_pin_inner!((PC15,GPIOC,2,15,EXTI15));
        __foreach_pin_inner!((PF0,GPIOF,5,0,EXTI0));
        __foreach_pin_inner!((PF1,GPIOF,5,1,EXTI1));
        __foreach_pin_inner!((PF2,GPIOF,5,2,EXTI2));
        __foreach_pin_inner!((PF3,GPIOF,5,3,EXTI3));
        __foreach_pin_inner!((PF4,GPIOF,5,4,EXTI4));
        __foreach_pin_inner!((PF5,GPIOF,5,5,EXTI5));
        __foreach_pin_inner!((PF6,GPIOF,5,6,EXTI6));
        __foreach_pin_inner!((PF7,GPIOF,5,7,EXTI7));
        __foreach_pin_inner!((PF8,GPIOF,5,8,EXTI8));
        __foreach_pin_inner!((PF9,GPIOF,5,9,EXTI9));
        __foreach_pin_inner!((PF10,GPIOF,5,10,EXTI10));
        __foreach_pin_inner!((PF11,GPIOF,5,11,EXTI11));
        __foreach_pin_inner!((PF12,GPIOF,5,12,EXTI12));
        __foreach_pin_inner!((PF13,GPIOF,5,13,EXTI13));
        __foreach_pin_inner!((PF14,GPIOF,5,14,EXTI14));
        __foreach_pin_inner!((PF15,GPIOF,5,15,EXTI15));
    };
}#[allow(unused)]
macro_rules! foreach_dma_channel {
    ($($pat:tt => $code:tt;)*) => {
        macro_rules! __foreach_dma_channel_inner {
            $(($pat) => $code;)*
            ($_:tt) => {}
        }
        __foreach_dma_channel_inner!((DMA1_CH1,DMA1,bdma,0,0,{}));
        __foreach_dma_channel_inner!((DMA1_CH2,DMA1,bdma,1,1,{}));
        __foreach_dma_channel_inner!((DMA1_CH3,DMA1,bdma,2,2,{}));
        __foreach_dma_channel_inner!((DMA1_CH4,DMA1,bdma,3,3,{}));
        __foreach_dma_channel_inner!((DMA1_CH5,DMA1,bdma,4,4,{}));
    };
}