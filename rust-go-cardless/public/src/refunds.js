(() => {
  const form = document.getElementById('form');
  const amount = document.getElementById('amount');
  const paymentSelect = document.getElementById('payment');

  window.addEventListener('DOMContentLoaded', () => {
    getAllPayments()
      .then(data => {
        console.log({ payments: data });

        data.payments.forEach((payment) => {
          paymentSelect.appendChild(new Option(payment.id, payment.id));
        });
      })
      .catch(error => {
        Flash.failure('Failed to fetch payments');
        console.error({ payments: error });
      })
  });

  form.addEventListener('submit', (event) => {
    event.preventDefault();

    let refund = {
      amount: amount.value,
      links: {
        payment: payment.value,
      }
    };

    post(`${SERVER_URL}/refunds`, refund)
      .then(data => {
        console.log({ refund: data });
        Flash.success('Customer created successfully!');
      })
      .catch((error) => {
        console.error({ refund: error });
        Flash.failure('Something went wrong while creating the payment');
      });
    ;
  });
})();
