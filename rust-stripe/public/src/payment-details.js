(() => {
  const customerSelect = document.getElementById('customer-id');
  const productSelect = document.getElementById('product-id');

  window.addEventListener('DOMContentLoaded', () => {
    get(`${SERVER_URL}/customers`)
      .then(data => {
        console.log({ customers: data });

        data.data.forEach((customer, index) => {
          const customerOption = new Option(customer.name, customer.id);
          customerOption.dataset.name = customer.name;
          customerOption.dataset.email = customer.email;
          customerSelect.appendChild(customerOption);
        });
      })
      .catch(error => {
        Flash.failure('Failed to fetch customers');
        console.error({ customers: error });
      })
  });

  window.addEventListener('DOMContentLoaded', () => {
    get(`${SERVER_URL}/prices`)
      .then(data => {
        console.log({ prices: data });

        data.data.forEach(price => {
          const productOption = new Option(`${price.product.name} - ${price.unit_amount / 100.0}€`, price.product.id);
          productOption.dataset.priceId = price.id;
          productOption.dataset.productId = price.product.id;
          productSelect.appendChild(productOption);
        });
      })
      .catch(error => {
        Flash.failure('Failed to fetch prices');
        console.error({ prices: error });
      })
  });
})();

