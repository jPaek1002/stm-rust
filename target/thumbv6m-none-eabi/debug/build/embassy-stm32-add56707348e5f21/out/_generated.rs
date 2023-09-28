embassy_hal_internal :: peripherals_definition ! (ADC , CRC , DBGMCU , DMA1 , FLASH , PA0 , PA1 , PA2 , PA3 , PA4 , PA5 , PA6 , PA7 , PA8 , PA9 , PA10 , PA11 , PA12 , PA13 , PA14 , PA15 , PB0 , PB1 , PB2 , PB3 , PB4 , PB5 , PB6 , PB7 , PB8 , PB9 , PB10 , PB11 , PB12 , PB13 , PB14 , PB15 , PC0 , PC1 , PC2 , PC3 , PC4 , PC5 , PC6 , PC7 , PC8 , PC9 , PC10 , PC11 , PC12 , PC13 , PC14 , PC15 , PF0 , PF1 , PF2 , PF3 , PF4 , PF5 , PF6 , PF7 , PF8 , PF9 , PF10 , PF11 , PF12 , PF13 , PF14 , PF15 , I2C1 , IWDG , RCC , RTC , SPI1 , SYSCFG , TIM1 , TIM14 , TIM16 , TIM17 , TIM2 , TIM3 , UID , USART1 , WWDG , EXTI0 , EXTI1 , EXTI2 , EXTI3 , EXTI4 , EXTI5 , EXTI6 , EXTI7 , EXTI8 , EXTI9 , EXTI10 , EXTI11 , EXTI12 , EXTI13 , EXTI14 , EXTI15 , DMA1_CH1 , DMA1_CH2 , DMA1_CH3 , DMA1_CH4 , DMA1_CH5) ; embassy_hal_internal :: peripherals_struct ! (ADC , CRC , DBGMCU , DMA1 , FLASH , PA0 , PA1 , PA2 , PA3 , PA4 , PA5 , PA6 , PA7 , PA8 , PA9 , PA10 , PA11 , PA12 , PA13 , PA14 , PA15 , PB0 , PB1 , PB2 , PB3 , PB4 , PB5 , PB6 , PB7 , PB8 , PB9 , PB10 , PB11 , PB12 , PB13 , PB14 , PB15 , PC0 , PC1 , PC2 , PC3 , PC4 , PC5 , PC6 , PC7 , PC8 , PC9 , PC10 , PC11 , PC12 , PC13 , PC14 , PC15 , PF0 , PF1 , PF2 , PF3 , PF4 , PF5 , PF6 , PF7 , PF8 , PF9 , PF10 , PF11 , PF12 , PF13 , PF14 , PF15 , I2C1 , IWDG , RCC , RTC , SPI1 , SYSCFG , TIM1 , TIM14 , TIM16 , TIM17 , TIM3 , UID , USART1 , WWDG , EXTI0 , EXTI1 , EXTI2 , EXTI3 , EXTI4 , EXTI5 , EXTI6 , EXTI7 , EXTI8 , EXTI9 , EXTI10 , EXTI11 , EXTI12 , EXTI13 , EXTI14 , EXTI15 , DMA1_CH1 , DMA1_CH2 , DMA1_CH3 , DMA1_CH4 , DMA1_CH5) ; embassy_hal_internal :: interrupt_mod ! (WWDG , PVD , RTC , FLASH , RCC , EXTI0_1 , EXTI2_3 , EXTI4_15 , DMA1_CHANNEL1 , DMA1_CHANNEL2_3 , DMA1_CHANNEL4_5 , ADC1 , TIM1_BRK_UP_TRG_COM , TIM1_CC , TIM2 , TIM3 , TIM14 , TIM16 , TIM17 , I2C1 , SPI1 , USART1 ,) ; pub const MAX_ERASE_SIZE : usize = 1024u32 as usize ; pub mod flash_regions { pub const BANK1_REGION : crate :: flash :: FlashRegion = crate :: flash :: FlashRegion { bank : crate :: flash :: FlashBank :: Bank1 , base : 134217728u32 , size : 32768u32 , erase_size : 1024u32 , write_size : 4u32 , erase_value : 255u8 , _ensure_internal : () , } ; # [cfg (flash)] pub struct Bank1Region < 'd , MODE = crate :: flash :: Async > (pub & 'static crate :: flash :: FlashRegion , pub (crate) embassy_hal_internal :: PeripheralRef < 'd , crate :: peripherals :: FLASH > , pub (crate) core :: marker :: PhantomData < MODE >) ; # [cfg (flash)] pub struct FlashLayout < 'd , MODE = crate :: flash :: Async > { pub bank1_region : Bank1Region < 'd , MODE > , _mode : core :: marker :: PhantomData < MODE > , } # [cfg (flash)] impl < 'd , MODE > FlashLayout < 'd , MODE > { pub (crate) fn new (p : embassy_hal_internal :: PeripheralRef < 'd , crate :: peripherals :: FLASH >) -> Self { Self { bank1_region : Bank1Region (& BANK1_REGION , unsafe { p . clone_unchecked () } , core :: marker :: PhantomData) , _mode : core :: marker :: PhantomData , } } } pub const FLASH_REGIONS : [& crate :: flash :: FlashRegion ; 1usize] = [& BANK1_REGION] ; } # [cfg (feature = "rt")] # [crate :: interrupt] unsafe fn DMA1_CHANNEL1 () { < crate :: peripherals :: DMA1_CH1 as crate :: dma :: bdma :: sealed :: Channel > :: on_irq () ; } # [cfg (feature = "rt")] # [crate :: interrupt] unsafe fn DMA1_CHANNEL2_3 () { < crate :: peripherals :: DMA1_CH2 as crate :: dma :: bdma :: sealed :: Channel > :: on_irq () ; < crate :: peripherals :: DMA1_CH3 as crate :: dma :: bdma :: sealed :: Channel > :: on_irq () ; } # [cfg (feature = "rt")] # [crate :: interrupt] unsafe fn DMA1_CHANNEL4_5 () { < crate :: peripherals :: DMA1_CH4 as crate :: dma :: bdma :: sealed :: Channel > :: on_irq () ; < crate :: peripherals :: DMA1_CH5 as crate :: dma :: bdma :: sealed :: Channel > :: on_irq () ; } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: ADC { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . apb2 } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . apb2enr () . modify (| w | w . set_adcen (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2enr () . modify (| w | w . set_adcen (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_adcrst (true)) ; crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_adcrst (false)) ; }) ; } } impl crate :: rcc :: RccPeripheral for peripherals :: ADC { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: CRC { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . ahb1 } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . ahbenr () . modify (| w | w . set_crcen (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . ahbenr () . modify (| w | w . set_crcen (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { } } impl crate :: rcc :: RccPeripheral for peripherals :: CRC { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: DBGMCU { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . apb2 } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . apb2enr () . modify (| w | w . set_dbgmcuen (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2enr () . modify (| w | w . set_dbgmcuen (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_dbgmcurst (true)) ; crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_dbgmcurst (false)) ; }) ; } } impl crate :: rcc :: RccPeripheral for peripherals :: DBGMCU { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: DMA1 { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . ahb1 } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . ahbenr () . modify (| w | w . set_dmaen (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . ahbenr () . modify (| w | w . set_dmaen (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { } } impl crate :: rcc :: RccPeripheral for peripherals :: DMA1 { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: FLASH { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . ahb1 } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . ahbenr () . modify (| w | w . set_flashen (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . ahbenr () . modify (| w | w . set_flashen (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { } } impl crate :: rcc :: RccPeripheral for peripherals :: FLASH { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: I2C1 { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . apb1 } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . apb1enr () . modify (| w | w . set_i2c1en (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . apb1enr () . modify (| w | w . set_i2c1en (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { critical_section :: with (| _ | { crate :: pac :: RCC . apb1rstr () . modify (| w | w . set_i2c1rst (true)) ; crate :: pac :: RCC . apb1rstr () . modify (| w | w . set_i2c1rst (false)) ; }) ; } } impl crate :: rcc :: RccPeripheral for peripherals :: I2C1 { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: SPI1 { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . apb2 } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . apb2enr () . modify (| w | w . set_spi1en (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2enr () . modify (| w | w . set_spi1en (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_spi1rst (true)) ; crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_spi1rst (false)) ; }) ; } } impl crate :: rcc :: RccPeripheral for peripherals :: SPI1 { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: SYSCFG { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . apb2 } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . apb2enr () . modify (| w | w . set_syscfgen (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2enr () . modify (| w | w . set_syscfgen (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_syscfgrst (true)) ; crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_syscfgrst (false)) ; }) ; } } impl crate :: rcc :: RccPeripheral for peripherals :: SYSCFG { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: TIM1 { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . apb2_tim } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . apb2enr () . modify (| w | w . set_tim1en (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2enr () . modify (| w | w . set_tim1en (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_tim1rst (true)) ; crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_tim1rst (false)) ; }) ; } } impl crate :: rcc :: RccPeripheral for peripherals :: TIM1 { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: TIM14 { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . apb1_tim } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . apb1enr () . modify (| w | w . set_tim14en (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . apb1enr () . modify (| w | w . set_tim14en (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { critical_section :: with (| _ | { crate :: pac :: RCC . apb1rstr () . modify (| w | w . set_tim14rst (true)) ; crate :: pac :: RCC . apb1rstr () . modify (| w | w . set_tim14rst (false)) ; }) ; } } impl crate :: rcc :: RccPeripheral for peripherals :: TIM14 { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: TIM16 { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . apb2_tim } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . apb2enr () . modify (| w | w . set_tim16en (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2enr () . modify (| w | w . set_tim16en (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_tim16rst (true)) ; crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_tim16rst (false)) ; }) ; } } impl crate :: rcc :: RccPeripheral for peripherals :: TIM16 { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: TIM17 { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . apb2_tim } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . apb2enr () . modify (| w | w . set_tim17en (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2enr () . modify (| w | w . set_tim17en (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_tim17rst (true)) ; crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_tim17rst (false)) ; }) ; } } impl crate :: rcc :: RccPeripheral for peripherals :: TIM17 { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: TIM2 { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . apb1_tim } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . apb1enr () . modify (| w | w . set_tim2en (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . apb1enr () . modify (| w | w . set_tim2en (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { critical_section :: with (| _ | { crate :: pac :: RCC . apb1rstr () . modify (| w | w . set_tim2rst (true)) ; crate :: pac :: RCC . apb1rstr () . modify (| w | w . set_tim2rst (false)) ; }) ; } } impl crate :: rcc :: RccPeripheral for peripherals :: TIM2 { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: TIM3 { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . apb1_tim } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . apb1enr () . modify (| w | w . set_tim3en (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . apb1enr () . modify (| w | w . set_tim3en (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { critical_section :: with (| _ | { crate :: pac :: RCC . apb1rstr () . modify (| w | w . set_tim3rst (true)) ; crate :: pac :: RCC . apb1rstr () . modify (| w | w . set_tim3rst (false)) ; }) ; } } impl crate :: rcc :: RccPeripheral for peripherals :: TIM3 { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: USART1 { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . apb2 } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . apb2enr () . modify (| w | w . set_usart1en (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2enr () . modify (| w | w . set_usart1en (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { critical_section :: with (| _ | { crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_usart1rst (true)) ; crate :: pac :: RCC . apb2rstr () . modify (| w | w . set_usart1rst (false)) ; }) ; } } impl crate :: rcc :: RccPeripheral for peripherals :: USART1 { } impl crate :: rcc :: sealed :: RccPeripheral for peripherals :: WWDG { fn frequency () -> crate :: time :: Hertz { unsafe { crate :: rcc :: get_freqs () . apb1 } } fn enable () { critical_section :: with (| _ | { # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_add () ; crate :: pac :: RCC . apb1enr () . modify (| w | w . set_wwdgen (true)) ; }) } fn disable () { critical_section :: with (| _ | { crate :: pac :: RCC . apb1enr () . modify (| w | w . set_wwdgen (false)) ; # [cfg (feature = "low-power")] crate :: rcc :: clock_refcount_sub () ; }) } fn reset () { critical_section :: with (| _ | { crate :: pac :: RCC . apb1rstr () . modify (| w | w . set_wwdgrst (true)) ; crate :: pac :: RCC . apb1rstr () . modify (| w | w . set_wwdgrst (false)) ; }) ; } } impl crate :: rcc :: RccPeripheral for peripherals :: WWDG { } pub unsafe fn init_dma () { } pub unsafe fn init_bdma () { crate :: pac :: RCC . ahbenr () . modify (| w | w . set_dmaen (true)) ; } pub unsafe fn init_dmamux () { } pub unsafe fn init_gpdma () { } pub unsafe fn init_gpio () { crate :: pac :: RCC . ahbenr () . modify (| w | w . set_gpioaen (true)) ; crate :: pac :: RCC . ahbenr () . modify (| w | w . set_gpioben (true)) ; crate :: pac :: RCC . ahbenr () . modify (| w | w . set_gpiocen (true)) ; crate :: pac :: RCC . ahbenr () . modify (| w | w . set_gpiofen (true)) ; } impl_adc_pin ! (ADC , PA0 , 0u8) ; impl_adc_pin ! (ADC , PA1 , 1u8) ; impl_adc_pin ! (ADC , PA2 , 2u8) ; impl_adc_pin ! (ADC , PA3 , 3u8) ; impl_adc_pin ! (ADC , PA4 , 4u8) ; impl_adc_pin ! (ADC , PA5 , 5u8) ; impl_adc_pin ! (ADC , PA6 , 6u8) ; impl_adc_pin ! (ADC , PA7 , 7u8) ; impl_adc_pin ! (ADC , PB0 , 8u8) ; impl_adc_pin ! (ADC , PB1 , 9u8) ; pin_trait_impl ! (crate :: i2c :: SclPin , I2C1 , PA9 , 4u8) ; pin_trait_impl ! (crate :: i2c :: SdaPin , I2C1 , PA10 , 4u8) ; pin_trait_impl ! (crate :: i2c :: SclPin , I2C1 , PB6 , 1u8) ; pin_trait_impl ! (crate :: i2c :: SdaPin , I2C1 , PB7 , 1u8) ; pin_trait_impl ! (crate :: i2c :: SclPin , I2C1 , PB8 , 1u8) ; pin_trait_impl ! (crate :: spi :: CsPin , SPI1 , PA4 , 0u8) ; pin_trait_impl ! (crate :: spi :: SckPin , SPI1 , PA5 , 0u8) ; pin_trait_impl ! (crate :: spi :: MisoPin , SPI1 , PA6 , 0u8) ; pin_trait_impl ! (crate :: spi :: MosiPin , SPI1 , PA7 , 0u8) ; pin_trait_impl ! (crate :: spi :: CsPin , SPI1 , PA15 , 0u8) ; pin_trait_impl ! (crate :: spi :: SckPin , SPI1 , PB3 , 0u8) ; pin_trait_impl ! (crate :: spi :: MisoPin , SPI1 , PB4 , 0u8) ; pin_trait_impl ! (crate :: spi :: MosiPin , SPI1 , PB5 , 0u8) ; pin_trait_impl ! (crate :: spi :: WsPin , SPI1 , PA4 , 0u8) ; pin_trait_impl ! (crate :: spi :: CkPin , SPI1 , PA5 , 0u8) ; pin_trait_impl ! (crate :: spi :: MckPin , SPI1 , PA6 , 0u8) ; pin_trait_impl ! (crate :: spi :: WsPin , SPI1 , PA15 , 0u8) ; pin_trait_impl ! (crate :: spi :: CkPin , SPI1 , PB3 , 0u8) ; pin_trait_impl ! (crate :: spi :: MckPin , SPI1 , PB4 , 0u8) ; pin_trait_impl ! (crate :: timer :: BreakInputPin , TIM1 , PA6 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel1ComplementaryPin , TIM1 , PA7 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel1Pin , TIM1 , PA8 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel2Pin , TIM1 , PA9 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel3Pin , TIM1 , PA10 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel4Pin , TIM1 , PA11 , 2u8) ; pin_trait_impl ! (crate :: timer :: ExternalTriggerPin , TIM1 , PA12 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel2ComplementaryPin , TIM1 , PB0 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel3ComplementaryPin , TIM1 , PB1 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel1Pin , TIM14 , PA4 , 4u8) ; pin_trait_impl ! (crate :: timer :: Channel1Pin , TIM14 , PA7 , 4u8) ; pin_trait_impl ! (crate :: timer :: Channel1Pin , TIM14 , PB1 , 0u8) ; pin_trait_impl ! (crate :: timer :: Channel1Pin , TIM16 , PA6 , 5u8) ; pin_trait_impl ! (crate :: timer :: BreakInputPin , TIM16 , PB5 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel1ComplementaryPin , TIM16 , PB6 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel1Pin , TIM16 , PB8 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel1Pin , TIM17 , PA7 , 5u8) ; pin_trait_impl ! (crate :: timer :: BreakInputPin , TIM17 , PA10 , 0u8) ; pin_trait_impl ! (crate :: timer :: Channel1ComplementaryPin , TIM17 , PB7 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel1Pin , TIM2 , PA0 , 2u8) ; pin_trait_impl ! (crate :: timer :: ExternalTriggerPin , TIM2 , PA0 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel2Pin , TIM2 , PA1 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel3Pin , TIM2 , PA2 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel4Pin , TIM2 , PA3 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel1Pin , TIM2 , PA5 , 2u8) ; pin_trait_impl ! (crate :: timer :: ExternalTriggerPin , TIM2 , PA5 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel1Pin , TIM2 , PA15 , 2u8) ; pin_trait_impl ! (crate :: timer :: ExternalTriggerPin , TIM2 , PA15 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel2Pin , TIM2 , PB3 , 2u8) ; pin_trait_impl ! (crate :: timer :: Channel1Pin , TIM3 , PA6 , 1u8) ; pin_trait_impl ! (crate :: timer :: Channel2Pin , TIM3 , PA7 , 1u8) ; pin_trait_impl ! (crate :: timer :: Channel3Pin , TIM3 , PB0 , 1u8) ; pin_trait_impl ! (crate :: timer :: Channel4Pin , TIM3 , PB1 , 1u8) ; pin_trait_impl ! (crate :: timer :: Channel1Pin , TIM3 , PB4 , 1u8) ; pin_trait_impl ! (crate :: timer :: Channel2Pin , TIM3 , PB5 , 1u8) ; pin_trait_impl ! (crate :: usart :: CtsPin , USART1 , PA0 , 1u8) ; pin_trait_impl ! (crate :: usart :: DePin , USART1 , PA1 , 1u8) ; pin_trait_impl ! (crate :: usart :: RtsPin , USART1 , PA1 , 1u8) ; pin_trait_impl ! (crate :: usart :: TxPin , USART1 , PA2 , 1u8) ; pin_trait_impl ! (crate :: usart :: RxPin , USART1 , PA3 , 1u8) ; pin_trait_impl ! (crate :: usart :: CkPin , USART1 , PA4 , 1u8) ; pin_trait_impl ! (crate :: usart :: CkPin , USART1 , PA8 , 1u8) ; pin_trait_impl ! (crate :: usart :: TxPin , USART1 , PA9 , 1u8) ; pin_trait_impl ! (crate :: usart :: RxPin , USART1 , PA10 , 1u8) ; pin_trait_impl ! (crate :: usart :: CtsPin , USART1 , PA11 , 1u8) ; pin_trait_impl ! (crate :: usart :: DePin , USART1 , PA12 , 1u8) ; pin_trait_impl ! (crate :: usart :: RtsPin , USART1 , PA12 , 1u8) ; pin_trait_impl ! (crate :: usart :: TxPin , USART1 , PA14 , 1u8) ; pin_trait_impl ! (crate :: usart :: RxPin , USART1 , PA15 , 1u8) ; pin_trait_impl ! (crate :: usart :: TxPin , USART1 , PB6 , 0u8) ; pin_trait_impl ! (crate :: usart :: RxPin , USART1 , PB7 , 0u8) ; dma_trait_impl ! (crate :: i2c :: TxDma , I2C1 , { channel : DMA1_CH2 } , ()) ; dma_trait_impl ! (crate :: i2c :: RxDma , I2C1 , { channel : DMA1_CH3 } , ()) ; dma_trait_impl ! (crate :: spi :: RxDma , SPI1 , { channel : DMA1_CH2 } , ()) ; dma_trait_impl ! (crate :: spi :: TxDma , SPI1 , { channel : DMA1_CH3 } , ()) ; dma_trait_impl ! (crate :: usart :: TxDma , USART1 , { channel : DMA1_CH2 } , ()) ; dma_trait_impl ! (crate :: usart :: RxDma , USART1 , { channel : DMA1_CH3 } , ()) ; dma_trait_impl ! (crate :: usart :: TxDma , USART1 , { channel : DMA1_CH4 } , ()) ; dma_trait_impl ! (crate :: usart :: RxDma , USART1 , { channel : DMA1_CH5 } , ()) ; pub (crate) const DMA_CHANNEL_COUNT : usize = 0usize ; pub (crate) const BDMA_CHANNEL_COUNT : usize = 5usize ; pub (crate) const GPDMA_CHANNEL_COUNT : usize = 0usize ;