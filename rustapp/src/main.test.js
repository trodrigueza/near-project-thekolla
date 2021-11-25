beforeAll(async function () {
  // NOTE: nearlib and nearConfig are made available by near-cli/test_environment
  const near = await nearlib.connect(nearConfig)
  window.accountId = nearConfig.contractName
  window.contract = await near.loadContract(nearConfig.contractName, {
    viewMethods: [],
    changeMethods: ['set_cid', 'transaction'],
    sender: window.accountId
  })
})

test('transaction', async () => {
  const trans = await window.contract.transaction({ account_id: window.accountId })
})
