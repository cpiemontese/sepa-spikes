(() => {
    const form = document.getElementById('setup-intent-form');
    const customerId = document.getElementById('customer-id');
    const paymentMethod = document.getElementById('payment-method');
    const stripeForm = document.getElementById('stripe-form');

    form.addEventListener('submit', (event) => {
        event.preventDefault();

        postData(`${SERVER_URL}/setup-intents`, { customer_id: customerId.value, payment_method: paymentMethod.value})
            .then(paymentIntent => {
                console.log({ paymentIntent });
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
