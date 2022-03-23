(() => {
  const stripe = Stripe(STRIPE_PUBLIC_KEY);
  const elements = stripe.elements();

  // Custom styling can be passed to options when creating an Element.
  const style = {
    base: {
      color: '#32325d',
      fontSize: '16px',
      '::placeholder': {
        color: '#aab7c4'
      },
      ':-webkit-autofill': {
        color: '#32325d',
      },
    },
    invalid: {
      color: '#fa755a',
      iconColor: '#fa755a',
      ':-webkit-autofill': {
        color: '#fa755a',
      },
    },
  };

  const options = {
    style,
    supportedCountries: ['SEPA'],
    // Elements can use a placeholder as an example IBAN that reflects
    // the IBAN format of your customer's country. If you know your
    // customer's country, we recommend passing it to the Element as the
    // placeholderCountry.
    placeholderCountry: 'NL',
  };

  // Create an instance of the IBAN Element
  const iban = elements.create('iban', options);

  // Add an instance of the IBAN Element into the `iban-element` <div>
  iban.mount('#iban-element');

  const form = document.getElementById('stripe-form');
  const subscriptionForm = document.getElementById('subscription-form');
  const stripeCustomerName = document.getElementById('stripe-customer-name');
  const stripeCustomerEmail = document.getElementById('stripe-customer-email');

  form.addEventListener('submit', (event) => {
    event.preventDefault();
    stripe.confirmSepaDebitSetup(
      form.dataset.secret,
      {
        payment_method: {
          sepa_debit: iban,
          billing_details: {
            name: stripeCustomerName.value,
            email: stripeCustomerEmail.value,
          },
        },
      }
    )
      .then((data) => {
        console.log({ sepaPayment: data });
        Flash.success('Payment completed successfully!');
        setAsDefaultPaymentMethod(data.setupIntent.payment_method);
      })
      .catch((error) => {
        console.error({ sepaPayment: error });
        Flash.failure('Something went wrong while processing your payment, try again later');
      });
  });

  function setAsDefaultPaymentMethod(paymentMethod) {
    let customerId = document.getElementById('customer-id').value;
    postData(`${SERVER_URL}/customers/${customerId}`, {payment_method: paymentMethod})
    .then((data) => {
      console.log({result: data});
      form.classList.add('hidden')
      subscriptionForm.classList.remove('hidden')
    }).catch((error)=> {
      console.error({error: error});
      Flash.failure('Could not set the default payment')
    })
  }
})();
