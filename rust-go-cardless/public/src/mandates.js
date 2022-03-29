(() => {
  const form = document.getElementById('form');
  const bankAccountSelect = document.getElementById('customer-bank-account');

  window.addEventListener('DOMContentLoaded', () => {
    getAllBankAccounts()
      .then(data => {
        console.log({ customers: data });

        data.forEach((bankAccount, _) => {
          const bankAccountOption = new Option(bankAccount.account_holder_name, bankAccount.id);
          bankAccountSelect.appendChild(bankAccountOption);
        });
      })
      .catch(error => {
        Flash.failure('Failed to fetch bankAccounts');
        console.error({ bankAccounts: error });
      })
  });

  form.addEventListener('submit', (event) => {
    event.preventDefault();

    let mandates = {
      links: {
        customer_bank_account: bankAccountSelect.value,
      }
    };

    post(`${SERVER_URL}/mandates`, mandates)
      .then(data => {
        console.log({ mandates: data });
        Flash.success('Customer created successfully!');
      })
      .catch((error) => {
        console.error({ mandates: error });
        Flash.failure('Something went wrong while creating the mandate');
      });
    ;
  });
})();
