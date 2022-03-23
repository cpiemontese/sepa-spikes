const SERVER_URL = 'http://localhost:8080';
const STRIPE_PUBLIC_KEY = 'pk_test_51KfjALGPUghqTKYzy0efLlYBX9ltHTFjxgKWsEO5UgvP9UqTH71eB5LTalEYblX3wHdN4C80oO7duxDMaT1BdtC500JmberCRj';

async function post (url = '', data = {}) {
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

async function get (url = '') {
	const response = await fetch(url, {
		method: 'GET',
		mode: 'cors',
		cache: 'no-cache',
		// include, *same-origin, omit
		credentials: 'same-origin',
		redirect: 'follow',
		referrerPolicy: 'no-referrer'
	});
	return response.json();
}

const Flash = (() => {
	const _FLASH_MESSAGE = document.getElementById('flash-message')

	let lastTimeout = null

	function resetTimeout () {
		if (lastTimeout) {
			clearTimeout(lastTimeout)
		}
	}

	return {
		success (msg) {
			// Avoid strange interactions in case a new flash arrives while another is still active
			resetTimeout();

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
			resetTimeout();

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
