#include "services.h"
#include "adapter_common.h"

void init_adapters()
{
  adapter_mcu_init();
  adapter_encoder_init();
  adapter_lcd_init();
}

void run_server()
{
  while(1)
  {
    run_hmi_service();
    run_rc_service();
  }

  return;
}

void main()
{
  init_adapters();

  run_server();

  return;
}
