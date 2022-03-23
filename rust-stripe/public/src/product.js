(() => {
  const form = document.getElementById('product-form');
  const productName = document.getElementById('product-name');
  const productPrice = document.getElementById('product-price');

  const priceId = document.getElementById('price-id');
  const setupIntentForm = document.getElementById('setup-intent-form');

  function sanitizePrice (price) {
    return parseFloat(price).toFixed(2)
  }

  form.addEventListener('submit', (event) => {
    event.preventDefault();

    console.log({ payload: { name: productName.value, price: sanitizePrice(productPrice.value) } });

    postData(`${SERVER_URL}/products`, { name: productName.value, price: sanitizePrice(productPrice.value) })
      .then(data => {
        console.log({ product: data });
        form.classList.add('hidden');
        priceId.value = data.id;
        setupIntentForm.classList.remove('hidden');
      })
      .catch((error) => {
        console.error({ product: error });
        Flash.failure('Something went wrong while creating the product');
      });
    ;
  });
})();

