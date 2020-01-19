
// ==========================
// adapter MCU
// ==========================

// channel1 pin assignements
#define MCU_CH1_ISET_INDEX GPIO7
#define MCU_CH1_ISET_PORT  GPIOA
#define MCU_CH1_VSET_INDEX GPIO1
#define MCU_CH1_VSET_PORT  GPIOB
#define MCU_CH1_VOUT_ADC_INDEX GPIO0
#define MCU_CH1_VOUT_ADC_PORT GPIOB
#define MCU_CH1_CLIM_INDEX GPIO11
#define MCU_CH1_CLIM_PORT GPIOB
#define MCU_CH1_CONV_EN_INDEX GPIO12
#define MCU_CH1_CONV_EN_PORT GPIOA

// channel2 pin assignements
#define MCU_CH2_ISET_INDEX GPIO4
#define MCU_CH2_ISET_PORT  GPIOA
#define MCU_CH2_VSET_INDEX GPIO6
#define MCU_CH2_VSET_PORT  GPIOA
#define MCU_CH2_VOUT_ADC_INDEX GPIO5
#define MCU_CH2_VOUT_ADC_PORT  GPIOA
#define MCU_CH2_CLIM_INDEX GPIO11
#define MCU_CH2_CLIM_PORT GPIOB
#define MCU_CH2_BOOST_EN_INDEX GPIO10
#define MCU_CH2_BOOST_EN_PORT GPIOB

// channel3 pin assignements
#define MCU_CH3_EN_INDEX GPIO15
#define MCU_CH3_EN_PORT  GPIOC
#define MCU_CH3_CTRL_INDEX GPIO14
#define MCU_CH3_CTRL_PORT GPIOC

// ADC pin assignments
#define MCU_REF_VLT_ADC_INDEX GPIO0
#define MCU_REF_VLT_ADC_PORT GPIOA
#define MCU_SUPPLY_VLT_ADC_INDEX GPIO1
#define MCU_SUPPLY_VLT_ADC_PORT GPIOA

// I2C pin assignments
#define MCU_I2C1_SCLK_INDEX GPIO8
#define MCU_I2C1_SDA_INDEX GPIO9
#define MCU_I2C1_PORT GPIOB

#define MCU_I2C2_SCLK_INDEX GPIO6
#define MCU_I2C2_SDA_INDEX GPIO7
#define MCU_I2C2_PORT GPIOF

// SWD pin assignments
#define MCU_SWDIO_INDEX GPIO13
#define MCU_SWCLK_INDEX GPIO14
#define MCU_SWD_PORT GPIOA

// USART pin assigments
#define MCU_USART1_TX_INDEX GPIO2
#define MCU_USART1_RX_INDEX GPIO3
#define MCU_USART1_PORT GPIOA

// ==========================
// adapter LCD 
// ==========================
#define LCD_SDA_INDEX GPIO12
#define LCD_SCLK_INDEX GPIO13
#define LCD_RS_INDEX GPIO14
#define LCD_CS_INDEX GPIO15
#define LCD_PORT GPIOB
// LCD backlight control is special in GPIOA8
#define LCD_BG_CTRL_INDEX GPIO8
#define LCD_BG_CTRL_PORT GPIOA

// ==========================
// adapter Encoder
// ==========================
#define ENCODER_1_SW_INDEX GPIO4
#define ENCODER_1_B_INDEX GPIO3
#define ENCODER_1_PORT GPIOB
// ENC1A is special in GPIOA15
#define ENCODER_1_A_INDEX GPIO15
#define ENCODER_1_A_PORT GPIOA

#define ENCODER_2_A_INDEX GPIO7
#define ENCODER_2_B_INDEX GPIO6
#define ENCODER_2_SW_INDEX GPIO5
#define ENCODER_2_PORT GPIOB

#define ENCODER_3_A_INDEX GPIO7
#define ENCODER_3_B_INDEX GPIO6
#define ENCODER_3_SW_INDEX GPIO5
#define ENCODER_3_PORT GPIOB

void mcu_clock_init(void);
void mcu_timer_init(void);
void mcu_usart_init(void);
void mcu_i2c_init(void);
void mcu_adc_init(void);

void adapter_mcu_init(void);
void adapter_lcd_init(void);
void adapter_encoder_init(void);
