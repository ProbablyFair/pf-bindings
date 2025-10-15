#include "pf_bindings.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    printf("PF Bindings C Example\n");
    printf("Library version: %s\n\n", pf_library_version());
    
    // Example: Verify a bet
    const char* receipt_json = "{}";  // Invalid JSON for demonstration
    const char* transcript_json = "{}";
    
    struct PfError error;
    PfResult result = pf_verify_bet(receipt_json, transcript_json, &error);
    
    if (result == PfResult_Success) {
        printf("Bet verification succeeded!\n");
    } else {
        printf("Bet verification failed: %s\n", pf_result_string(result));
        if (error.message != NULL) {
            printf("Error details: %s\n", error.message);
            pf_free_error(error);
        }
    }
    
    // Example: Register a GDP package
    unsigned char package_data[] = {0x01, 0x02, 0x03, 0x04};
    
    error = (struct PfError){NULL};
    result = pf_register_gdp_package(package_data, sizeof(package_data), &error);
    
    if (result == PfResult_Success) {
        printf("GDP package registration succeeded!\n");
    } else {
        printf("GDP package registration failed: %s\n", pf_result_string(result));
        if (error.message != NULL) {
            printf("Error details: %s\n", error.message);
            pf_free_error(error);
        }
    }
    
    printf("\nExample complete.\n");
    return result == PfResult_Success ? 0 : 1;
}

// Makefile example for building
/*
CC=gcc
PREFIX=/usr/local
LIBPFDIR=../../target/release

all: example

example: example.c
	$(CC) -Wall -Wextra -O2 -I. -L$(LIBPFDIR) -lpf_bindings_c -o example $<

clean:
	rm -f example

install: libpf_bindings_c.a
	install -m 644 libpf_bindings_c.a $(PREFIX)/lib/
	install -m 644 pf_bindings.h $(PREFIX)/include/

.PHONY: all clean install
*/
