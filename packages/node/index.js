const native = require('./pf-bindings.node')

module.exports = {
  verifyBet: (receiptJson, transcriptJson) => {
    return native.verify_bet(receiptJson, transcriptJson)
  },
  registerGdpPackage: (bytes) => {
    return native.register_gdp_package(bytes)
  }
}