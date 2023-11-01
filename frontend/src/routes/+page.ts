import { schema } from './types';

import { env } from '$env/dynamic/public';

/** @type {import('./$types').PageLoad} */
export const load = async ({ fetch }) => {
	let apiUrl: string;
	if (import.meta.env.MODE === 'development') {
		apiUrl = env.PUBLIC_DEV_URL;
	} else {
		apiUrl = env.PUBLIC_PROD_URL;
	}
	const response = await fetch(`${apiUrl}/api/chess`);
	const chess = await response.json();
	// console.log(JSON.stringify(chess));
	const validatedChess = schema.parse(chess);

	return { post: validatedChess, url: apiUrl };
};
