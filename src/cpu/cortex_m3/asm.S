.syntax unified
.cpu cortex-m3
.arch armv7-m
.text

#define THUMB_FUNC(NAME)   \
.section .text.##NAME; \
.align 2; \
.global NAME; \
.thumb; \
.thumb_func; \
.type NAME, %function; \
NAME:

THUMB_FUNC(systick_handler)
  //mrs r0, psp
  //stmdb r0!, {r4-r11}
  //msr psp, r0

  //bl set_current_task_stack
  //bl handle_exception
  //bl reschedule
  bl tick

  // r0 is the value of the next task psp returned by reschedule()
  //ldmfd r0!, {r4-r11}
  //msr psp, r0

  ldr lr, =0xfffffff9  /* return to thread mode using main SP */
  bx lr

THUMB_FUNC(pendsv_handler)
  //mrs r0, psp
  //ldmfd r0!, {r4-r11}
  //msr psp, r0

  ldr r0, =0xfffffffd  /* return to thread mode using PSP */
  bx r0

