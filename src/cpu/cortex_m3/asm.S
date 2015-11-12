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
  mrs r0, psp
  stmdb r0!, {r4-r11}
  msr psp, r0

  // Kernel state is stored in one global variable
  // gets a pointer to it and pass it to handle_exception
  ldr r1, =kernel_state
  ldr r1, [r1]
  bl handle_exception

  // r0 is the value of the next task psp returned by reschedule()
  ldmfd r0!, {r4-r11}
  msr psp, r0

  ldr lr, =0xfffffffd  /* return to thread mode using process SP */
  bx lr

THUMB_FUNC(svc_handler)
  mrs r0, psp
  stmdb r0!, {r4-r11}
  msr psp, r0

  // Kernel state is stored in one global variable
  // gets a pointer to it and pass it to handle_exception
  ldr r1, =kernel_state
  ldr r1, [r1]
  bl handle_syscall

  // r0 is the value of the next task psp returned by reschedule()
  ldmfd r0!, {r4-r11}
  msr psp, r0

  ldr lr, =0xfffffffd  /* return to thread mode using process SP */
  bx lr

THUMB_FUNC(pendsv_handler)
  mrs r0, psp
  ldmfd r0!, {r4-r11}
  msr psp, r0

  ldr r0, =0xfffffffd  /* return to thread mode using process SP */
  bx r0

