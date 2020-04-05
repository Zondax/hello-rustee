#pragma once

#include <stdint.h>

extern uint8_t ta_version(void);

extern uint8_t ta_sign(uint8_t *message_ptr, uint32_t message_len,
                       uint8_t *signature_ptr, uint32_t signature_len);
