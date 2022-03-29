(() => {
  const form = document.getElementById('form');
  const customer_select = document.getElementById('customer');
  const iban = document.getElementById('iban');
  const currency = document.getElementById('currency');
  const account_holder_name = document.getElementById('account_holder_name');

  window.addEventListener('DOMContentLoaded', () => {
    getAll().then(data => {
      data.customers.forEach((customer, _) => {
        const customerOption = new Option(`${customer.given_name} ${customer.family_name}`, customer.id);
        customerOption.dataset.name = customer.given_name;
        customerOption.dataset.email = customer.email;
        customer_select.appendChild(customerOption);
      });
    })
  });

  form.addEventListener('submit', (event) => {
    event.preventDefault();

    let bank_account = {
      iban: iban.value,
      currency: currency.value,
      account_holder_name: account_holder_name.value,
      links: {
        customer: customer.value,
      }
    };

    post(`${SERVER_URL}/customer_bank_accounts`, bank_account)
      .then(data => {
        console.log({ bank_account: data });
        Flash.success('Bank Account created successfully!');
      })
      .catch((error) => {
        console.error({ bank_account: error });
        Flash.failure('Something went wrong while creating the bank account');
      });
    ;
  });
})();
