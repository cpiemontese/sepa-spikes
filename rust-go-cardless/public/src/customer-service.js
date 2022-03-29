function getAll() {
  return get(`${SERVER_URL}/customers`);
}

function getCustomer(id) {
  return get(`${SERVER_URL}/customers/${id}`);
}
