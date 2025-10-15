#ifndef PF_BINDINGS_H
#define PF_BINDINGS_H

#ifdef __cplusplus
extern "C" {
#endif

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

// Get string representation of result code
const char* pf_result_string(PfResult result);

// Get library version
const char* pf_library_version();

// Convenience macros for error handling
#define PF_CHECK_ERROR(err, label) if ((err) != PfResult_Success) goto label
#define PF_HANDLE_ERROR(err_ptr, result) do { \
    if ((err_ptr) != NULL) { \
        (*err_ptr) = err; \
    } \
    return (result); \
} while(0)

// Helper function to print error messages
static inline void pf_print_error(struct PfError error) {
    if (error.message != NULL) {
        fprintf(stderr, "PF Error: %s\n", error.message);
        pf_free_error(error);
    } else {
        fprintf(stderr, "PF Error: Unknown error\n");
    }
}

#ifdef __cplusplus
}
#endif

#endif /* PF_BINDINGS_H */
