const Customer = (() => {
  const form = document.getElementById('form');
  const customerName = document.getElementById('name');
  const customerEmail = document.getElementById('email');
  const customerFamilyName = document.getElementById('family-name');

  form.addEventListener('submit', (event) => {
    event.preventDefault();

    post(`${SERVER_URL}/customers`, { given_name: customerName.value, family_name: customerFamilyName.value, email: customerEmail.value })
      .then(data => {
        console.log({ customer: data });
        Flash.success('Customer created successfully!');
      })
      .catch((error) => {
        console.error({ customer: error });
        Flash.failure('Something went wrong while creating the customer');
      });
  });
})();
