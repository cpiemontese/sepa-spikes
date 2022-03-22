(() => {
  const form = document.getElementById('payment-form');
  const customerId = document.getElementById('customer-id');
  const paymentMethod = document.getElementById('payment-method');
  const amount = document.getElementById('payment-amount');
  const currency = document.getElementById('payment-currency');

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

    postData(`${serverUrl}/payment-intents`, { customer_id: customerId.value, amount: amount.value, payment_method: paymentMethod.value, currency: currency.value })
      .then(_ => {
        form.classList.add('hidden');
        form.classList.remove('hidden');
      });
  });
})();

