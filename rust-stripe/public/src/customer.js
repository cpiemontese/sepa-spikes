
(() => {
  const form = document.getElementById('customer-form');
  const paymentForm = document.getElementById('payment-form');
  const customerName = document.getElementById('customer-name');
  const customerEmail = document.getElementById('customer-email');
  const select = document.getElementById('customer-id');

  window.addEventListener('DOMContentLoaded', (event) => {
    get(`${SERVER_URL}/customers`)
      .then(data => {
        data.data.forEach(customer => {
          select.appendChild(new Option(customer.name, customer.id));
        });
      })
  });

  form.addEventListener('submit', (event) => {
    event.preventDefault();

    post(`${SERVER_URL}/customers`, { name: customerName.value, email: customerEmail.value })
      .then(data => {
        console.log({ customer: data });
        form.classList.add('hidden');
        paymentForm.classList.remove('hidden');
        document.getElementById('customer-id').value = data.id
      })
      .catch((error) => {
        console.error({ customer: error });
        Flash.failure('Something went wrong while saving your data');
      });
    ;
  });
})();

