import { env } from '$env/dynamic/public';
import { error } from '@sveltejs/kit';

export const load = async ({ fetch }) => {
	let apiUrl: string;
	if (import.meta.env.MODE === 'development') {
		apiUrl = env.PUBLIC_DEV_URL;
	} else {
		apiUrl = env.PUBLIC_PROD_URL;
	}

	//this is to check if backend is online
	try {
		const response = await fetch(`${apiUrl}/api/chess`);
		if (!response.ok) {
			throw new Error('Failed to fetch chess data');
		}
		return { url: apiUrl };
	} catch (e) {
		return error(500, { message: 'Backend not found' });
	}
};
