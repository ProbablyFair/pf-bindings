#ifndef PF_BINDINGS_H
#define PF_BINDINGS_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

// Result codes for verification and registration
typedef enum PfResult {
    PfResult_Success = 0,
    PfResult_VerificationFailed = -1,
    PfResult_RegistrationFailed = -2,
    PfResult_InvalidInput = -3,
    PfResult_InternalError = -4,
} PfResult;

// Context for error messages
typedef struct PfError {
    char* message;
} PfError;

// Verify a bet receipt against its transcript
PfResult pf_verify_bet(
    const char* receipt_json,
    const char* transcript_json,
    struct PfError* error_out
);

// Register a GDP package for use in betting operations
PfResult pf_register_gdp_package(
    const unsigned char* bytes,
    size_t len,
    struct PfError* error_out
);

// Free error message memory
void pf_free_error(struct PfError error);

// Get library version
const char* pf_library_version();

#ifdef __cplusplus
}
#endif

#endif /* PF_BINDINGS_H */
