(() => {
  const form = document.getElementById('payment-form');
  const customerId = document.getElementById('customer-id');
  const paymentMethod = document.getElementById('payment-method');
  const amount = document.getElementById('payment-amount');
  const currency = document.getElementById('payment-currency');

  const stripeForm = document.getElementById('stripe-form');

  async function postData (url = '', data = {}) {
    const response = await fetch(url, {
      method: 'POST',
      mode: 'cors',
      cache: 'no-cache',
      // include, *same-origin, omit
      credentials: 'same-origin',
      headers: {
        'Content-Type': 'application/json'
      },
      redirect: 'follow',
      referrerPolicy: 'no-referrer',
      body: JSON.stringify(data)
    });
    return response.json();
  }

  form.addEventListener('submit', (event) => {
    event.preventDefault();

    postData(`${SERVER_URL}/payment-intents`, { customer_id: customerId.value, amount: amount.value, payment_method: paymentMethod.value, currency: currency.value })
      .then(paymentIntent => {
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

