import { browser } from '$app/environment';

export function registerServiceWorker() {
	if (browser && 'serviceWorker' in navigator) {
		navigator.serviceWorker
			.register('/sw.js')
			.then((registration) => {
				console.log('Service Worker registered successfully:', registration);
			})
			.catch((error) => {
				console.log('Service Worker registration failed:', error);
			});
	}
}
