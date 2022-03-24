
(() => {
  const form = document.getElementById('customer-form');
  const customerName = document.getElementById('customer-name');
  const customerEmail = document.getElementById('customer-email');

  form.addEventListener('submit', (event) => {
    event.preventDefault();

    post(`${SERVER_URL}/customers`, { name: customerName.value, email: customerEmail.value })
      .then(data => {
        console.log({ customer: data });
        Flash.success('Customer created successfully!');
      })
      .catch((error) => {
        console.error({ customer: error });
        Flash.failure('Something went wrong while creating the customer');
      });
    ;
  });
})();

