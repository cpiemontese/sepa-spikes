const PaymentDetails = (() => {
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
          productSelect.appendChild(productOption);
        });
      })
      .catch(error => {
        Flash.failure('Failed to fetch prices');
        console.error({ prices: error });
      })
  });

  const paymentDetailsForm = document.getElementById('payment-details-form');
  const setupIntentForm = document.getElementById('setup-intent-form');

  paymentDetailsForm.addEventListener('submit', (e) => {
    e.preventDefault();

    customerSelect.setAttribute('disabled', 'true');
    productSelect.setAttribute('disabled', 'true');

    setupIntentForm.classList.remove('hidden');
  })

  return {
    getSelectedCustomer () {
      const selectedCustomer = customerSelect.options[customerSelect.selectedIndex]
      return {
        id: selectedCustomer.value,
        name: selectedCustomer.dataset.name,
        email: selectedCustomer.dataset.email
      };
    },
    getSelectedProduct () {
      const selectedProduct = productSelect.options[productSelect.selectedIndex]
      return {
        id: selectedProduct.value,
        priceId: selectedProduct.dataset.priceId,
      };
    }
  }
})();

