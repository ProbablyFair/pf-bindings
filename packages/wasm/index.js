const path = require('path');
const fs = require('fs');

// Import the generated WebAssembly module
const { default: init, verify_bet, register_gdp_package, library_version, supported_features } = require('./pkg/pf_bindings_wasm.js');

let wasmInitialized = false;

async function ensureInitialized() {
    if (!wasmInitialized) {
        // Load the WebAssembly file
        const wasmBuffer = fs.readFileSync(path.join(__dirname, 'pkg', 'pf_bindings_wasm_bg.wasm'));
        await init(wasmBuffer);
        wasmInitialized = true;
    }
}

class PfBindingsWasm {
    static async verifyBet(receiptJson, transcriptJson) {
        await ensureInitialized();
        try {
            await verify_bet(receiptJson, transcriptJson);
        } catch (error) {
            throw new Error(`Verification failed: ${error.message}`);
        }
    }

    static async registerGdpPackage(bytes) {
        await ensureInitialized();
        try {
            if (typeof bytes === 'string') {
                // Base64 string
                await register_gdp_package_base64(bytes);
            } else if (bytes instanceof Uint8Array) {
                // Uint8Array
                await register_gdp_package_uint8_array(bytes);
            } else if (Buffer.isBuffer(bytes)) {
                // Node.js buffer
                await register_gdp_package(new Uint8Array(bytes));
            } else {
                throw new Error('bytes must be a Uint8Array, Buffer, or base64 string');
            }
        } catch (error) {
            throw new Error(`Registration failed: ${error.message}`);
        }
    }

    static async getVersion() {
        await ensureInitialized();
        return library_version();
    }

    static async getSupportedFeatures() {
        await ensureInitialized();
        return supported_features();
    }
}

module.exports = PfBindingsWasm;
