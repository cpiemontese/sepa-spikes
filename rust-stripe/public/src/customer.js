(() => {
  const form = document.getElementById('customer-form');
  const paymentForm = document.getElementById('payment-form');
  const customerName = document.getElementById('customer-name');
  const customerEmail = document.getElementById('customer-email');

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

    postData(`${SERVER_URL}/customers`, { name: customerName.value, email: customerEmail.value })
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

