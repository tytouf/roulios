//===-- aeabi_memcpy.S - EABI memcpy implementation -----------------------===//
//
//                     The LLVM Compiler Infrastructure
//
// This file is dual licensed under the MIT and the University of Illinois Open
// Source Licenses. See LICENSE.TXT for details.
//
//===----------------------------------------------------------------------===//

#include "memcpy-armv7m.S"

#include "../assembly.h"

//  void __aeabi_memcpy(void *dest, void *src, size_t n) { memcpy(dest, src, n); }
DEFINE_AEABI_FUNCTION_ALIAS(__aeabi_memcpy, memcpy)
DEFINE_AEABI_FUNCTION_ALIAS(__aeabi_memcpy4, memcpy)
DEFINE_AEABI_FUNCTION_ALIAS(__aeabi_memcpy8, memcpy)
