#include <libopencm3/stm32/rcc.h>
#include <libopencm3/stm32/gpio.h>
#include <libopencm3/stm32/usart.h>

#include "adapter_common.h"

static void mcu_clock_init(void)
{
  rcc_clock_setup_in_hse_8mhz_out_48mhz();
}

static void mcu_usart_init(void)
{
  // Enable clocks for USART1.
  rcc_periph_clock_enable(RCC_USART1);
  rcc_periph_clock_enable(RCC_GPIOA);

  // Enable the USART1 interrupt vector.
  nvic_enable_irq(NVIC_USART1_IRQ);
  // Setup GPIO pin GPIO_USART1_TX/GPIOA2 GPIO_USART1_RX/GPIOA3 on GPIO port A for tx/rx.
  gpio_mode_setup(MCU_USART1_PORT, GPIO_MODE_AF, GPIO_PUPD_NONE, MCU_USART1_TX_INDEX | MCU_USART1_RX_INDEX);
  gpio_set_af(GPIOA, GPIO_AF1, MCU_USART1_TX_INDEX | MCU_USART1_RX_INDEX);

  // Setup USART1 parameters.
  // baud rate 115200,
  // data bits 8,
  // parity none,
  // stop bits 1,
  // flow control none.
  usart_set_baudrate(USART1, 115200);
  usart_set_databits(USART1, 8);
  usart_set_parity(USART1, USART_PARITY_NONE);
  usart_set_stopbits(USART1, USART_CR2_STOPBITS_1);
  usart_set_mode(USART1, USART_MODE_TX_RX);
  usart_set_flow_control(USART1, USART_FLOWCONTROL_NONE);

  // enable USART1 rx interrupt
  usart_enable_rx_interrupt(USART1);
  /* Finally enable the USART. */
  usart_enable(USART1);
}

void usart1_isr(void)
{

}

static void mcu_gpio_init(void)
{

  // channel1 converstion pin
  // channel1 clim pin
  // channel2 clim pin
  // channel2 boost enable pin
  // channel3 ctrl pin
  // channle3 enable pin
  // Setup GPIO pins for USART2 transmit. */
  gpio_mode_setup(GPIOA, GPIO_MODE_AF, GPIO_PUPD_NONE, GPIO9);

  /* Setup USART1 TX pin as alternate function. */
  gpio_set_af(GPIOA, GPIO_AF1, GPIO9);
}

void mcu_i2c_init()
{

}

void mcu_usart_init()
{

}

void mcu_adc_init()
{

}

void mcu_timer_init()
{

}

void adapter_mcu_init()
{
  mcu_timer_init();
  mcu_usart_init();
  mcu_i2c_init();
  mcu_adc_init();
}
