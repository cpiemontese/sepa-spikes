(() => {
  const form = document.getElementById('customer-form');
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

    postData(`${serverUrl}/customer`, { name: customerName.value, email: customerEmail.value })
      .then(data => {
        console.log(data); // JSON data parsed by `data.json()` call
        form.classList.add('hidden');
      });
  });
})();

