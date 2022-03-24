(() => {
    const form = document.getElementById('subscription-form');
    const button = document.getElementById('subscription-button');

    form.addEventListener('submit', (event) => {
        event.preventDefault();

        const { id: customerId } = PaymentDetails.getSelectedCustomer();
        const { priceId } = PaymentDetails.getSelectedProduct();

        post(`${SERVER_URL}/subscriptions`, { customer_id: customerId, price_id: priceId })
            .then(data => {
                button.setAttribute('disabled', 'true');
                console.log({ subscription_output: data });
                Flash.success('Subscription completed successfully!');
            })
            .catch((error) => {
                console.error({ error: error });
                Flash.failure('Something went wrong while creating a new subscription');
            });
    });
})();
