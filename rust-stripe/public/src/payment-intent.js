(() => {
  const form = document.getElementById('payment-form');
  const customerId = document.getElementById('customer-id');
  const paymentMethod = document.getElementById('payment-method');
  const amount = document.getElementById('payment-amount');
  const currency = document.getElementById('payment-currency');

  const stripeForm = document.getElementById('stripe-form');

  form.addEventListener('submit', (event) => {
    event.preventDefault();

    post(`${SERVER_URL}/payment-intents`, { customer_id: customerId.value, amount: amount.value, payment_method: paymentMethod.value, currency: currency.value })
      .then(paymentIntent => {
        console.log({ paymentIntent });
        form.classList.add('hidden');
        stripeForm.classList.remove('hidden');
        stripeForm.dataset.secret = paymentIntent.client_secret;
      })
      .catch((error) => {
        console.error({ paymentIntent: error });
        Flash.failure('Something went wrong while setting up your payment, try again later');
      });
  });
})();

