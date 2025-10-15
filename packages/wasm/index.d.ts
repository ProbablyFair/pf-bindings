export declare class PfBindingsWasm {
    static verifyBet(receiptJson: string, transcriptJson: string): Promise<void>;
    static registerGdpPackage(bytes: Uint8Array | Buffer | string): Promise<void>;
    static getVersion(): Promise<string>;
    static getSupportedFeatures(): Promise<string[]>;
}
