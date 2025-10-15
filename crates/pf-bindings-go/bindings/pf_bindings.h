#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

int pf_verify_bet(const char *receipt_json, const char *transcript_json, char **error_out);

int pf_register_gdp_package(const unsigned char *bytes, uintptr_t len, char **error_out);

void pf_free_error(char *error);
