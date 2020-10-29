#include <tee_internal_api.h>
#include <tee_internal_api_extensions.h>

#include <librustee_ta.h>
#include <rustee_ta.h>

#define UNUSED(X) (void)&X;

// TODO: Do this directly in Rust
TEE_Result TA_OpenSessionEntryPoint(uint32_t param_types,
                                    TEE_Param params[4],
                                    void **sess_ctx) {
    UNUSED(params)
    UNUSED(sess_ctx)
    // TODO: Redirect full call to rust library

    uint32_t exp_param_types = TEE_PARAM_TYPES(
            TEE_PARAM_TYPE_NONE,
            TEE_PARAM_TYPE_NONE,
            TEE_PARAM_TYPE_NONE,
            TEE_PARAM_TYPE_NONE);

    if (param_types != exp_param_types) {
        return TEE_ERROR_BAD_PARAMETERS;
    }

    return TEE_SUCCESS;
}

// TODO: Redirect full call to rust library
TEE_Result TA_InvokeCommandEntryPoint(void *sess_ctx,
                                      uint32_t cmd_id,
                                      uint32_t param_types,
                                      TEE_Param params[4]) {
    UNUSED(sess_ctx)

    return invoke_command(cmd_id, param_types, params);

    /*switch (cmd_id) {
        case TA_FUNCID_VERSION: {
            // TODO: Do this directly in Rust
            uint32_t exp_param_types = TEE_PARAM_TYPES(TEE_PARAM_TYPE_VALUE_INOUT,
                                                       TEE_PARAM_TYPE_NONE,
                                                       TEE_PARAM_TYPE_NONE,
                                                       TEE_PARAM_TYPE_NONE);
            if (param_types != exp_param_types) {
                return TEE_ERROR_BAD_PARAMETERS;
            }

            IMSG("[RUSTEE-BEFORE] : %u", params[0].value.a);

            // Here we call rust
            params[0].value.a += ta_version();

            IMSG("[RUSTEE-AFTER ] : %u", params[0].value.a);

            return TEE_SUCCESS;
        }
        case TA_FUNCID_SIGN: {
            uint32_t exp_param_types = TEE_PARAM_TYPES(TEE_PARAM_TYPE_VALUE_INOUT,
                                                       TEE_PARAM_TYPE_NONE,
                                                       TEE_PARAM_TYPE_NONE,
                                                       TEE_PARAM_TYPE_NONE);
            if (param_types != exp_param_types) {
                return TEE_ERROR_BAD_PARAMETERS;
            }

            IMSG("[RUSTEE-BEFORE] : %u", params[0].value.a);

            // Here we call rust
            uint8_t data[4] = {'h', 'o', 'l', 'a'};
            uint8_t signature[65];
            signature[64] = '\n';
            params[0].value.a += ta_sign(data, 4, signature, 64);

            IMSG("[RUSTEE-AFTER ] : %u - signature %s", params[0].value.a, signature);

            return TEE_SUCCESS;
        }
        default:
            return TEE_ERROR_BAD_PARAMETERS;
    }*/
}
