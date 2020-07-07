#include "libopencm3/stm32/rcc.h"
#include "libopencm3/stm32/gpio.h"

#include "adapter_common.h"

void adapter_lcd_init()
{
  // Enable GPIOA|GPIOB clock.
  rcc_periph_clock_enable(RCC_GPIOA);
  rcc_periph_clock_enable(RCC_GPIOB);

  // GPIOA8 LCD Backlight control
  gpio_mode_setup(LCD_BG_CTRL_PORT, GPIO_MODE_OUTPUT, GPIO_PUPD_NONE, LCD_BG_CTRL_INDEX);
}
