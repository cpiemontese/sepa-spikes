(() => {
    const form = document.getElementById('subscription-form');
    const customerId = document.getElementById('customer-id');
    const priceId = document.getElementById('price-id');

    form.addEventListener('submit', (event) => {
        event.preventDefault();

        post(`${SERVER_URL}/subscriptions`, { customer_id: customerId.value, price_id: priceId.value })
            .then(data => {
                Flash.success('Subscription Done!');
                console.log({ subscription_output: data });
            })
            .catch((error) => {
                console.error({ error: error });
                Flash.failure('Something went wrong while creating a new subscription');
            });
    });
})();
