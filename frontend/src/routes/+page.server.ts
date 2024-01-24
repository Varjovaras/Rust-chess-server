import { schema } from './types';
import { env } from '$env/dynamic/public';
import { startingPosition } from '$lib/startingPosition';
import { error } from '@sveltejs/kit';

export const load = async ({ fetch }) => {
	let apiUrl: string;
	if (import.meta.env.MODE === 'development') {
		apiUrl = env.PUBLIC_DEV_URL;
	} else {
		apiUrl = env.PUBLIC_PROD_URL;
	}

	try {
		const response = await fetch(apiUrl);
		if (!response.ok) {
			throw new Error('Failed to fetch chess data');
		}
		const validatedChess = schema.parse(startingPosition);

		return { post: validatedChess, url: apiUrl };
	} catch (e) {
		return error(500, { message: 'Backend not found' });
	}
};
