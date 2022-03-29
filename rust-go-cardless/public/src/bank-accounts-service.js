function getAllBankAccounts() {
  return get(`${SERVER_URL}/customer_bank_accounts`)
    .then(data => {
      return data.customer_bank_accounts
    })
}
