const binding = require('../index.node') as {
  verify_bet(receiptJson: string, transcriptJson: string): void;
  register_gdp_package(bytes: Buffer): void;
};

export default binding;
