#include "services.h"
#include "adapters/adapter_common.h"

void init_adapters(void);
void run_server(void);

void init_adapters(void)
{
  adapter_mcu_init();
  adapter_encoder_init();
  adapter_lcd_init();
}

void run_server(void)
{
  while(1)
  {
    run_hmi_service();
    run_rc_service();
  }

  return;
}

int main(void)
{
  init_adapters();

  run_server();

  return 0;
}
