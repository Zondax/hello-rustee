#pragma once

#include <stdint.h>

extern uint8_t ta_version(void);

extern uint8_t ta_sign(uint8_t *message_ptr, uint32_t message_len,
                       uint8_t *signature_ptr, uint32_t signature_len);

extern uint32_t invoke_command(uint32_t cmd_id, uint32_t param_types, TEE_Param params[4]);
