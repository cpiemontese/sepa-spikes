function getAllMandates() {
  return get(`${SERVER_URL}/mandates`)
    .then(data => {
      return data.mandates.map(async (mandate) => {
        let customer = (await getCustomer(mandate.links.customer)).customers;

        return { name: `${customer.given_name} ${customer.family_name}`, id: mandate.id }
      });
    })
    .then(data => Promise.all(data))
}
