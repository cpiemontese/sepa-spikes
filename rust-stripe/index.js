const stripe = Stripe('pk_test_51KfjAzD9axmx7ico2Sa1hQivJdJvXZ5Yx6ssC9vZ3vwRCADlsXJgDYiRj07LWehg2pLfYYkpVIhW0X1E2kLw9pgj00BmqmTPM1');
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

const form = document.getElementById('payment-form');
const accountholderName = document.getElementById('accountholder-name');
const email = document.getElementById('email');
const submitButton = document.getElementById('submit-button');
const clientSecret = submitButton.dataset.secret;

form.addEventListener('submit', (event) => {
  event.preventDefault();
  stripe.confirmSepaDebitPayment(
    clientSecret,
    {
      payment_method: {
        sepa_debit: iban,
        billing_details: {
          name: accountholderName.value,
          email: email.value,
        },
      },
    }
  );
});
