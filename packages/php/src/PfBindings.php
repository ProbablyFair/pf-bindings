<?php

namespace ProbablyFair\PfBindings;

class PfBindingsError extends \Exception {}

/**
 * PHP bindings for Probably Fair operations
 */
class PfBindings
{
    /**
     * Verifies a bet receipt against its transcript
     *
     * @param string $receiptJson The bet receipt as a JSON string
     * @param string $transcriptJson The transcript as a JSON string
     * @throws PfBindingsError If verification fails
     */
    public static function verifyBet(string $receiptJson, string $transcriptJson): void
    {
        try {
            if (!extension_loaded('pf-bindings')) {
                throw new PfBindingsError('pf-bindings extension not loaded');
            }
            
            $result = \pf_verify_bet($receiptJson, $transcriptJson);
            
            if ($result !== null) {
                throw new PfBindingsError("Verification failed: {$result}");
            }
        } catch (\Exception $e) {
            throw new PfBindingsError("Verification failed: {$e->getMessage()}");
        }
    }
    
    /**
     * Registers a GDP package for use in betting operations
     *
     * @param string $bytes The GDP package binary data as a string
     * @throws PfBindingsError If registration fails
     */
    public static function registerGdpPackage(string $bytes): void
    {
        try {
            if (!extension_loaded('pf-bindings')) {
                throw new PfBindingsError('pf-bindings extension not loaded');
            }
            
            $byteArray = array_values(unpack('C*', $bytes));
            $result = \pf_register_gdp_package($byteArray);
            
            if ($result !== null) {
                throw new PfBindingsError("Registration failed: {$result}");
            }
        } catch (\Exception $e) {
            throw new PfBindingsError("Registration failed: {$e->getMessage()}");
        }
    }
    
    /**
     * Convenience method for registering a GDP package from a hex string
     *
     * @param string $hexString The package data as a hex string
     * @throws PfBindingsError If registration fails
     */
    public static function registerGdpPackageFromHex(string $hexString): void
    {
        try {
            $bytes = hex2bin($hexString);
            if ($bytes === false) {
                throw new PfBindingsError('Invalid hex string');
            }
            
            self::registerGdpPackage($bytes);
        } catch (PfBindingsError $e) {
            throw $e;
        } catch (\Exception $e) {
            throw new PfBindingsError("Registration failed: {$e->getMessage()}");
        }
    }
}
