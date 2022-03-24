(() => {
    const form = document.getElementById('setup-intent-form');
    const paymentMethod = document.getElementById('payment-method');
    const stripeForm = document.getElementById('stripe-form');

    form.addEventListener('submit', (event) => {
        event.preventDefault();

        const { id: customerId } = PaymentDetails.getSelectedCustomer();

        post(`${SERVER_URL}/setup-intents`, { customer_id: customerId, payment_method: paymentMethod.value })
            .then(setupIntent => {
                console.log({ setupIntent });
                form.classList.add('hidden');
                stripeForm.classList.remove('hidden');
                stripeForm.dataset.secret = setupIntent.client_secret;
            })
            .catch((error) => {
                console.error({ setupIntent: error });
                Flash.failure('Something went wrong while setting up your payment, try again later');
            });
    });
})();
