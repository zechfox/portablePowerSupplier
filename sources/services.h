
#define HMI_EVENT_QUEUE_LENGTH 16U
#define RC_EVENT_QUEUE_LENGTH 16U
#define TICK_EVENT_QUEUE_LENGTH 16U

typedef enum {
  E_DUMMY,
  E_TICK_1MS,
  E_TICK_100MS,
  E_ENC1_INC,
  E_ENC1_DEC,
  E_ENC1_CLK,
  E_ENC1_PRS,
  E_ENC1_2CLK,
  E_ENC1_3CLK,
  E_ENC2_INC,
  E_ENC2_DEC,
  E_ENC2_CLK,
  E_ENC2_PRS,
  E_ENC2_2CLK,
  E_ENC2_3CLK,
  E_ENC3_INC,
  E_ENC3_DEC,
  E_ENC3_CLK,
  E_ENC3_PRS,
  E_ENC3_2CLK,
  E_ENC3_3CLK,

  E_LAST_EVENT
} EventType;

typedef enum {
  POP,
  PUSH
} Action;

typedef struct {
  EventType eventType;

} Event;

bool hmi_event(Action action, Event *pEvent);
bool rc_event(Action action, Event *pEvent);
bool tick_event(Action action, Event *pEvent);

void run_hmi_service();
void run_rc_service();
void run_tick_service();

