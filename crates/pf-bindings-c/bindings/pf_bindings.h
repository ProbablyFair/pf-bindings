#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Error codes for C API
 */
#define PF_SUCCESS 0

#define PF_ERROR -1

/**
 * Result codes for verification and registration
 */
typedef enum PfResult {
  PfResultSuccess = 0,
  PfResultVerificationFailed = -1,
  PfResultRegistrationFailed = -2,
  PfResultInvalidInput = -3,
  PfResultInternalError = -4,
} PfResult;

/**
 * Context for error messages
 */
typedef struct PfError {
  char *message;
} PfError;

/**
 * Verify a bet receipt against its transcript
 *
 * # Parameters
 * - receipt_json: JSON string containing the bet receipt
 * - transcript_json: JSON string containing the transcript
 * - error_out: Output parameter for error message (can be null)
 *
 * # Returns
 * PfResult::Success on success, error code otherwise
 */
enum PfResult pf_verify_bet(const char *receipt_json,
                            const char *transcript_json,
                            struct PfError *error_out);

/**
 * Register a GDP package for use in betting operations
 *
 * # Parameters
 * - bytes: Pointer to the GDP package binary data
 * - len: Length of the data in bytes
 * - error_out: Output parameter for error message (can be null)
 *
 * # Returns
 * PfResult::Success on success, error code otherwise
 */
enum PfResult pf_register_gdp_package(const unsigned char *bytes,
                                      uintptr_t len,
                                      struct PfError *error_out);

/**
 * Free error message memory
 *
 * # Parameters
 * - error: Error structure containing the message to free
 */
void pf_free_error(struct PfError error);

/**
 * Get string representation of result code
 *
 * # Parameters
 * - result: Result code
 *
 * # Returns
 * Static string describing the result (do not free)
 */
const char *pf_result_string(enum PfResult result);

/**
 * Get library version
 *
 * # Returns
 * Static string containing version info (do not free)
 */
const char *pf_library_version(void);
