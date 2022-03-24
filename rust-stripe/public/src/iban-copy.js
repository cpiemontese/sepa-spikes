window.addEventListener('DOMContentLoaded', () => {
  Array.from(document.getElementsByClassName('iban-number')).forEach(number => {
    number.addEventListener('click', (e) => {
      e.preventDefault();

      navigator.clipboard.writeText(number.textContent)
        .then(() => Flash.success('Copied to clipboard!'))
        .catch((error) => {
          Flash.failure('Could not copy to clipboard');
          console.error({ clipboard: error });
        });
    });
  });
});

