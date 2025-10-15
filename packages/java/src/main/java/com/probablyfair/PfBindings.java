package com.probablyfair;

import java.util.Arrays;

public class PfBindings {
    static {
        // Load the native library
        // The library name should match the compiled Rust library name
        System.loadLibrary("pf_bindings_java");
    }

    /**
     * Verifies a bet receipt against its transcript.
     *
     * @param receiptJson The bet receipt JSON string
     * @param transcriptJson The transcript JSON string
     * @throws Exception if verification fails
     */
    public static native void verifyBet(String receiptJson, String transcriptJson) throws Exception;

    /**
     * Registers a GDP package for use in betting operations.
     *
     * @param bytes The GDP package binary data
     * @throws Exception if registration fails
     */
    public static native void registerGdpPackage(byte[] bytes) throws Exception;

    /**
     * Convenience method for registering a GDP package from a string.
     *
     * @param packageData The package data as a string
     * @throws Exception if registration fails
     */
    public static void registerGdpPackageFromString(String packageData) throws Exception {
        if (packageData == null) {
            throw new IllegalArgumentException("Package data cannot be null");
        }
        registerGdpPackage(packageData.getBytes());
    }
}
