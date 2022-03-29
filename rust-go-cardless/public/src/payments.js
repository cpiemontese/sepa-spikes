(() => {
  const form = document.getElementById('form');
  const amount = document.getElementById('amount');
  const currency = document.getElementById('currency');
  const mandateSelect = document.getElementById('mandate');

  window.addEventListener('DOMContentLoaded', () => {
    getAllMandates()
      .then(data => {
        console.log({ mandates: data });

        data.forEach((mandate) => {
          const mandateOption = new Option(mandate.name, mandate.id);
          mandateSelect.appendChild(mandateOption);
        });
      })
      .catch(error => {
        Flash.failure('Failed to fetch mandates');
        console.error({ mandates: error });
      })
  });

  form.addEventListener('submit', (event) => {
    event.preventDefault();

    let payment = {
      amount: amount.value,
      currency: currency.value,
      links: {
        mandate: mandate.value,
      }
    };

    post(`${SERVER_URL}/payments`, payment)
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
