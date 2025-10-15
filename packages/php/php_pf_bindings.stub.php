<?php
/**
 * PHP bindings for Probably Fair core library
 */

const PF_BINDING_VERSION = "0.1.1";

/**
 * Verifies a bet receipt against its transcript
 * 
 * @param string $receiptJson The bet receipt as a JSON string
 * @param string $transcriptJson The transcript as a JSON string
 * @throws PfBindingsException If verification fails
 */
function pf_verify_bet(string $receiptJson, string $transcriptJson): void {
    // This function is implemented in Rust
}

/**
 * Registers a GDP package for use in betting operations
 * 
 * @param string $bytes The GDP package binary data as a string
 * @throws PfBindingsException If registration fails
 */
function pf_register_gdp_package(string $bytes): void {
    // This function is implemented in Rust
}

/**
 * Exception class for PF bindings errors
 */
class PfBindingsException extends Exception {
    public function __construct(string $message = "", int $code = 0, ?Throwable $previous = null) {
        parent::__construct($message, $code, $previous);
    }
}
