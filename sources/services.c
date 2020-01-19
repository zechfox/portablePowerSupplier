#include "services.h"

Event g_hmi_event_fifo_queue[HMI_EVENT_QUEUE_LENGTH];
Event g_rc_event_fifo_queue[RC_EVENT_QUEUE_LENGTH];
Event g_tick_event_fifo_queue[TICK_EVENT_QUEUE_LENGTH];

// pop out event from g_hmi_event_fifo_queue
bool hmi_event(Action action, Event * pEvent)
{
  static uint8_t s_pop_index = 0; 
  static uint8_t s_push_index = 0;
  Event new_event;

  if (POP == action)
  {
    if ( ( HMI_EVENT_QUEUE_LENGTH > (s_push_index - s_pop_index) )
      && ( (s_push_index - s_pop_index) > 0 ) )
    {
      pEvent = g_hmi_event_fifo_queue[s_pop_index & HMI_EVENT_QUEUE_LENGTH];
      s_pop_index++;
    }
    // * is event, - is empty.
    // 
    // pop : ------------....***************
    // push : *--------...-----------
    else if ( (255 - HMI_EVENT_QUEUE_LENGTH) < (s_pop_index - s_push_index) )
    {
      // push index was wrapped
      pEvent = g_hmi_event_fifo_queue[s_pop_index & HMI_EVENT_QUEUE_LENGTH];
      s_pop_index++;
    }
    else
    {
      // queue was wrapped
      // error trace
      return false;
    }
  } // end of POP action
  // PUSH action
  else
  {
    g_hmi_event_fifo_queue[s_push_index] = *pEvent;
    s_push_index++;
  } // end of PUSH action

  return true;
}

bool rc_event(Action action, Event * pEvent)
{
  static uint8_t s_pop_index = 0; 
  static uint8_t s_push_index = 0;
  Event new_event;

  if (POP == action)
  {
    if ( ( RC_EVENT_QUEUE_LENGTH > (s_push_index - s_pop_index) )
      && ( (s_push_index - s_pop_index) > 0 ) )
    {
      pEvent = g_rc_event_fifo_queue[s_pop_index & RC_EVENT_QUEUE_LENGTH];
      s_pop_index++;
    }
    // * is event, - is empty.
    // 
    // pop : ------------....***************
    // push : *--------...-----------
    else if ( (255 - RC_EVENT_QUEUE_LENGTH) < (s_pop_index - s_push_index) )
    {
      // push index was wrapped
      pEvent = g_rc_event_fifo_queue[s_pop_index & RC_EVENT_QUEUE_LENGTH];
      s_pop_index++;
    }
    else
    {
      // queue was wrapped
      // error trace
      return false;
    }
  } // end of POP action
  // PUSH action
  else
  {
    g_rc_event_fifo_queue[s_push_index] = *pEvent;
    s_push_index++;
  } // end of PUSH action

  return true;
}

bool tick_event(Action action, Event * pEvent)
{
  static uint8_t s_pop_index = 0; 
  static uint8_t s_push_index = 0;
  Event new_event;

  if (POP == action)
  {
    if ( ( TICK_EVENT_QUEUE_LENGTH > (s_push_index - s_pop_index) )
      && ( (s_push_index - s_pop_index) > 0 ) )
    {
      pEvent = g_tick_event_fifo_queue[s_pop_index & TICK_EVENT_QUEUE_LENGTH];
      s_pop_index++;
    }

    // * is event, - is empty.
    // 
    // pop : ------------....***************
    // push : *--------...-----------
    else if ( (255 - TICK_EVENT_QUEUE_LENGTH) < (s_pop_index - s_push_index) )
    {
      // push index was wrapped
      pEvent = g_tick_event_fifo_queue[s_pop_index & TICK_EVENT_QUEUE_LENGTH];
      s_pop_index++;
    }
    else
    {
      // queue was wrapped
      // error trace
      return false;
    }
  } // end of POP action
  // PUSH action
  else
  {
    g_tick_event_fifo_queue[s_push_index++] = *pEvent;
  } // end of PUSH action

  return true;
}

// human machine interface service
void run_hmi_service()
{
  Event * event_ptr = NULL; 

  if(pop_hmi_event(event_ptr))
  {

  }
  return;
}

// remote control service
void run_rc_service()
{

  return;
}

// timer service
void run_tick_service()
{

  return;
}
