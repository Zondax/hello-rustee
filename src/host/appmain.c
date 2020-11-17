#include "appmain.h"
#include <err.h>
#include <stdio.h>
#include <string.h>

#include <librustee_host.h>
#include <rustee_ta.h>

// The optee session to be used along the program execution
TEEC_Session *session = NULL;

TEEC_Result call_rustee(TEEC_Session *sess) {
    session = sess;

    TEEC_Result res;
    uint32_t err_origin;

    TEEC_Operation op;
    memset(&op, 0, sizeof(op));
    op.paramTypes = TEEC_PARAM_TYPES(TEEC_NONE, TEEC_NONE, TEEC_NONE, TEEC_NONE);

    printf("[RUSTEE] <= %d\n", op.params[0].value.a);

    res = TEEC_InvokeCommand(session, TA_FUNCID_SIGN, &op, &err_origin);
    if (res != TEEC_SUCCESS) {
        errx(1, "TEEC_InvokeCommand failed. [Code 0x%x origin 0x%x]", res, err_origin);
    }

    printf("[RUSTEE] => %d\n", op.params[0].value.a);
    printf("running server\n");
    run();
    return TEEC_SUCCESS;
}

TEEC_Result invoke_optee_command(uint32_t command_id, TEEC_Operation *op) {
    uint32_t err_origin = 0;
    return TEEC_InvokeCommand(session, command_id, op, &err_origin);
} 

void appMain(TEEC_Session *sess) {
    // TODO: Eventually this should be a rust function
    // First we need to expose types and some functions such as TEEC_InvokeCommand

    call_rustee(sess);
}
