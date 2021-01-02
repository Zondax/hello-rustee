#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub const CHAR_MIN: u32 = 0;
pub const MB_LEN_MAX: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_MAX: i32 = -1;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_LEAST64_MAX: i32 = -1;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -32768;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 32767;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 65535;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const UINT_FAST64_MAX: i32 = -1;
pub const INTMAX_MAX: u64 = 9223372036854775807;
pub const INTMAX_MIN: i64 = -9223372036854775808;
pub const UINTMAX_MAX: i32 = -1;
pub const __PRI64_PREFIX: &'static [u8; 3usize] = b"ll\0";
pub const __PRIPTR_PREFIX: &'static [u8; 2usize] = b"l\0";
pub const PRId8: &'static [u8; 2usize] = b"d\0";
pub const PRId16: &'static [u8; 2usize] = b"d\0";
pub const PRId32: &'static [u8; 2usize] = b"d\0";
pub const PRId64: &'static [u8; 4usize] = b"lld\0";
pub const PRIdPTR: &'static [u8; 3usize] = b"ld\0";
pub const PRIi8: &'static [u8; 2usize] = b"i\0";
pub const PRIi16: &'static [u8; 2usize] = b"i\0";
pub const PRIi32: &'static [u8; 2usize] = b"i\0";
pub const PRIi64: &'static [u8; 4usize] = b"lli\0";
pub const PRIiPTR: &'static [u8; 3usize] = b"li\0";
pub const PRIo8: &'static [u8; 2usize] = b"o\0";
pub const PRIo16: &'static [u8; 2usize] = b"o\0";
pub const PRIo32: &'static [u8; 2usize] = b"o\0";
pub const PRIo64: &'static [u8; 4usize] = b"llo\0";
pub const PRIoPTR: &'static [u8; 3usize] = b"lo\0";
pub const PRIu8: &'static [u8; 2usize] = b"u\0";
pub const PRIu16: &'static [u8; 2usize] = b"u\0";
pub const PRIu32: &'static [u8; 2usize] = b"u\0";
pub const PRIu64: &'static [u8; 4usize] = b"llu\0";
pub const PRIuPTR: &'static [u8; 3usize] = b"lu\0";
pub const PRIx8: &'static [u8; 2usize] = b"x\0";
pub const PRIx16: &'static [u8; 2usize] = b"x\0";
pub const PRIx32: &'static [u8; 2usize] = b"x\0";
pub const PRIx64: &'static [u8; 4usize] = b"llx\0";
pub const PRIxPTR: &'static [u8; 3usize] = b"lx\0";
pub const PRIX8: &'static [u8; 2usize] = b"X\0";
pub const PRIX16: &'static [u8; 2usize] = b"X\0";
pub const PRIX32: &'static [u8; 2usize] = b"X\0";
pub const PRIX64: &'static [u8; 4usize] = b"llX\0";
pub const PRIXPTR: &'static [u8; 3usize] = b"lX\0";
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const TEE_INT_CORE_API_SPEC_VERSION: u32 = 10;
pub const TEE_HANDLE_NULL: u32 = 0;
pub const TEE_TIMEOUT_INFINITE: u32 = 4294967295;
pub const TEE_SUCCESS: u32 = 0;
pub const TEE_ERROR_CORRUPT_OBJECT: u32 = 4027580417;
pub const TEE_ERROR_CORRUPT_OBJECT_2: u32 = 4027580418;
pub const TEE_ERROR_STORAGE_NOT_AVAILABLE: u32 = 4027580419;
pub const TEE_ERROR_STORAGE_NOT_AVAILABLE_2: u32 = 4027580420;
pub const TEE_ERROR_CIPHERTEXT_INVALID: u32 = 4027580422;
pub const TEE_ERROR_GENERIC: u32 = 4294901760;
pub const TEE_ERROR_ACCESS_DENIED: u32 = 4294901761;
pub const TEE_ERROR_CANCEL: u32 = 4294901762;
pub const TEE_ERROR_ACCESS_CONFLICT: u32 = 4294901763;
pub const TEE_ERROR_EXCESS_DATA: u32 = 4294901764;
pub const TEE_ERROR_BAD_FORMAT: u32 = 4294901765;
pub const TEE_ERROR_BAD_PARAMETERS: u32 = 4294901766;
pub const TEE_ERROR_BAD_STATE: u32 = 4294901767;
pub const TEE_ERROR_ITEM_NOT_FOUND: u32 = 4294901768;
pub const TEE_ERROR_NOT_IMPLEMENTED: u32 = 4294901769;
pub const TEE_ERROR_NOT_SUPPORTED: u32 = 4294901770;
pub const TEE_ERROR_NO_DATA: u32 = 4294901771;
pub const TEE_ERROR_OUT_OF_MEMORY: u32 = 4294901772;
pub const TEE_ERROR_BUSY: u32 = 4294901773;
pub const TEE_ERROR_COMMUNICATION: u32 = 4294901774;
pub const TEE_ERROR_SECURITY: u32 = 4294901775;
pub const TEE_ERROR_SHORT_BUFFER: u32 = 4294901776;
pub const TEE_ERROR_EXTERNAL_CANCEL: u32 = 4294901777;
pub const TEE_ERROR_OVERFLOW: u32 = 4294914063;
pub const TEE_ERROR_TARGET_DEAD: u32 = 4294914084;
pub const TEE_ERROR_STORAGE_NO_SPACE: u32 = 4294914113;
pub const TEE_ERROR_MAC_INVALID: u32 = 4294914161;
pub const TEE_ERROR_SIGNATURE_INVALID: u32 = 4294914162;
pub const TEE_ERROR_TIME_NOT_SET: u32 = 4294922240;
pub const TEE_ERROR_TIME_NEEDS_RESET: u32 = 4294922241;
pub const TEE_PARAM_TYPE_NONE: u32 = 0;
pub const TEE_PARAM_TYPE_VALUE_INPUT: u32 = 1;
pub const TEE_PARAM_TYPE_VALUE_OUTPUT: u32 = 2;
pub const TEE_PARAM_TYPE_VALUE_INOUT: u32 = 3;
pub const TEE_PARAM_TYPE_MEMREF_INPUT: u32 = 5;
pub const TEE_PARAM_TYPE_MEMREF_OUTPUT: u32 = 6;
pub const TEE_PARAM_TYPE_MEMREF_INOUT: u32 = 7;
pub const TEE_LOGIN_PUBLIC: u32 = 0;
pub const TEE_LOGIN_USER: u32 = 1;
pub const TEE_LOGIN_GROUP: u32 = 2;
pub const TEE_LOGIN_APPLICATION: u32 = 4;
pub const TEE_LOGIN_APPLICATION_USER: u32 = 5;
pub const TEE_LOGIN_APPLICATION_GROUP: u32 = 6;
pub const TEE_LOGIN_TRUSTED_APP: u32 = 4026531840;
pub const TEE_ORIGIN_API: u32 = 1;
pub const TEE_ORIGIN_COMMS: u32 = 2;
pub const TEE_ORIGIN_TEE: u32 = 3;
pub const TEE_ORIGIN_TRUSTED_APP: u32 = 4;
pub const TEE_MEMORY_ACCESS_READ: u32 = 1;
pub const TEE_MEMORY_ACCESS_WRITE: u32 = 2;
pub const TEE_MEMORY_ACCESS_ANY_OWNER: u32 = 4;
pub const TEE_MALLOC_FILL_ZERO: u32 = 0;
pub const TEE_STORAGE_PRIVATE: u32 = 1;
pub const TEE_DATA_FLAG_ACCESS_READ: u32 = 1;
pub const TEE_DATA_FLAG_ACCESS_WRITE: u32 = 2;
pub const TEE_DATA_FLAG_ACCESS_WRITE_META: u32 = 4;
pub const TEE_DATA_FLAG_SHARE_READ: u32 = 16;
pub const TEE_DATA_FLAG_SHARE_WRITE: u32 = 32;
pub const TEE_DATA_FLAG_OVERWRITE: u32 = 1024;
pub const TEE_DATA_MAX_POSITION: u32 = 4294967295;
pub const TEE_OBJECT_ID_MAX_LEN: u32 = 64;
pub const TEE_USAGE_EXTRACTABLE: u32 = 1;
pub const TEE_USAGE_ENCRYPT: u32 = 2;
pub const TEE_USAGE_DECRYPT: u32 = 4;
pub const TEE_USAGE_MAC: u32 = 8;
pub const TEE_USAGE_SIGN: u32 = 16;
pub const TEE_USAGE_VERIFY: u32 = 32;
pub const TEE_USAGE_DERIVE: u32 = 64;
pub const TEE_HANDLE_FLAG_PERSISTENT: u32 = 65536;
pub const TEE_HANDLE_FLAG_INITIALIZED: u32 = 131072;
pub const TEE_HANDLE_FLAG_KEY_SET: u32 = 262144;
pub const TEE_HANDLE_FLAG_EXPECT_TWO_KEYS: u32 = 524288;
pub const TEE_OPERATION_CIPHER: u32 = 1;
pub const TEE_OPERATION_MAC: u32 = 3;
pub const TEE_OPERATION_AE: u32 = 4;
pub const TEE_OPERATION_DIGEST: u32 = 5;
pub const TEE_OPERATION_ASYMMETRIC_CIPHER: u32 = 6;
pub const TEE_OPERATION_ASYMMETRIC_SIGNATURE: u32 = 7;
pub const TEE_OPERATION_KEY_DERIVATION: u32 = 8;
pub const TEE_OPERATION_EXTENSION: u32 = 15;
pub const TEE_OPERATION_STATE_INITIAL: u32 = 0;
pub const TEE_OPERATION_STATE_ACTIVE: u32 = 1;
pub const TEE_ALG_AES_ECB_NOPAD: u32 = 268435472;
pub const TEE_ALG_AES_CBC_NOPAD: u32 = 268435728;
pub const TEE_ALG_AES_CTR: u32 = 268435984;
pub const TEE_ALG_AES_CTS: u32 = 268436240;
pub const TEE_ALG_AES_XTS: u32 = 268436496;
pub const TEE_ALG_AES_CBC_MAC_NOPAD: u32 = 805306640;
pub const TEE_ALG_AES_CBC_MAC_PKCS5: u32 = 805307664;
pub const TEE_ALG_AES_CMAC: u32 = 805307920;
pub const TEE_ALG_AES_CCM: u32 = 1073743632;
pub const TEE_ALG_AES_GCM: u32 = 1073743888;
pub const TEE_ALG_DES_ECB_NOPAD: u32 = 268435473;
pub const TEE_ALG_DES_CBC_NOPAD: u32 = 268435729;
pub const TEE_ALG_DES_CBC_MAC_NOPAD: u32 = 805306641;
pub const TEE_ALG_DES_CBC_MAC_PKCS5: u32 = 805307665;
pub const TEE_ALG_DES3_ECB_NOPAD: u32 = 268435475;
pub const TEE_ALG_DES3_CBC_NOPAD: u32 = 268435731;
pub const TEE_ALG_DES3_CBC_MAC_NOPAD: u32 = 805306643;
pub const TEE_ALG_DES3_CBC_MAC_PKCS5: u32 = 805307667;
pub const TEE_ALG_SM4_ECB_NOPAD: u32 = 268435476;
pub const TEE_ALG_SM4_CBC_NOPAD: u32 = 268435732;
pub const TEE_ALG_SM4_CTR: u32 = 268435988;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_MD5: u32 = 1879054384;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA1: u32 = 1879058480;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA224: u32 = 1879062576;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA256: u32 = 1879066672;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA384: u32 = 1879070768;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_SHA512: u32 = 1879074864;
pub const TEE_ALG_RSASSA_PKCS1_V1_5_MD5SHA1: u32 = 1879111728;
pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA1: u32 = 1881221424;
pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA224: u32 = 1882274096;
pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA256: u32 = 1883326768;
pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA384: u32 = 1884379440;
pub const TEE_ALG_RSASSA_PKCS1_PSS_MGF1_SHA512: u32 = 1885432112;
pub const TEE_ALG_RSAES_PKCS1_V1_5: u32 = 1610613040;
pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA1: u32 = 1612775984;
pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA224: u32 = 1613824560;
pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA256: u32 = 1614873136;
pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA384: u32 = 1615921712;
pub const TEE_ALG_RSAES_PKCS1_OAEP_MGF1_SHA512: u32 = 1616970288;
pub const TEE_ALG_RSA_NOPAD: u32 = 1610612784;
pub const TEE_ALG_DSA_SHA1: u32 = 1879056689;
pub const TEE_ALG_DSA_SHA224: u32 = 1879060785;
pub const TEE_ALG_DSA_SHA256: u32 = 1879064881;
pub const TEE_ALG_SM2_DSA_SM3: u32 = 1879072837;
pub const TEE_ALG_DH_DERIVE_SHARED_SECRET: u32 = 2147483698;
pub const TEE_ALG_SM2_KEP: u32 = 1610612805;
pub const TEE_ALG_MD5: u32 = 1342177281;
pub const TEE_ALG_SHA1: u32 = 1342177282;
pub const TEE_ALG_SHA224: u32 = 1342177283;
pub const TEE_ALG_SHA256: u32 = 1342177284;
pub const TEE_ALG_SHA384: u32 = 1342177285;
pub const TEE_ALG_SHA512: u32 = 1342177286;
pub const TEE_ALG_MD5SHA1: u32 = 1342177295;
pub const TEE_ALG_HMAC_MD5: u32 = 805306369;
pub const TEE_ALG_HMAC_SHA1: u32 = 805306370;
pub const TEE_ALG_HMAC_SHA224: u32 = 805306371;
pub const TEE_ALG_HMAC_SHA256: u32 = 805306372;
pub const TEE_ALG_HMAC_SHA384: u32 = 805306373;
pub const TEE_ALG_HMAC_SHA512: u32 = 805306374;
pub const TEE_ALG_HMAC_SM3: u32 = 805306375;
pub const TEE_ALG_ECDSA_P192: u32 = 1879052353;
pub const TEE_ALG_ECDSA_P224: u32 = 1879056449;
pub const TEE_ALG_ECDSA_P256: u32 = 1879060545;
pub const TEE_ALG_ECDSA_P384: u32 = 1879064641;
pub const TEE_ALG_ECDSA_P521: u32 = 1879068737;
pub const TEE_ALG_ECDH_P192: u32 = 2147487810;
pub const TEE_ALG_ECDH_P224: u32 = 2147491906;
pub const TEE_ALG_ECDH_P256: u32 = 2147496002;
pub const TEE_ALG_ECDH_P384: u32 = 2147500098;
pub const TEE_ALG_ECDH_P521: u32 = 2147504194;
pub const TEE_ALG_SM2_PKE: u32 = 2147483717;
pub const TEE_ALG_SM3: u32 = 1342177287;
pub const TEE_TYPE_AES: u32 = 2684354576;
pub const TEE_TYPE_DES: u32 = 2684354577;
pub const TEE_TYPE_DES3: u32 = 2684354579;
pub const TEE_TYPE_SM4: u32 = 2684354580;
pub const TEE_TYPE_HMAC_MD5: u32 = 2684354561;
pub const TEE_TYPE_HMAC_SHA1: u32 = 2684354562;
pub const TEE_TYPE_HMAC_SHA224: u32 = 2684354563;
pub const TEE_TYPE_HMAC_SHA256: u32 = 2684354564;
pub const TEE_TYPE_HMAC_SHA384: u32 = 2684354565;
pub const TEE_TYPE_HMAC_SHA512: u32 = 2684354566;
pub const TEE_TYPE_HMAC_SM3: u32 = 2684354567;
pub const TEE_TYPE_RSA_PUBLIC_KEY: u32 = 2684354608;
pub const TEE_TYPE_RSA_KEYPAIR: u32 = 2701131824;
pub const TEE_TYPE_DSA_PUBLIC_KEY: u32 = 2684354609;
pub const TEE_TYPE_DSA_KEYPAIR: u32 = 2701131825;
pub const TEE_TYPE_DH_KEYPAIR: u32 = 2701131826;
pub const TEE_TYPE_ECDSA_PUBLIC_KEY: u32 = 2684354625;
pub const TEE_TYPE_ECDSA_KEYPAIR: u32 = 2701131841;
pub const TEE_TYPE_ECDH_PUBLIC_KEY: u32 = 2684354626;
pub const TEE_TYPE_ECDH_KEYPAIR: u32 = 2701131842;
pub const TEE_TYPE_SM2_DSA_PUBLIC_KEY: u32 = 2684354629;
pub const TEE_TYPE_SM2_DSA_KEYPAIR: u32 = 2701131845;
pub const TEE_TYPE_SM2_KEP_PUBLIC_KEY: u32 = 2684354630;
pub const TEE_TYPE_SM2_KEP_KEYPAIR: u32 = 2701131846;
pub const TEE_TYPE_SM2_PKE_PUBLIC_KEY: u32 = 2684354631;
pub const TEE_TYPE_SM2_PKE_KEYPAIR: u32 = 2701131847;
pub const TEE_TYPE_GENERIC_SECRET: u32 = 2684354560;
pub const TEE_TYPE_CORRUPTED_OBJECT: u32 = 2684354750;
pub const TEE_TYPE_DATA: u32 = 2684354751;
pub const TEE_ATTR_SECRET_VALUE: u32 = 3221225472;
pub const TEE_ATTR_RSA_MODULUS: u32 = 3489661232;
pub const TEE_ATTR_RSA_PUBLIC_EXPONENT: u32 = 3489661488;
pub const TEE_ATTR_RSA_PRIVATE_EXPONENT: u32 = 3221226288;
pub const TEE_ATTR_RSA_PRIME1: u32 = 3221226544;
pub const TEE_ATTR_RSA_PRIME2: u32 = 3221226800;
pub const TEE_ATTR_RSA_EXPONENT1: u32 = 3221227056;
pub const TEE_ATTR_RSA_EXPONENT2: u32 = 3221227312;
pub const TEE_ATTR_RSA_COEFFICIENT: u32 = 3221227568;
pub const TEE_ATTR_DSA_PRIME: u32 = 3489665073;
pub const TEE_ATTR_DSA_SUBPRIME: u32 = 3489665329;
pub const TEE_ATTR_DSA_BASE: u32 = 3489665585;
pub const TEE_ATTR_DSA_PUBLIC_VALUE: u32 = 3489661233;
pub const TEE_ATTR_DSA_PRIVATE_VALUE: u32 = 3221226033;
pub const TEE_ATTR_DH_PRIME: u32 = 3489665074;
pub const TEE_ATTR_DH_SUBPRIME: u32 = 3489665330;
pub const TEE_ATTR_DH_BASE: u32 = 3489665586;
pub const TEE_ATTR_DH_X_BITS: u32 = 4026536754;
pub const TEE_ATTR_DH_PUBLIC_VALUE: u32 = 3489661234;
pub const TEE_ATTR_DH_PRIVATE_VALUE: u32 = 3221226034;
pub const TEE_ATTR_RSA_OAEP_LABEL: u32 = 3489663280;
pub const TEE_ATTR_RSA_PSS_SALT_LENGTH: u32 = 4026534448;
pub const TEE_ATTR_ECC_PUBLIC_VALUE_X: u32 = 3489661249;
pub const TEE_ATTR_ECC_PUBLIC_VALUE_Y: u32 = 3489661505;
pub const TEE_ATTR_ECC_PRIVATE_VALUE: u32 = 3221226305;
pub const TEE_ATTR_ECC_CURVE: u32 = 4026532929;
pub const TEE_ATTR_SM2_ID_INITIATOR: u32 = 3489662022;
pub const TEE_ATTR_SM2_ID_RESPONDER: u32 = 3489662278;
pub const TEE_ATTR_SM2_KEP_USER: u32 = 4026533446;
pub const TEE_ATTR_SM2_KEP_CONFIRMATION_IN: u32 = 3489662790;
pub const TEE_ATTR_SM2_KEP_CONFIRMATION_OUT: u32 = 3489663046;
pub const TEE_ATTR_ECC_EPHEMERAL_PUBLIC_VALUE_X: u32 = 3489663302;
pub const TEE_ATTR_ECC_EPHEMERAL_PUBLIC_VALUE_Y: u32 = 3489663558;
pub const TEE_ATTR_BIT_PROTECTED: u32 = 268435456;
pub const TEE_ATTR_BIT_VALUE: u32 = 536870912;
pub const TEE_ECC_CURVE_NIST_P192: u32 = 1;
pub const TEE_ECC_CURVE_NIST_P224: u32 = 2;
pub const TEE_ECC_CURVE_NIST_P256: u32 = 3;
pub const TEE_ECC_CURVE_NIST_P384: u32 = 4;
pub const TEE_ECC_CURVE_NIST_P521: u32 = 5;
pub const TEE_ECC_CURVE_SM2: u32 = 768;
pub const TEE_PANIC_ID_TA_CLOSESESSIONENTRYPOINT: u32 = 257;
pub const TEE_PANIC_ID_TA_CREATEENTRYPOINT: u32 = 258;
pub const TEE_PANIC_ID_TA_DESTROYENTRYPOINT: u32 = 259;
pub const TEE_PANIC_ID_TA_INVOKECOMMANDENTRYPOINT: u32 = 260;
pub const TEE_PANIC_ID_TA_OPENSESSIONENTRYPOINT: u32 = 261;
pub const TEE_PANIC_ID_TEE_ALLOCATEPROPERTYENUMERATOR: u32 = 513;
pub const TEE_PANIC_ID_TEE_FREEPROPERTYENUMERATOR: u32 = 514;
pub const TEE_PANIC_ID_TEE_GETNEXTPROPERTY: u32 = 515;
pub const TEE_PANIC_ID_TEE_GETPROPERTYASBINARYBLOCK: u32 = 516;
pub const TEE_PANIC_ID_TEE_GETPROPERTYASBOOL: u32 = 517;
pub const TEE_PANIC_ID_TEE_GETPROPERTYASIDENTITY: u32 = 518;
pub const TEE_PANIC_ID_TEE_GETPROPERTYASSTRING: u32 = 519;
pub const TEE_PANIC_ID_TEE_GETPROPERTYASU32: u32 = 520;
pub const TEE_PANIC_ID_TEE_GETPROPERTYASUUID: u32 = 521;
pub const TEE_PANIC_ID_TEE_GETPROPERTYNAME: u32 = 522;
pub const TEE_PANIC_ID_TEE_RESETPROPERTYENUMERATOR: u32 = 523;
pub const TEE_PANIC_ID_TEE_STARTPROPERTYENUMERATOR: u32 = 524;
pub const TEE_PANIC_ID_TEE_PANIC: u32 = 769;
pub const TEE_PANIC_ID_TEE_CLOSETASESSION: u32 = 1025;
pub const TEE_PANIC_ID_TEE_INVOKETACOMMAND: u32 = 1026;
pub const TEE_PANIC_ID_TEE_OPENTASESSION: u32 = 1027;
pub const TEE_PANIC_ID_TEE_GETCANCELLATIONFLAG: u32 = 1281;
pub const TEE_PANIC_ID_TEE_MASKCANCELLATION: u32 = 1282;
pub const TEE_PANIC_ID_TEE_UNMASKCANCELLATION: u32 = 1283;
pub const TEE_PANIC_ID_TEE_CHECKMEMORYACCESSRIGHTS: u32 = 1537;
pub const TEE_PANIC_ID_TEE_FREE: u32 = 1538;
pub const TEE_PANIC_ID_TEE_GETINSTANCEDATA: u32 = 1539;
pub const TEE_PANIC_ID_TEE_MALLOC: u32 = 1540;
pub const TEE_PANIC_ID_TEE_MEMCOMPARE: u32 = 1541;
pub const TEE_PANIC_ID_TEE_MEMFILL: u32 = 1542;
pub const TEE_PANIC_ID_TEE_MEMMOVE: u32 = 1543;
pub const TEE_PANIC_ID_TEE_REALLOC: u32 = 1544;
pub const TEE_PANIC_ID_TEE_SETINSTANCEDATA: u32 = 1545;
pub const TEE_PANIC_ID_TEE_CLOSEOBJECT: u32 = 1793;
pub const TEE_PANIC_ID_TEE_GETOBJECTBUFFERATTRIBUTE: u32 = 1794;
pub const TEE_PANIC_ID_TEE_GETOBJECTINFO: u32 = 1795;
pub const TEE_PANIC_ID_TEE_GETOBJECTVALUEATTRIBUTE: u32 = 1796;
pub const TEE_PANIC_ID_TEE_RESTRICTOBJECTUSAGE: u32 = 1797;
pub const TEE_PANIC_ID_TEE_GETOBJECTINFO1: u32 = 1798;
pub const TEE_PANIC_ID_TEE_RESTRICTOBJECTUSAGE1: u32 = 1799;
pub const TEE_PANIC_ID_TEE_ALLOCATETRANSIENTOBJECT: u32 = 2049;
pub const TEE_PANIC_ID_TEE_COPYOBJECTATTRIBUTES: u32 = 2050;
pub const TEE_PANIC_ID_TEE_FREETRANSIENTOBJECT: u32 = 2051;
pub const TEE_PANIC_ID_TEE_GENERATEKEY: u32 = 2052;
pub const TEE_PANIC_ID_TEE_INITREFATTRIBUTE: u32 = 2053;
pub const TEE_PANIC_ID_TEE_INITVALUEATTRIBUTE: u32 = 2054;
pub const TEE_PANIC_ID_TEE_POPULATETRANSIENTOBJECT: u32 = 2055;
pub const TEE_PANIC_ID_TEE_RESETTRANSIENTOBJECT: u32 = 2056;
pub const TEE_PANIC_ID_TEE_COPYOBJECTATTRIBUTES1: u32 = 2057;
pub const TEE_PANIC_ID_TEE_CLOSEANDDELETEPERSISTENTOBJECT: u32 = 2305;
pub const TEE_PANIC_ID_TEE_CREATEPERSISTENTOBJECT: u32 = 2306;
pub const TEE_PANIC_ID_TEE_OPENPERSISTENTOBJECT: u32 = 2307;
pub const TEE_PANIC_ID_TEE_RENAMEPERSISTENTOBJECT: u32 = 2308;
pub const TEE_PANIC_ID_TEE_CLOSEANDDELETEPERSISTENTOBJECT1: u32 = 2309;
pub const TEE_PANIC_ID_TEE_ALLOCATEPERSISTENTOBJECTENUMERATOR: u32 = 2561;
pub const TEE_PANIC_ID_TEE_FREEPERSISTENTOBJECTENUMERATOR: u32 = 2562;
pub const TEE_PANIC_ID_TEE_GETNEXTPERSISTENTOBJECT: u32 = 2563;
pub const TEE_PANIC_ID_TEE_RESETPERSISTENTOBJECTENUMERATOR: u32 = 2564;
pub const TEE_PANIC_ID_TEE_STARTPERSISTENTOBJECTENUMERATOR: u32 = 2565;
pub const TEE_PANIC_ID_TEE_READOBJECTDATA: u32 = 2817;
pub const TEE_PANIC_ID_TEE_SEEKOBJECTDATA: u32 = 2818;
pub const TEE_PANIC_ID_TEE_TRUNCATEOBJECTDATA: u32 = 2819;
pub const TEE_PANIC_ID_TEE_WRITEOBJECTDATA: u32 = 2820;
pub const TEE_PANIC_ID_TEE_ALLOCATEOPERATION: u32 = 3073;
pub const TEE_PANIC_ID_TEE_COPYOPERATION: u32 = 3074;
pub const TEE_PANIC_ID_TEE_FREEOPERATION: u32 = 3075;
pub const TEE_PANIC_ID_TEE_GETOPERATIONINFO: u32 = 3076;
pub const TEE_PANIC_ID_TEE_RESETOPERATION: u32 = 3077;
pub const TEE_PANIC_ID_TEE_SETOPERATIONKEY: u32 = 3078;
pub const TEE_PANIC_ID_TEE_SETOPERATIONKEY2: u32 = 3079;
pub const TEE_PANIC_ID_TEE_GETOPERATIONINFOMULTIPLE: u32 = 3080;
pub const TEE_PANIC_ID_TEE_DIGESTDOFINAL: u32 = 3329;
pub const TEE_PANIC_ID_TEE_DIGESTUPDATE: u32 = 3330;
pub const TEE_PANIC_ID_TEE_CIPHERDOFINAL: u32 = 3585;
pub const TEE_PANIC_ID_TEE_CIPHERINIT: u32 = 3586;
pub const TEE_PANIC_ID_TEE_CIPHERUPDATE: u32 = 3587;
pub const TEE_PANIC_ID_TEE_MACCOMPAREFINAL: u32 = 3841;
pub const TEE_PANIC_ID_TEE_MACCOMPUTEFINAL: u32 = 3842;
pub const TEE_PANIC_ID_TEE_MACINIT: u32 = 3843;
pub const TEE_PANIC_ID_TEE_MACUPDATE: u32 = 3844;
pub const TEE_PANIC_ID_TEE_AEDECRYPTFINAL: u32 = 4097;
pub const TEE_PANIC_ID_TEE_AEENCRYPTFINAL: u32 = 4098;
pub const TEE_PANIC_ID_TEE_AEINIT: u32 = 4099;
pub const TEE_PANIC_ID_TEE_AEUPDATE: u32 = 4100;
pub const TEE_PANIC_ID_TEE_AEUPDATEAAD: u32 = 4101;
pub const TEE_PANIC_ID_TEE_ASYMMETRICDECRYPT: u32 = 4353;
pub const TEE_PANIC_ID_TEE_ASYMMETRICENCRYPT: u32 = 4354;
pub const TEE_PANIC_ID_TEE_ASYMMETRICSIGNDIGEST: u32 = 4355;
pub const TEE_PANIC_ID_TEE_ASYMMETRICVERIFYDIGEST: u32 = 4356;
pub const TEE_PANIC_ID_TEE_DERIVEKEY: u32 = 4609;
pub const TEE_PANIC_ID_TEE_GENERATERANDOM: u32 = 4865;
pub const TEE_PANIC_ID_TEE_GETREETIME: u32 = 5121;
pub const TEE_PANIC_ID_TEE_GETSYSTEMTIME: u32 = 5122;
pub const TEE_PANIC_ID_TEE_GETTAPERSISTENTTIME: u32 = 5123;
pub const TEE_PANIC_ID_TEE_SETTAPERSISTENTTIME: u32 = 5124;
pub const TEE_PANIC_ID_TEE_WAIT: u32 = 5125;
pub const TEE_PANIC_ID_TEE_BIGINTFMMCONTEXTSIZEINU32: u32 = 5377;
pub const TEE_PANIC_ID_TEE_BIGINTFMMSIZEINU32: u32 = 5378;
pub const TEE_PANIC_ID_TEE_BIGINTINIT: u32 = 5633;
pub const TEE_PANIC_ID_TEE_BIGINTINITFMM: u32 = 5634;
pub const TEE_PANIC_ID_TEE_BIGINTINITFMMCONTEXT: u32 = 5635;
pub const TEE_PANIC_ID_TEE_BIGINTCONVERTFROMOCTETSTRING: u32 = 5889;
pub const TEE_PANIC_ID_TEE_BIGINTCONVERTFROMS32: u32 = 5890;
pub const TEE_PANIC_ID_TEE_BIGINTCONVERTTOOCTETSTRING: u32 = 5891;
pub const TEE_PANIC_ID_TEE_BIGINTCONVERTTOS32: u32 = 5892;
pub const TEE_PANIC_ID_TEE_BIGINTCMP: u32 = 6145;
pub const TEE_PANIC_ID_TEE_BIGINTCMPS32: u32 = 6146;
pub const TEE_PANIC_ID_TEE_BIGINTGETBIT: u32 = 6147;
pub const TEE_PANIC_ID_TEE_BIGINTGETBITCOUNT: u32 = 6148;
pub const TEE_PANIC_ID_TEE_BIGINTSHIFTRIGHT: u32 = 6149;
pub const TEE_PANIC_ID_TEE_BIGINTADD: u32 = 6401;
pub const TEE_PANIC_ID_TEE_BIGINTDIV: u32 = 6402;
pub const TEE_PANIC_ID_TEE_BIGINTMUL: u32 = 6403;
pub const TEE_PANIC_ID_TEE_BIGINTNEG: u32 = 6404;
pub const TEE_PANIC_ID_TEE_BIGINTSQUARE: u32 = 6405;
pub const TEE_PANIC_ID_TEE_BIGINTSUB: u32 = 6406;
pub const TEE_PANIC_ID_TEE_BIGINTADDMOD: u32 = 6657;
pub const TEE_PANIC_ID_TEE_BIGINTINVMOD: u32 = 6658;
pub const TEE_PANIC_ID_TEE_BIGINTMOD: u32 = 6659;
pub const TEE_PANIC_ID_TEE_BIGINTMULMOD: u32 = 6660;
pub const TEE_PANIC_ID_TEE_BIGINTSQUAREMOD: u32 = 6661;
pub const TEE_PANIC_ID_TEE_BIGINTSUBMOD: u32 = 6662;
pub const TEE_PANIC_ID_TEE_BIGINTCOMPUTEEXTENDEDGCD: u32 = 6913;
pub const TEE_PANIC_ID_TEE_BIGINTISPROBABLEPRIME: u32 = 6914;
pub const TEE_PANIC_ID_TEE_BIGINTRELATIVEPRIME: u32 = 6915;
pub const TEE_PANIC_ID_TEE_BIGINTCOMPUTEFMM: u32 = 7169;
pub const TEE_PANIC_ID_TEE_BIGINTCONVERTFROMFMM: u32 = 7170;
pub const TEE_PANIC_ID_TEE_BIGINTCONVERTTOFMM: u32 = 7171;
pub const TEE_NUM_PARAMS: u32 = 4;
pub const TEE_MEM_INPUT: u32 = 1;
pub const TEE_MEM_OUTPUT: u32 = 2;
pub const TEE_MEMREF_0_USED: u32 = 1;
pub const TEE_MEMREF_1_USED: u32 = 2;
pub const TEE_MEMREF_2_USED: u32 = 4;
pub const TEE_MEMREF_3_USED: u32 = 8;
pub const TEE_SE_READER_NAME_MAX: u32 = 20;
pub const TA_FLAG_USER_MODE: u32 = 0;
pub const TA_FLAG_EXEC_DDR: u32 = 0;
pub const TA_FLAG_SINGLE_INSTANCE: u32 = 4;
pub const TA_FLAG_MULTI_SESSION: u32 = 8;
pub const TA_FLAG_INSTANCE_KEEP_ALIVE: u32 = 16;
pub const TA_FLAG_SECURE_DATA_PATH: u32 = 32;
pub const TA_FLAG_REMAP_SUPPORT: u32 = 0;
pub const TA_FLAG_CACHE_MAINTENANCE: u32 = 128;
pub const TA_FLAG_CONCURRENT: u32 = 256;
pub const TA_FLAG_DEVICE_ENUM: u32 = 512;
pub const TA_PROP_STR_SINGLE_INSTANCE: &'static [u8; 22usize] = b"gpd.ta.singleInstance\0";
pub const TA_PROP_STR_MULTI_SESSION: &'static [u8; 20usize] = b"gpd.ta.multiSession\0";
pub const TA_PROP_STR_KEEP_ALIVE: &'static [u8; 25usize] = b"gpd.ta.instanceKeepAlive\0";
pub const TA_PROP_STR_DATA_SIZE: &'static [u8; 16usize] = b"gpd.ta.dataSize\0";
pub const TA_PROP_STR_STACK_SIZE: &'static [u8; 17usize] = b"gpd.ta.stackSize\0";
pub const TA_PROP_STR_VERSION: &'static [u8; 15usize] = b"gpd.ta.version\0";
pub const TA_PROP_STR_DESCRIPTION: &'static [u8; 19usize] = b"gpd.ta.description\0";
pub const TA_PROP_STR_UNSAFE_PARAM: &'static [u8; 20usize] = b"op-tee.unsafe_param\0";
pub const TA_PROP_STR_REMAP: &'static [u8; 13usize] = b"op-tee.remap\0";
pub const TA_PROP_STR_CACHE_SYNC: &'static [u8; 18usize] = b"op-tee.cache_sync\0";
pub const __GNUC_VA_LIST: u32 = 1;
pub const TRACE_MIN: u32 = 0;
pub const TRACE_ERROR: u32 = 1;
pub const TRACE_INFO: u32 = 2;
pub const TRACE_DEBUG: u32 = 3;
pub const TRACE_FLOW: u32 = 4;
pub const TRACE_MAX: u32 = 4;
pub const TRACE_PRINTF_LEVEL: u32 = 1;
pub const MAX_PRINT_SIZE: u32 = 256;
pub const MAX_FUNC_PRINT_SIZE: u32 = 32;
pub const TRACE_LEVEL: u32 = 4;
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type int_fast16_t = i16;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u16;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type size_t = libc::c_uint;
pub type wchar_t = libc::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: libc::c_longlong,
    pub __clang_max_align_nonce2: f64,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        ::core::mem::size_of::<max_align_t>(),
        16usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::core::mem::align_of::<max_align_t>(),
        8usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<max_align_t>())).__clang_max_align_nonce1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<max_align_t>())).__clang_max_align_nonce2 as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
pub type TEE_Result = u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TEE_UUID {
    pub timeLow: u32,
    pub timeMid: u16,
    pub timeHiAndVersion: u16,
    pub clockSeqAndNode: [u8; 8usize],
}
#[test]
fn bindgen_test_layout_TEE_UUID() {
    assert_eq!(
        ::core::mem::size_of::<TEE_UUID>(),
        16usize,
        concat!("Size of: ", stringify!(TEE_UUID))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_UUID>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_UUID))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_UUID>())).timeLow as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_UUID),
            "::",
            stringify!(timeLow)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_UUID>())).timeMid as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_UUID),
            "::",
            stringify!(timeMid)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_UUID>())).timeHiAndVersion as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_UUID),
            "::",
            stringify!(timeHiAndVersion)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_UUID>())).clockSeqAndNode as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_UUID),
            "::",
            stringify!(clockSeqAndNode)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TEE_Identity {
    pub login: u32,
    pub uuid: TEE_UUID,
}
#[test]
fn bindgen_test_layout_TEE_Identity() {
    assert_eq!(
        ::core::mem::size_of::<TEE_Identity>(),
        20usize,
        concat!("Size of: ", stringify!(TEE_Identity))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_Identity>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_Identity))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_Identity>())).login as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Identity),
            "::",
            stringify!(login)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_Identity>())).uuid as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Identity),
            "::",
            stringify!(uuid)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union TEE_Param {
    pub memref: TEE_Param__bindgen_ty_1,
    pub value: TEE_Param__bindgen_ty_2,
    _bindgen_union_align: [u32; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TEE_Param__bindgen_ty_1 {
    pub buffer: *mut libc::c_void,
    pub size: u32,
}
#[test]
fn bindgen_test_layout_TEE_Param__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<TEE_Param__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(TEE_Param__bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_Param__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_Param__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_Param__bindgen_ty_1>())).buffer as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Param__bindgen_ty_1),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_Param__bindgen_ty_1>())).size as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Param__bindgen_ty_1),
            "::",
            stringify!(size)
        )
    );
}
impl Default for TEE_Param__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TEE_Param__bindgen_ty_2 {
    pub a: u32,
    pub b: u32,
}
#[test]
fn bindgen_test_layout_TEE_Param__bindgen_ty_2() {
    assert_eq!(
        ::core::mem::size_of::<TEE_Param__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(TEE_Param__bindgen_ty_2))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_Param__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_Param__bindgen_ty_2))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_Param__bindgen_ty_2>())).a as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Param__bindgen_ty_2),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_Param__bindgen_ty_2>())).b as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Param__bindgen_ty_2),
            "::",
            stringify!(b)
        )
    );
}
#[test]
fn bindgen_test_layout_TEE_Param() {
    assert_eq!(
        ::core::mem::size_of::<TEE_Param>(),
        8usize,
        concat!("Size of: ", stringify!(TEE_Param))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_Param>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_Param))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_Param>())).memref as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Param),
            "::",
            stringify!(memref)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_Param>())).value as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Param),
            "::",
            stringify!(value)
        )
    );
}
impl Default for TEE_Param {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __TEE_TASessionHandle {
    _unused: [u8; 0],
}
pub type TEE_TASessionHandle = *mut __TEE_TASessionHandle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __TEE_PropSetHandle {
    _unused: [u8; 0],
}
pub type TEE_PropSetHandle = *mut __TEE_PropSetHandle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __TEE_ObjectHandle {
    _unused: [u8; 0],
}
pub type TEE_ObjectHandle = *mut __TEE_ObjectHandle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __TEE_ObjectEnumHandle {
    _unused: [u8; 0],
}
pub type TEE_ObjectEnumHandle = *mut __TEE_ObjectEnumHandle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __TEE_OperationHandle {
    _unused: [u8; 0],
}
pub type TEE_OperationHandle = *mut __TEE_OperationHandle;
pub type TEE_ObjectType = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TEE_ObjectInfo {
    pub objectType: u32,
    pub __bindgen_anon_1: TEE_ObjectInfo__bindgen_ty_1,
    pub __bindgen_anon_2: TEE_ObjectInfo__bindgen_ty_2,
    pub objectUsage: u32,
    pub dataSize: u32,
    pub dataPosition: u32,
    pub handleFlags: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union TEE_ObjectInfo__bindgen_ty_1 {
    pub keySize: u32,
    pub objectSize: u32,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_TEE_ObjectInfo__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<TEE_ObjectInfo__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(TEE_ObjectInfo__bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_ObjectInfo__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_ObjectInfo__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_ObjectInfo__bindgen_ty_1>())).keySize as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_ObjectInfo__bindgen_ty_1),
            "::",
            stringify!(keySize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_ObjectInfo__bindgen_ty_1>())).objectSize as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_ObjectInfo__bindgen_ty_1),
            "::",
            stringify!(objectSize)
        )
    );
}
impl Default for TEE_ObjectInfo__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union TEE_ObjectInfo__bindgen_ty_2 {
    pub maxKeySize: u32,
    pub maxObjectSize: u32,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_TEE_ObjectInfo__bindgen_ty_2() {
    assert_eq!(
        ::core::mem::size_of::<TEE_ObjectInfo__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(TEE_ObjectInfo__bindgen_ty_2))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_ObjectInfo__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_ObjectInfo__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_ObjectInfo__bindgen_ty_2>())).maxKeySize as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_ObjectInfo__bindgen_ty_2),
            "::",
            stringify!(maxKeySize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_ObjectInfo__bindgen_ty_2>())).maxObjectSize as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_ObjectInfo__bindgen_ty_2),
            "::",
            stringify!(maxObjectSize)
        )
    );
}
impl Default for TEE_ObjectInfo__bindgen_ty_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_TEE_ObjectInfo() {
    assert_eq!(
        ::core::mem::size_of::<TEE_ObjectInfo>(),
        28usize,
        concat!("Size of: ", stringify!(TEE_ObjectInfo))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_ObjectInfo>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_ObjectInfo))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_ObjectInfo>())).objectType as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_ObjectInfo),
            "::",
            stringify!(objectType)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_ObjectInfo>())).objectUsage as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_ObjectInfo),
            "::",
            stringify!(objectUsage)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_ObjectInfo>())).dataSize as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_ObjectInfo),
            "::",
            stringify!(dataSize)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_ObjectInfo>())).dataPosition as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_ObjectInfo),
            "::",
            stringify!(dataPosition)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_ObjectInfo>())).handleFlags as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_ObjectInfo),
            "::",
            stringify!(handleFlags)
        )
    );
}
impl Default for TEE_ObjectInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TEE_Whence_TEE_DATA_SEEK_SET: TEE_Whence = 0;
pub const TEE_Whence_TEE_DATA_SEEK_CUR: TEE_Whence = 1;
pub const TEE_Whence_TEE_DATA_SEEK_END: TEE_Whence = 2;
pub type TEE_Whence = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TEE_Attribute {
    pub attributeID: u32,
    pub content: TEE_Attribute__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union TEE_Attribute__bindgen_ty_1 {
    pub ref_: TEE_Attribute__bindgen_ty_1__bindgen_ty_1,
    pub value: TEE_Attribute__bindgen_ty_1__bindgen_ty_2,
    _bindgen_union_align: [u32; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TEE_Attribute__bindgen_ty_1__bindgen_ty_1 {
    pub buffer: *mut libc::c_void,
    pub length: u32,
}
#[test]
fn bindgen_test_layout_TEE_Attribute__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<TEE_Attribute__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(TEE_Attribute__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_Attribute__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(TEE_Attribute__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_Attribute__bindgen_ty_1__bindgen_ty_1>())).buffer
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Attribute__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_Attribute__bindgen_ty_1__bindgen_ty_1>())).length
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Attribute__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(length)
        )
    );
}
impl Default for TEE_Attribute__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TEE_Attribute__bindgen_ty_1__bindgen_ty_2 {
    pub a: u32,
    pub b: u32,
}
#[test]
fn bindgen_test_layout_TEE_Attribute__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::core::mem::size_of::<TEE_Attribute__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(TEE_Attribute__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_Attribute__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(TEE_Attribute__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_Attribute__bindgen_ty_1__bindgen_ty_2>())).a as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Attribute__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_Attribute__bindgen_ty_1__bindgen_ty_2>())).b as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Attribute__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(b)
        )
    );
}
#[test]
fn bindgen_test_layout_TEE_Attribute__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<TEE_Attribute__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(TEE_Attribute__bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_Attribute__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_Attribute__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_Attribute__bindgen_ty_1>())).ref_ as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Attribute__bindgen_ty_1),
            "::",
            stringify!(ref_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_Attribute__bindgen_ty_1>())).value as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Attribute__bindgen_ty_1),
            "::",
            stringify!(value)
        )
    );
}
impl Default for TEE_Attribute__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_TEE_Attribute() {
    assert_eq!(
        ::core::mem::size_of::<TEE_Attribute>(),
        12usize,
        concat!("Size of: ", stringify!(TEE_Attribute))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_Attribute>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_Attribute))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_Attribute>())).attributeID as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Attribute),
            "::",
            stringify!(attributeID)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_Attribute>())).content as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Attribute),
            "::",
            stringify!(content)
        )
    );
}
impl Default for TEE_Attribute {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TEE_OperationMode_TEE_MODE_ENCRYPT: TEE_OperationMode = 0;
pub const TEE_OperationMode_TEE_MODE_DECRYPT: TEE_OperationMode = 1;
pub const TEE_OperationMode_TEE_MODE_SIGN: TEE_OperationMode = 2;
pub const TEE_OperationMode_TEE_MODE_VERIFY: TEE_OperationMode = 3;
pub const TEE_OperationMode_TEE_MODE_MAC: TEE_OperationMode = 4;
pub const TEE_OperationMode_TEE_MODE_DIGEST: TEE_OperationMode = 5;
pub const TEE_OperationMode_TEE_MODE_DERIVE: TEE_OperationMode = 6;
pub type TEE_OperationMode = u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TEE_OperationInfo {
    pub algorithm: u32,
    pub operationClass: u32,
    pub mode: u32,
    pub digestLength: u32,
    pub maxKeySize: u32,
    pub keySize: u32,
    pub requiredKeyUsage: u32,
    pub handleState: u32,
}
#[test]
fn bindgen_test_layout_TEE_OperationInfo() {
    assert_eq!(
        ::core::mem::size_of::<TEE_OperationInfo>(),
        32usize,
        concat!("Size of: ", stringify!(TEE_OperationInfo))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_OperationInfo>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_OperationInfo))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_OperationInfo>())).algorithm as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfo),
            "::",
            stringify!(algorithm)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_OperationInfo>())).operationClass as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfo),
            "::",
            stringify!(operationClass)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_OperationInfo>())).mode as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfo),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_OperationInfo>())).digestLength as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfo),
            "::",
            stringify!(digestLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_OperationInfo>())).maxKeySize as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfo),
            "::",
            stringify!(maxKeySize)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_OperationInfo>())).keySize as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfo),
            "::",
            stringify!(keySize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_OperationInfo>())).requiredKeyUsage as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfo),
            "::",
            stringify!(requiredKeyUsage)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_OperationInfo>())).handleState as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfo),
            "::",
            stringify!(handleState)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TEE_OperationInfoKey {
    pub keySize: u32,
    pub requiredKeyUsage: u32,
}
#[test]
fn bindgen_test_layout_TEE_OperationInfoKey() {
    assert_eq!(
        ::core::mem::size_of::<TEE_OperationInfoKey>(),
        8usize,
        concat!("Size of: ", stringify!(TEE_OperationInfoKey))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_OperationInfoKey>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_OperationInfoKey))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_OperationInfoKey>())).keySize as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfoKey),
            "::",
            stringify!(keySize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_OperationInfoKey>())).requiredKeyUsage as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfoKey),
            "::",
            stringify!(requiredKeyUsage)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct TEE_OperationInfoMultiple {
    pub algorithm: u32,
    pub operationClass: u32,
    pub mode: u32,
    pub digestLength: u32,
    pub maxKeySize: u32,
    pub handleState: u32,
    pub operationState: u32,
    pub numberOfKeys: u32,
    pub keyInformation: __IncompleteArrayField<TEE_OperationInfoKey>,
}
#[test]
fn bindgen_test_layout_TEE_OperationInfoMultiple() {
    assert_eq!(
        ::core::mem::size_of::<TEE_OperationInfoMultiple>(),
        32usize,
        concat!("Size of: ", stringify!(TEE_OperationInfoMultiple))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_OperationInfoMultiple>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_OperationInfoMultiple))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_OperationInfoMultiple>())).algorithm as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfoMultiple),
            "::",
            stringify!(algorithm)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_OperationInfoMultiple>())).operationClass as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfoMultiple),
            "::",
            stringify!(operationClass)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_OperationInfoMultiple>())).mode as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfoMultiple),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_OperationInfoMultiple>())).digestLength as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfoMultiple),
            "::",
            stringify!(digestLength)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_OperationInfoMultiple>())).maxKeySize as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfoMultiple),
            "::",
            stringify!(maxKeySize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_OperationInfoMultiple>())).handleState as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfoMultiple),
            "::",
            stringify!(handleState)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_OperationInfoMultiple>())).operationState as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfoMultiple),
            "::",
            stringify!(operationState)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_OperationInfoMultiple>())).numberOfKeys as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfoMultiple),
            "::",
            stringify!(numberOfKeys)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_OperationInfoMultiple>())).keyInformation as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_OperationInfoMultiple),
            "::",
            stringify!(keyInformation)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TEE_Time {
    pub seconds: u32,
    pub millis: u32,
}
#[test]
fn bindgen_test_layout_TEE_Time() {
    assert_eq!(
        ::core::mem::size_of::<TEE_Time>(),
        8usize,
        concat!("Size of: ", stringify!(TEE_Time))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_Time>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_Time))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_Time>())).seconds as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Time),
            "::",
            stringify!(seconds)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_Time>())).millis as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_Time),
            "::",
            stringify!(millis)
        )
    );
}
pub type TEE_BigInt = u32;
pub type TEE_BigIntFMM = u32;
pub type TEE_BigIntFMMContext = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __TEE_SEServiceHandle {
    _unused: [u8; 0],
}
pub type TEE_SEServiceHandle = *mut __TEE_SEServiceHandle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __TEE_SEReaderHandle {
    _unused: [u8; 0],
}
pub type TEE_SEReaderHandle = *mut __TEE_SEReaderHandle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __TEE_SESessionHandle {
    _unused: [u8; 0],
}
pub type TEE_SESessionHandle = *mut __TEE_SESessionHandle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __TEE_SEChannelHandle {
    _unused: [u8; 0],
}
pub type TEE_SEChannelHandle = *mut __TEE_SEChannelHandle;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TEE_SEReaderProperties {
    pub sePresent: bool,
    pub teeOnly: bool,
    pub selectResponseEnable: bool,
}
#[test]
fn bindgen_test_layout_TEE_SEReaderProperties() {
    assert_eq!(
        ::core::mem::size_of::<TEE_SEReaderProperties>(),
        3usize,
        concat!("Size of: ", stringify!(TEE_SEReaderProperties))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_SEReaderProperties>(),
        1usize,
        concat!("Alignment of ", stringify!(TEE_SEReaderProperties))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_SEReaderProperties>())).sePresent as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_SEReaderProperties),
            "::",
            stringify!(sePresent)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_SEReaderProperties>())).teeOnly as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_SEReaderProperties),
            "::",
            stringify!(teeOnly)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<TEE_SEReaderProperties>())).selectResponseEnable as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_SEReaderProperties),
            "::",
            stringify!(selectResponseEnable)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TEE_SEAID {
    pub buffer: *mut u8,
    pub bufferLen: size_t,
}
#[test]
fn bindgen_test_layout_TEE_SEAID() {
    assert_eq!(
        ::core::mem::size_of::<TEE_SEAID>(),
        8usize,
        concat!("Size of: ", stringify!(TEE_SEAID))
    );
    assert_eq!(
        ::core::mem::align_of::<TEE_SEAID>(),
        4usize,
        concat!("Alignment of ", stringify!(TEE_SEAID))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_SEAID>())).buffer as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_SEAID),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<TEE_SEAID>())).bufferLen as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TEE_SEAID),
            "::",
            stringify!(bufferLen)
        )
    );
}
impl Default for TEE_SEAID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type TEE_ErrorOrigin = u32;
pub type TEE_Session = *mut libc::c_void;
pub const utee_time_category_UTEE_TIME_CAT_SYSTEM: utee_time_category = 0;
pub const utee_time_category_UTEE_TIME_CAT_TA_PERSISTENT: utee_time_category = 1;
pub const utee_time_category_UTEE_TIME_CAT_REE: utee_time_category = 2;
pub type utee_time_category = u32;
pub const utee_entry_func_UTEE_ENTRY_FUNC_OPEN_SESSION: utee_entry_func = 0;
pub const utee_entry_func_UTEE_ENTRY_FUNC_CLOSE_SESSION: utee_entry_func = 1;
pub const utee_entry_func_UTEE_ENTRY_FUNC_INVOKE_COMMAND: utee_entry_func = 2;
pub type utee_entry_func = u32;
pub const utee_cache_operation_TEE_CACHECLEAN: utee_cache_operation = 0;
pub const utee_cache_operation_TEE_CACHEFLUSH: utee_cache_operation = 1;
pub const utee_cache_operation_TEE_CACHEINVALIDATE: utee_cache_operation = 2;
pub type utee_cache_operation = u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct utee_params {
    pub types: u64,
    pub vals: [u64; 8usize],
}
#[test]
fn bindgen_test_layout_utee_params() {
    assert_eq!(
        ::core::mem::size_of::<utee_params>(),
        72usize,
        concat!("Size of: ", stringify!(utee_params))
    );
    assert_eq!(
        ::core::mem::align_of::<utee_params>(),
        8usize,
        concat!("Alignment of ", stringify!(utee_params))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<utee_params>())).types as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(utee_params),
            "::",
            stringify!(types)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<utee_params>())).vals as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(utee_params),
            "::",
            stringify!(vals)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct utee_attribute {
    pub a: u64,
    pub b: u64,
    pub attribute_id: u32,
}
#[test]
fn bindgen_test_layout_utee_attribute() {
    assert_eq!(
        ::core::mem::size_of::<utee_attribute>(),
        24usize,
        concat!("Size of: ", stringify!(utee_attribute))
    );
    assert_eq!(
        ::core::mem::align_of::<utee_attribute>(),
        8usize,
        concat!("Alignment of ", stringify!(utee_attribute))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<utee_attribute>())).a as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(utee_attribute),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<utee_attribute>())).b as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(utee_attribute),
            "::",
            stringify!(b)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<utee_attribute>())).attribute_id as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(utee_attribute),
            "::",
            stringify!(attribute_id)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ta_head {
    pub uuid: TEE_UUID,
    pub stack_size: u32,
    pub flags: u32,
    pub depr_entry: u64,
}
#[test]
fn bindgen_test_layout_ta_head() {
    assert_eq!(
        ::core::mem::size_of::<ta_head>(),
        32usize,
        concat!("Size of: ", stringify!(ta_head))
    );
    assert_eq!(
        ::core::mem::align_of::<ta_head>(),
        8usize,
        concat!("Alignment of ", stringify!(ta_head))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ta_head>())).uuid as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ta_head),
            "::",
            stringify!(uuid)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ta_head>())).stack_size as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ta_head),
            "::",
            stringify!(stack_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ta_head>())).flags as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(ta_head),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ta_head>())).depr_entry as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ta_head),
            "::",
            stringify!(depr_entry)
        )
    );
}
pub const user_ta_prop_type_USER_TA_PROP_TYPE_BOOL: user_ta_prop_type = 0;
pub const user_ta_prop_type_USER_TA_PROP_TYPE_U32: user_ta_prop_type = 1;
pub const user_ta_prop_type_USER_TA_PROP_TYPE_UUID: user_ta_prop_type = 2;
pub const user_ta_prop_type_USER_TA_PROP_TYPE_IDENTITY: user_ta_prop_type = 3;
pub const user_ta_prop_type_USER_TA_PROP_TYPE_STRING: user_ta_prop_type = 4;
pub const user_ta_prop_type_USER_TA_PROP_TYPE_BINARY_BLOCK: user_ta_prop_type = 5;
pub type user_ta_prop_type = u32;
pub const user_ta_core_service_id_USER_TA_CORE_ENTRY_MATH_INIT: user_ta_core_service_id = 16;
pub const user_ta_core_service_id_USER_TA_CORE_ENTRY_GARBAGE: user_ta_core_service_id = 17;
pub const user_ta_core_service_id_USER_TA_CORE_ENTRY_CLOSESESSION: user_ta_core_service_id = 18;
pub type user_ta_core_service_id = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct user_ta_property {
    pub name: *const libc::c_char,
    pub type_: user_ta_prop_type,
    pub value: *const libc::c_void,
}
#[test]
fn bindgen_test_layout_user_ta_property() {
    assert_eq!(
        ::core::mem::size_of::<user_ta_property>(),
        12usize,
        concat!("Size of: ", stringify!(user_ta_property))
    );
    assert_eq!(
        ::core::mem::align_of::<user_ta_property>(),
        4usize,
        concat!("Alignment of ", stringify!(user_ta_property))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<user_ta_property>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(user_ta_property),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<user_ta_property>())).type_ as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(user_ta_property),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<user_ta_property>())).value as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(user_ta_property),
            "::",
            stringify!(value)
        )
    );
}
impl Default for user_ta_property {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
extern "C" {
    pub static mut ta_props: [user_ta_property; 0usize];
}
extern "C" {
    pub static ta_num_props: size_t;
}
extern "C" {
    pub static mut ta_param_types: u32;
}
extern "C" {
    pub static mut ta_params: [TEE_Param; 4usize];
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ta_func_head {
    pub cmd_id: u32,
    pub start: u32,
}
#[test]
fn bindgen_test_layout_ta_func_head() {
    assert_eq!(
        ::core::mem::size_of::<ta_func_head>(),
        8usize,
        concat!("Size of: ", stringify!(ta_func_head))
    );
    assert_eq!(
        ::core::mem::align_of::<ta_func_head>(),
        4usize,
        concat!("Alignment of ", stringify!(ta_func_head))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ta_func_head>())).cmd_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ta_func_head),
            "::",
            stringify!(cmd_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ta_func_head>())).start as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ta_func_head),
            "::",
            stringify!(start)
        )
    );
}
pub type ta_func_head_t = ta_func_head;
extern "C" {
    pub fn tahead_get_trace_level() -> libc::c_int;
}
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
extern "C" {
    pub static mut trace_level: libc::c_int;
}
extern "C" {
    pub static mut trace_ext_prefix: [libc::c_char; 0usize];
}
extern "C" {
    pub fn trace_ext_puts(str: *const libc::c_char);
}
extern "C" {
    pub fn trace_ext_get_thread_id() -> libc::c_int;
}
extern "C" {
    pub fn trace_set_level(level: libc::c_int);
}
extern "C" {
    pub fn trace_get_level() -> libc::c_int;
}
extern "C" {
    pub fn plat_trace_ext_puts(str: *const libc::c_char);
}
extern "C" {
    pub fn trace_vprintf(
        func: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        level_ok: bool,
        fmt: *const libc::c_char,
        args: va_list,
    );
}
extern "C" {
    pub fn trace_printf(
        func: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        level_ok: bool,
        fmt: *const libc::c_char,
        ...
    );
}
extern "C" {
    pub fn dhex_dump(
        function: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        buf: *const libc::c_void,
        len: libc::c_int,
    );
}
extern "C" {
    pub fn TEE_GetPropertyAsString(
        propsetOrEnumerator: TEE_PropSetHandle,
        name: *const libc::c_char,
        valueBuffer: *mut libc::c_char,
        valueBufferLen: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_GetPropertyAsBool(
        propsetOrEnumerator: TEE_PropSetHandle,
        name: *const libc::c_char,
        value: *mut bool,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_GetPropertyAsU32(
        propsetOrEnumerator: TEE_PropSetHandle,
        name: *const libc::c_char,
        value: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_GetPropertyAsBinaryBlock(
        propsetOrEnumerator: TEE_PropSetHandle,
        name: *const libc::c_char,
        valueBuffer: *mut libc::c_void,
        valueBufferLen: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_GetPropertyAsUUID(
        propsetOrEnumerator: TEE_PropSetHandle,
        name: *const libc::c_char,
        value: *mut TEE_UUID,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_GetPropertyAsIdentity(
        propsetOrEnumerator: TEE_PropSetHandle,
        name: *const libc::c_char,
        value: *mut TEE_Identity,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_AllocatePropertyEnumerator(enumerator: *mut TEE_PropSetHandle) -> TEE_Result;
}
extern "C" {
    pub fn TEE_FreePropertyEnumerator(enumerator: TEE_PropSetHandle);
}
extern "C" {
    pub fn TEE_StartPropertyEnumerator(enumerator: TEE_PropSetHandle, propSet: TEE_PropSetHandle);
}
extern "C" {
    pub fn TEE_ResetPropertyEnumerator(enumerator: TEE_PropSetHandle);
}
extern "C" {
    pub fn TEE_GetPropertyName(
        enumerator: TEE_PropSetHandle,
        nameBuffer: *mut libc::c_void,
        nameBufferLen: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_GetNextProperty(enumerator: TEE_PropSetHandle) -> TEE_Result;
}
extern "C" {
    pub fn TEE_Panic(panicCode: TEE_Result);
}
extern "C" {
    pub fn TEE_OpenTASession(
        destination: *const TEE_UUID,
        cancellationRequestTimeout: u32,
        paramTypes: u32,
        params: *mut TEE_Param,
        session: *mut TEE_TASessionHandle,
        returnOrigin: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_CloseTASession(session: TEE_TASessionHandle);
}
extern "C" {
    pub fn TEE_InvokeTACommand(
        session: TEE_TASessionHandle,
        cancellationRequestTimeout: u32,
        commandID: u32,
        paramTypes: u32,
        params: *mut TEE_Param,
        returnOrigin: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_GetCancellationFlag() -> bool;
}
extern "C" {
    pub fn TEE_UnmaskCancellation() -> bool;
}
extern "C" {
    pub fn TEE_MaskCancellation() -> bool;
}
extern "C" {
    pub fn TEE_CheckMemoryAccessRights(
        accessFlags: u32,
        buffer: *mut libc::c_void,
        size: u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_SetInstanceData(instanceData: *const libc::c_void);
}
extern "C" {
    pub fn TEE_GetInstanceData() -> *const libc::c_void;
}
extern "C" {
    pub fn TEE_Malloc(size: u32, hint: u32) -> *mut libc::c_void;
}
extern "C" {
    pub fn TEE_Realloc(buffer: *mut libc::c_void, newSize: u32) -> *mut libc::c_void;
}
extern "C" {
    pub fn TEE_Free(buffer: *mut libc::c_void);
}
extern "C" {
    pub fn TEE_MemMove(
        dest: *mut libc::c_void,
        src: *const libc::c_void,
        size: u32,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn TEE_MemCompare(
        buffer1: *const libc::c_void,
        buffer2: *const libc::c_void,
        size: u32,
    ) -> i32;
}
extern "C" {
    pub fn TEE_MemFill(buff: *mut libc::c_void, x: u32, size: u32) -> *mut libc::c_void;
}
extern "C" {
    pub fn TEE_GetObjectInfo(object: TEE_ObjectHandle, objectInfo: *mut TEE_ObjectInfo);
}
extern "C" {
    pub fn TEE_GetObjectInfo1(
        object: TEE_ObjectHandle,
        objectInfo: *mut TEE_ObjectInfo,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_RestrictObjectUsage(object: TEE_ObjectHandle, objectUsage: u32);
}
extern "C" {
    pub fn TEE_RestrictObjectUsage1(object: TEE_ObjectHandle, objectUsage: u32) -> TEE_Result;
}
extern "C" {
    pub fn TEE_GetObjectBufferAttribute(
        object: TEE_ObjectHandle,
        attributeID: u32,
        buffer: *mut libc::c_void,
        size: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_GetObjectValueAttribute(
        object: TEE_ObjectHandle,
        attributeID: u32,
        a: *mut u32,
        b: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_CloseObject(object: TEE_ObjectHandle);
}
extern "C" {
    pub fn TEE_AllocateTransientObject(
        objectType: TEE_ObjectType,
        maxKeySize: u32,
        object: *mut TEE_ObjectHandle,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_FreeTransientObject(object: TEE_ObjectHandle);
}
extern "C" {
    pub fn TEE_ResetTransientObject(object: TEE_ObjectHandle);
}
extern "C" {
    pub fn TEE_PopulateTransientObject(
        object: TEE_ObjectHandle,
        attrs: *const TEE_Attribute,
        attrCount: u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_InitRefAttribute(
        attr: *mut TEE_Attribute,
        attributeID: u32,
        buffer: *const libc::c_void,
        length: u32,
    );
}
extern "C" {
    pub fn TEE_InitValueAttribute(attr: *mut TEE_Attribute, attributeID: u32, a: u32, b: u32);
}
extern "C" {
    pub fn TEE_CopyObjectAttributes(destObject: TEE_ObjectHandle, srcObject: TEE_ObjectHandle);
}
extern "C" {
    pub fn TEE_CopyObjectAttributes1(
        destObject: TEE_ObjectHandle,
        srcObject: TEE_ObjectHandle,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_GenerateKey(
        object: TEE_ObjectHandle,
        keySize: u32,
        params: *const TEE_Attribute,
        paramCount: u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_OpenPersistentObject(
        storageID: u32,
        objectID: *const libc::c_void,
        objectIDLen: u32,
        flags: u32,
        object: *mut TEE_ObjectHandle,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_CreatePersistentObject(
        storageID: u32,
        objectID: *const libc::c_void,
        objectIDLen: u32,
        flags: u32,
        attributes: TEE_ObjectHandle,
        initialData: *const libc::c_void,
        initialDataLen: u32,
        object: *mut TEE_ObjectHandle,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_CloseAndDeletePersistentObject(object: TEE_ObjectHandle);
}
extern "C" {
    pub fn TEE_CloseAndDeletePersistentObject1(object: TEE_ObjectHandle) -> TEE_Result;
}
extern "C" {
    pub fn TEE_RenamePersistentObject(
        object: TEE_ObjectHandle,
        newObjectID: *const libc::c_void,
        newObjectIDLen: u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_AllocatePersistentObjectEnumerator(
        objectEnumerator: *mut TEE_ObjectEnumHandle,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_FreePersistentObjectEnumerator(objectEnumerator: TEE_ObjectEnumHandle);
}
extern "C" {
    pub fn TEE_ResetPersistentObjectEnumerator(objectEnumerator: TEE_ObjectEnumHandle);
}
extern "C" {
    pub fn TEE_StartPersistentObjectEnumerator(
        objectEnumerator: TEE_ObjectEnumHandle,
        storageID: u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_GetNextPersistentObject(
        objectEnumerator: TEE_ObjectEnumHandle,
        objectInfo: *mut TEE_ObjectInfo,
        objectID: *mut libc::c_void,
        objectIDLen: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_ReadObjectData(
        object: TEE_ObjectHandle,
        buffer: *mut libc::c_void,
        size: u32,
        count: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_WriteObjectData(
        object: TEE_ObjectHandle,
        buffer: *const libc::c_void,
        size: u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_TruncateObjectData(object: TEE_ObjectHandle, size: u32) -> TEE_Result;
}
extern "C" {
    pub fn TEE_SeekObjectData(
        object: TEE_ObjectHandle,
        offset: i32,
        whence: TEE_Whence,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_AllocateOperation(
        operation: *mut TEE_OperationHandle,
        algorithm: u32,
        mode: u32,
        maxKeySize: u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_FreeOperation(operation: TEE_OperationHandle);
}
extern "C" {
    pub fn TEE_GetOperationInfo(
        operation: TEE_OperationHandle,
        operationInfo: *mut TEE_OperationInfo,
    );
}
extern "C" {
    pub fn TEE_GetOperationInfoMultiple(
        operation: TEE_OperationHandle,
        operationInfoMultiple: *mut TEE_OperationInfoMultiple,
        operationSize: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_ResetOperation(operation: TEE_OperationHandle);
}
extern "C" {
    pub fn TEE_SetOperationKey(operation: TEE_OperationHandle, key: TEE_ObjectHandle)
        -> TEE_Result;
}
extern "C" {
    pub fn TEE_SetOperationKey2(
        operation: TEE_OperationHandle,
        key1: TEE_ObjectHandle,
        key2: TEE_ObjectHandle,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_CopyOperation(dstOperation: TEE_OperationHandle, srcOperation: TEE_OperationHandle);
}
extern "C" {
    pub fn TEE_DigestUpdate(
        operation: TEE_OperationHandle,
        chunk: *const libc::c_void,
        chunkSize: u32,
    );
}
extern "C" {
    pub fn TEE_DigestDoFinal(
        operation: TEE_OperationHandle,
        chunk: *const libc::c_void,
        chunkLen: u32,
        hash: *mut libc::c_void,
        hashLen: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_CipherInit(operation: TEE_OperationHandle, IV: *const libc::c_void, IVLen: u32);
}
extern "C" {
    pub fn TEE_CipherUpdate(
        operation: TEE_OperationHandle,
        srcData: *const libc::c_void,
        srcLen: u32,
        destData: *mut libc::c_void,
        destLen: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_CipherDoFinal(
        operation: TEE_OperationHandle,
        srcData: *const libc::c_void,
        srcLen: u32,
        destData: *mut libc::c_void,
        destLen: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_MACInit(operation: TEE_OperationHandle, IV: *const libc::c_void, IVLen: u32);
}
extern "C" {
    pub fn TEE_MACUpdate(
        operation: TEE_OperationHandle,
        chunk: *const libc::c_void,
        chunkSize: u32,
    );
}
extern "C" {
    pub fn TEE_MACComputeFinal(
        operation: TEE_OperationHandle,
        message: *const libc::c_void,
        messageLen: u32,
        mac: *mut libc::c_void,
        macLen: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_MACCompareFinal(
        operation: TEE_OperationHandle,
        message: *const libc::c_void,
        messageLen: u32,
        mac: *const libc::c_void,
        macLen: u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_AEInit(
        operation: TEE_OperationHandle,
        nonce: *const libc::c_void,
        nonceLen: u32,
        tagLen: u32,
        AADLen: u32,
        payloadLen: u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_AEUpdateAAD(
        operation: TEE_OperationHandle,
        AADdata: *const libc::c_void,
        AADdataLen: u32,
    );
}
extern "C" {
    pub fn TEE_AEUpdate(
        operation: TEE_OperationHandle,
        srcData: *const libc::c_void,
        srcLen: u32,
        destData: *mut libc::c_void,
        destLen: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_AEEncryptFinal(
        operation: TEE_OperationHandle,
        srcData: *const libc::c_void,
        srcLen: u32,
        destData: *mut libc::c_void,
        destLen: *mut u32,
        tag: *mut libc::c_void,
        tagLen: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_AEDecryptFinal(
        operation: TEE_OperationHandle,
        srcData: *const libc::c_void,
        srcLen: u32,
        destData: *mut libc::c_void,
        destLen: *mut u32,
        tag: *mut libc::c_void,
        tagLen: u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_AsymmetricEncrypt(
        operation: TEE_OperationHandle,
        params: *const TEE_Attribute,
        paramCount: u32,
        srcData: *const libc::c_void,
        srcLen: u32,
        destData: *mut libc::c_void,
        destLen: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_AsymmetricDecrypt(
        operation: TEE_OperationHandle,
        params: *const TEE_Attribute,
        paramCount: u32,
        srcData: *const libc::c_void,
        srcLen: u32,
        destData: *mut libc::c_void,
        destLen: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_AsymmetricSignDigest(
        operation: TEE_OperationHandle,
        params: *const TEE_Attribute,
        paramCount: u32,
        digest: *const libc::c_void,
        digestLen: u32,
        signature: *mut libc::c_void,
        signatureLen: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_AsymmetricVerifyDigest(
        operation: TEE_OperationHandle,
        params: *const TEE_Attribute,
        paramCount: u32,
        digest: *const libc::c_void,
        digestLen: u32,
        signature: *const libc::c_void,
        signatureLen: u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_DeriveKey(
        operation: TEE_OperationHandle,
        params: *const TEE_Attribute,
        paramCount: u32,
        derivedKey: TEE_ObjectHandle,
    );
}
extern "C" {
    pub fn TEE_GenerateRandom(randomBuffer: *mut libc::c_void, randomBufferLen: u32);
}
extern "C" {
    pub fn TEE_GetSystemTime(time: *mut TEE_Time);
}
extern "C" {
    pub fn TEE_Wait(timeout: u32) -> TEE_Result;
}
extern "C" {
    pub fn TEE_GetTAPersistentTime(time: *mut TEE_Time) -> TEE_Result;
}
extern "C" {
    pub fn TEE_SetTAPersistentTime(time: *const TEE_Time) -> TEE_Result;
}
extern "C" {
    pub fn TEE_GetREETime(time: *mut TEE_Time);
}
extern "C" {
    pub fn TEE_BigIntFMMSizeInU32(modulusSizeInBits: u32) -> u32;
}
extern "C" {
    pub fn TEE_BigIntFMMContextSizeInU32(modulusSizeInBits: u32) -> u32;
}
extern "C" {
    pub fn TEE_BigIntInit(bigInt: *mut TEE_BigInt, len: u32);
}
extern "C" {
    pub fn TEE_BigIntInitFMMContext(
        context: *mut TEE_BigIntFMMContext,
        len: u32,
        modulus: *const TEE_BigInt,
    );
}
extern "C" {
    pub fn TEE_BigIntInitFMM(bigIntFMM: *mut TEE_BigIntFMM, len: u32);
}
extern "C" {
    pub fn TEE_BigIntConvertFromOctetString(
        dest: *mut TEE_BigInt,
        buffer: *const u8,
        bufferLen: u32,
        sign: i32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_BigIntConvertToOctetString(
        buffer: *mut u8,
        bufferLen: *mut u32,
        bigInt: *const TEE_BigInt,
    ) -> TEE_Result;
}
extern "C" {
    pub fn TEE_BigIntConvertFromS32(dest: *mut TEE_BigInt, shortVal: i32);
}
extern "C" {
    pub fn TEE_BigIntConvertToS32(dest: *mut i32, src: *const TEE_BigInt) -> TEE_Result;
}
extern "C" {
    pub fn TEE_BigIntCmp(op1: *const TEE_BigInt, op2: *const TEE_BigInt) -> i32;
}
extern "C" {
    pub fn TEE_BigIntCmpS32(op: *const TEE_BigInt, shortVal: i32) -> i32;
}
extern "C" {
    pub fn TEE_BigIntShiftRight(dest: *mut TEE_BigInt, op: *const TEE_BigInt, bits: size_t);
}
extern "C" {
    pub fn TEE_BigIntGetBit(src: *const TEE_BigInt, bitIndex: u32) -> bool;
}
extern "C" {
    pub fn TEE_BigIntGetBitCount(src: *const TEE_BigInt) -> u32;
}
extern "C" {
    pub fn TEE_BigIntAdd(dest: *mut TEE_BigInt, op1: *const TEE_BigInt, op2: *const TEE_BigInt);
}
extern "C" {
    pub fn TEE_BigIntSub(dest: *mut TEE_BigInt, op1: *const TEE_BigInt, op2: *const TEE_BigInt);
}
extern "C" {
    pub fn TEE_BigIntNeg(dest: *mut TEE_BigInt, op: *const TEE_BigInt);
}
extern "C" {
    pub fn TEE_BigIntMul(dest: *mut TEE_BigInt, op1: *const TEE_BigInt, op2: *const TEE_BigInt);
}
extern "C" {
    pub fn TEE_BigIntSquare(dest: *mut TEE_BigInt, op: *const TEE_BigInt);
}
extern "C" {
    pub fn TEE_BigIntDiv(
        dest_q: *mut TEE_BigInt,
        dest_r: *mut TEE_BigInt,
        op1: *const TEE_BigInt,
        op2: *const TEE_BigInt,
    );
}
extern "C" {
    pub fn TEE_BigIntMod(dest: *mut TEE_BigInt, op: *const TEE_BigInt, n: *const TEE_BigInt);
}
extern "C" {
    pub fn TEE_BigIntAddMod(
        dest: *mut TEE_BigInt,
        op1: *const TEE_BigInt,
        op2: *const TEE_BigInt,
        n: *const TEE_BigInt,
    );
}
extern "C" {
    pub fn TEE_BigIntSubMod(
        dest: *mut TEE_BigInt,
        op1: *const TEE_BigInt,
        op2: *const TEE_BigInt,
        n: *const TEE_BigInt,
    );
}
extern "C" {
    pub fn TEE_BigIntMulMod(
        dest: *mut TEE_BigInt,
        op1: *const TEE_BigInt,
        op2: *const TEE_BigInt,
        n: *const TEE_BigInt,
    );
}
extern "C" {
    pub fn TEE_BigIntSquareMod(dest: *mut TEE_BigInt, op: *const TEE_BigInt, n: *const TEE_BigInt);
}
extern "C" {
    pub fn TEE_BigIntInvMod(dest: *mut TEE_BigInt, op: *const TEE_BigInt, n: *const TEE_BigInt);
}
extern "C" {
    pub fn TEE_BigIntRelativePrime(op1: *const TEE_BigInt, op2: *const TEE_BigInt) -> bool;
}
extern "C" {
    pub fn TEE_BigIntComputeExtendedGcd(
        gcd: *mut TEE_BigInt,
        u: *mut TEE_BigInt,
        v: *mut TEE_BigInt,
        op1: *const TEE_BigInt,
        op2: *const TEE_BigInt,
    );
}
extern "C" {
    pub fn TEE_BigIntIsProbablePrime(op: *const TEE_BigInt, confidenceLevel: u32) -> i32;
}
extern "C" {
    pub fn TEE_BigIntConvertToFMM(
        dest: *mut TEE_BigIntFMM,
        src: *const TEE_BigInt,
        n: *const TEE_BigInt,
        context: *const TEE_BigIntFMMContext,
    );
}
extern "C" {
    pub fn TEE_BigIntConvertFromFMM(
        dest: *mut TEE_BigInt,
        src: *const TEE_BigIntFMM,
        n: *const TEE_BigInt,
        context: *const TEE_BigIntFMMContext,
    );
}
extern "C" {
    pub fn TEE_BigIntFMMConvertToBigInt(
        dest: *mut TEE_BigInt,
        src: *const TEE_BigIntFMM,
        n: *const TEE_BigInt,
        context: *const TEE_BigIntFMMContext,
    );
}
extern "C" {
    pub fn TEE_BigIntComputeFMM(
        dest: *mut TEE_BigIntFMM,
        op1: *const TEE_BigIntFMM,
        op2: *const TEE_BigIntFMM,
        n: *const TEE_BigInt,
        context: *const TEE_BigIntFMMContext,
    );
}
extern "C" {
    pub fn utee_return(ret: libc::c_ulong);
}
extern "C" {
    pub fn utee_log(buf: *const libc::c_void, len: size_t);
}
extern "C" {
    pub fn utee_panic(code: libc::c_ulong);
}
extern "C" {
    pub fn utee_get_property(
        prop_set: libc::c_ulong,
        index: libc::c_ulong,
        name: *mut libc::c_void,
        name_len: *mut u32,
        buf: *mut libc::c_void,
        blen: *mut u32,
        prop_type: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_get_property_name_to_index(
        prop_set: libc::c_ulong,
        name: *const libc::c_void,
        name_len: libc::c_ulong,
        index: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_open_ta_session(
        dest: *const TEE_UUID,
        cancel_req_to: libc::c_ulong,
        params: *mut utee_params,
        sess: *mut u32,
        ret_orig: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_close_ta_session(sess: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_invoke_ta_command(
        sess: libc::c_ulong,
        cancel_req_to: libc::c_ulong,
        cmd_id: libc::c_ulong,
        params: *mut utee_params,
        ret_orig: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_check_access_rights(
        flags: u32,
        buf: *const libc::c_void,
        len: size_t,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_get_cancellation_flag(cancel: *mut u32) -> TEE_Result;
}
extern "C" {
    pub fn utee_unmask_cancellation(old_mask: *mut u32) -> TEE_Result;
}
extern "C" {
    pub fn utee_mask_cancellation(old_mask: *mut u32) -> TEE_Result;
}
extern "C" {
    pub fn utee_wait(timeout: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_get_time(cat: libc::c_ulong, time: *mut TEE_Time) -> TEE_Result;
}
extern "C" {
    pub fn utee_set_ta_time(time: *const TEE_Time) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_state_alloc(
        algo: libc::c_ulong,
        op_mode: libc::c_ulong,
        key1: libc::c_ulong,
        key2: libc::c_ulong,
        state: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_state_copy(dst: libc::c_ulong, src: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_state_free(state: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_hash_init(
        state: libc::c_ulong,
        iv: *const libc::c_void,
        iv_len: size_t,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_hash_update(
        state: libc::c_ulong,
        chunk: *const libc::c_void,
        chunk_size: size_t,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_hash_final(
        state: libc::c_ulong,
        chunk: *const libc::c_void,
        chunk_size: size_t,
        hash: *mut libc::c_void,
        hash_len: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_cipher_init(
        state: libc::c_ulong,
        iv: *const libc::c_void,
        iv_len: size_t,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_cipher_update(
        state: libc::c_ulong,
        src: *const libc::c_void,
        src_len: size_t,
        dest: *mut libc::c_void,
        dest_len: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_cipher_final(
        state: libc::c_ulong,
        src: *const libc::c_void,
        src_len: size_t,
        dest: *mut libc::c_void,
        dest_len: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_obj_get_info(obj: libc::c_ulong, info: *mut TEE_ObjectInfo) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_obj_restrict_usage(obj: libc::c_ulong, usage: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_obj_get_attr(
        obj: libc::c_ulong,
        attr_id: libc::c_ulong,
        buffer: *mut libc::c_void,
        size: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_obj_alloc(
        type_: libc::c_ulong,
        max_size: libc::c_ulong,
        obj: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_obj_close(obj: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_obj_reset(obj: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_obj_populate(
        obj: libc::c_ulong,
        attrs: *mut utee_attribute,
        attr_count: libc::c_ulong,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_obj_copy(dst_obj: libc::c_ulong, src_obj: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_obj_generate_key(
        obj: libc::c_ulong,
        key_size: libc::c_ulong,
        params: *const utee_attribute,
        param_count: libc::c_ulong,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_derive_key(
        state: libc::c_ulong,
        params: *const utee_attribute,
        param_count: libc::c_ulong,
        derived_key: libc::c_ulong,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_cryp_random_number_generate(buf: *mut libc::c_void, blen: size_t) -> TEE_Result;
}
extern "C" {
    pub fn utee_authenc_init(
        state: libc::c_ulong,
        nonce: *const libc::c_void,
        nonce_len: size_t,
        tag_len: size_t,
        aad_len: size_t,
        payload_len: size_t,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_authenc_update_aad(
        state: libc::c_ulong,
        aad_data: *const libc::c_void,
        aad_data_len: size_t,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_authenc_update_payload(
        state: libc::c_ulong,
        src_data: *const libc::c_void,
        src_len: size_t,
        dest_data: *mut libc::c_void,
        dest_len: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_authenc_enc_final(
        state: libc::c_ulong,
        src_data: *const libc::c_void,
        src_len: size_t,
        dest_data: *mut libc::c_void,
        dest_len: *mut u64,
        tag: *mut libc::c_void,
        tag_len: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_authenc_dec_final(
        state: libc::c_ulong,
        src_data: *const libc::c_void,
        src_len: size_t,
        dest_data: *mut libc::c_void,
        dest_len: *mut u64,
        tag: *const libc::c_void,
        tag_len: size_t,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_asymm_operate(
        state: libc::c_ulong,
        params: *const utee_attribute,
        num_params: libc::c_ulong,
        src_data: *const libc::c_void,
        src_len: size_t,
        dest_data: *mut libc::c_void,
        dest_len: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_asymm_verify(
        state: libc::c_ulong,
        params: *const utee_attribute,
        num_params: libc::c_ulong,
        data: *const libc::c_void,
        data_len: size_t,
        sig: *const libc::c_void,
        sig_len: size_t,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_storage_obj_open(
        storage_id: libc::c_ulong,
        object_id: *const libc::c_void,
        object_id_len: size_t,
        flags: libc::c_ulong,
        obj: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_storage_obj_create(
        storage_id: libc::c_ulong,
        object_id: *const libc::c_void,
        object_id_len: size_t,
        flags: libc::c_ulong,
        attr: libc::c_ulong,
        data: *const libc::c_void,
        len: size_t,
        obj: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_storage_obj_del(obj: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_storage_obj_rename(
        obj: libc::c_ulong,
        new_obj_id: *const libc::c_void,
        new_obj_id_len: size_t,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_storage_alloc_enum(obj_enum: *mut u32) -> TEE_Result;
}
extern "C" {
    pub fn utee_storage_free_enum(obj_enum: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_storage_reset_enum(obj_enum: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_storage_start_enum(
        obj_enum: libc::c_ulong,
        storage_id: libc::c_ulong,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_storage_next_enum(
        obj_enum: libc::c_ulong,
        info: *mut TEE_ObjectInfo,
        obj_id: *mut libc::c_void,
        len: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_storage_obj_read(
        obj: libc::c_ulong,
        data: *mut libc::c_void,
        len: size_t,
        count: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_storage_obj_write(
        obj: libc::c_ulong,
        data: *const libc::c_void,
        len: size_t,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_storage_obj_trunc(obj: libc::c_ulong, len: size_t) -> TEE_Result;
}
extern "C" {
    pub fn utee_storage_obj_seek(
        obj: libc::c_ulong,
        offset: i32,
        whence: libc::c_ulong,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_service_open(seServiceHandle: *mut u32) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_service_close(seServiceHandle: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_service_get_readers(
        seServiceHandle: libc::c_ulong,
        r: *mut u32,
        len: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_reader_get_prop(r: libc::c_ulong, p: *mut u32) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_reader_get_name(
        r: libc::c_ulong,
        name: *mut libc::c_char,
        name_len: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_reader_open_session(r: libc::c_ulong, s: *mut u32) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_reader_close_sessions(r: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_session_is_closed(s: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_session_get_atr(
        s: libc::c_ulong,
        atr: *mut libc::c_void,
        atr_len: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_session_open_channel(
        s: libc::c_ulong,
        is_logical: libc::c_ulong,
        aid_buffer: *const libc::c_void,
        aid_buffer_len: size_t,
        c: *mut u32,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_session_close(s: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_channel_select_next(c: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_channel_get_select_resp(
        c: libc::c_ulong,
        resp: *mut libc::c_void,
        resp_len: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_channel_transmit(
        c: libc::c_ulong,
        cmd: *mut libc::c_void,
        cmd_len: size_t,
        resp: *mut libc::c_void,
        resp_len: *mut u64,
    ) -> TEE_Result;
}
extern "C" {
    pub fn utee_se_channel_close(c: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_cache_operation(va: *mut libc::c_void, l: size_t, op: libc::c_ulong) -> TEE_Result;
}
extern "C" {
    pub fn utee_gprof_send(buf: *mut libc::c_void, size: size_t, id: *mut u32) -> TEE_Result;
}
pub type __builtin_va_list = __va_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list {
    pub __ap: *mut libc::c_void,
}
#[test]
fn bindgen_test_layout___va_list() {
    assert_eq!(
        ::core::mem::size_of::<__va_list>(),
        4usize,
        concat!("Size of: ", stringify!(__va_list))
    );
    assert_eq!(
        ::core::mem::align_of::<__va_list>(),
        4usize,
        concat!("Alignment of ", stringify!(__va_list))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<__va_list>())).__ap as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list),
            "::",
            stringify!(__ap)
        )
    );
}
impl Default for __va_list {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
