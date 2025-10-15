import binding from './native';

export function verifyBet(receiptJson: string, transcriptJson: string): void {
  binding.verify_bet(receiptJson, transcriptJson);
}

export function registerGDPPackage(bytes: Buffer | Uint8Array): void {
  const buffer = Buffer.isBuffer(bytes) ? bytes : Buffer.from(bytes);
  binding.register_gdp_package(buffer);
}
