#pragma once

#include <rustee_ta.h>

#define TA_VERSION              "1.0"
#define TA_DESCRIPTION          "Hi RusTEE"
#define TA_FLAGS                TA_FLAG_EXEC_DDR
#define TA_STACK_SIZE           (2 * 1024)
#define TA_DATA_SIZE            (32 * 1024)

// Extra properties
#define TA_CURRENT_TA_EXT_PROPERTIES \
    { "ch.zondax.rustee.prop1", USER_TA_PROP_TYPE_STRING, "Prop1Value" }, \
    { "ch.zondax.rustee.prop2", USER_TA_PROP_TYPE_U32, &(const uint32_t){ 0x0010 } \
}
