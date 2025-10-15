#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Result codes C-compatible enum for PHP compatibility
 */
typedef enum PfResult {
  Success = 0,
  VerificationFailed = -1,
  RegistrationFailed = -2,
  InvalidInput = -3,
  InternalError = -4,
} PfResult;

/**
 * Error context structure for PHP compatibility
 */
typedef struct PfError {
  char *message;
} PfError;

/**
 * Verify a bet receipt against its transcript
 */
enum PfResult pf_verify_bet(const char *receipt_json,
                            const char *transcript_json,
                            struct PfError *error_out);

/**
 * Register a GDP package for use in betting operations
 */
enum PfResult pf_register_gdp_package(const unsigned char *bytes,
                                      uintptr_t len,
                                      struct PfError *error_out);

/**
 * Free error message memory
 */
void pf_free_error(struct PfError error);

/**
 * Get library version
 */
const char *pf_library_version(void);
