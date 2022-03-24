(() => {
  const form = document.getElementById('product-form');
  const productName = document.getElementById('product-name');
  const productPrice = document.getElementById('product-price');

  function sanitizePrice (price) {
    return parseFloat(price).toFixed(2)
  }

  form.addEventListener('submit', (event) => {
    event.preventDefault();

    post(`${SERVER_URL}/products`, { name: productName.value, price: sanitizePrice(productPrice.value) })
      .then(data => {
        console.log({ product: data });
        Flash.success('Product created successfully!');
      })
      .catch((error) => {
        console.error({ product: error });
        Flash.failure('Something went wrong while creating the product');
      });
  });
})();

