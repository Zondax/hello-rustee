#include "appmain.h"
#include <err.h>
#include <stdio.h>
#include <string.h>

#include <librustee_host.h>
#include <rustee_ta.h>

TEEC_Result call_rustee(TEEC_Session *sess) {
    TEEC_Result res;
    uint32_t err_origin;

    TEEC_Operation op;
    memset(&op, 0, sizeof(op));
    op.paramTypes = TEEC_PARAM_TYPES(TEEC_VALUE_INOUT, TEEC_NONE, TEEC_NONE, TEEC_NONE);

    // TODO: The whole function should be in rust, for now we just get a value
    op.params[0].value.a = host_test();

    printf("[RUSTEE] <= %d\n", op.params[0].value.a);

    res = TEEC_InvokeCommand(sess, TA_FUNCID_CALL_RUSTEE, &op, &err_origin);
    if (res != TEEC_SUCCESS) {
        errx(1, "TEEC_InvokeCommand failed. [Code 0x%x origin 0x%x]", res, err_origin);
    }

    printf("[RUSTEE] => %d\n", op.params[0].value.a);

    return res;
}

void appMain(TEEC_Session *sess) {
    // TODO: Eventually this should be a rust function
    // First we need to expose types and some functions such as TEEC_InvokeCommand

    call_rustee(sess);
}
