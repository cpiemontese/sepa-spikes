const SERVER_URL = 'http://localhost:8080';
const STRIPE_PUBLIC_KEY = 'pk_test_51KfjAzD9axmx7ico2Sa1hQivJdJvXZ5Yx6ssC9vZ3vwRCADlsXJgDYiRj07LWehg2pLfYYkpVIhW0X1E2kLw9pgj00BmqmTPM1';

async function postData (url = '', data = {}) {
	const response = await fetch(url, {
		method: 'POST',
		mode: 'cors',
		cache: 'no-cache',
		// include, *same-origin, omit
		credentials: 'same-origin',
		headers: {
			'Content-Type': 'application/json'
		},
		redirect: 'follow',
		referrerPolicy: 'no-referrer',
		body: JSON.stringify(data)
	});
	return response.json();
}

const Flash = (() => {
	const _FLASH_MESSAGE = document.getElementById('flash-message')

	let lastTimeout = null

	function clearTimeout () {
		if (lastTimeout) {
			clearTimeout(lastTimeout)
		}
	}

	return {
		success (msg) {
			// Avoid strange interactions in case a new flash arrives while another is still active
			clearTimeout();

			_FLASH_MESSAGE.innerHTML = msg;
			_FLASH_MESSAGE.classList.remove('hidden');
			_FLASH_MESSAGE.classList.remove('bg-red-300');
			_FLASH_MESSAGE.classList.add('bg-green-300');

			lastTimeout = setTimeout(() => {
				_FLASH_MESSAGE.classList.add('hidden');
				_FLASH_MESSAGE.classList.remove('bg-green-300');
			}, 5000)
		},
		failure (msg) {
			// Avoid strange interactions in case a new flash arrives while another is still active
			clearTimeout();

			_FLASH_MESSAGE.innerHTML = msg;
			_FLASH_MESSAGE.classList.remove('hidden');
			_FLASH_MESSAGE.classList.remove('bg-green-300');
			_FLASH_MESSAGE.classList.add('bg-red-300');

			lastTimeout = setTimeout(() => {
				_FLASH_MESSAGE.classList.add('hidden');
				_FLASH_MESSAGE.classList.remove('bg-red-300');
			}, 5000)
		}
	}
})();
