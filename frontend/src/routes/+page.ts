import { schema } from './types';

/** @type {import('./$types').PageLoad} */
export const load = async ({ fetch }) => {
	let apiUrl: string;
	if (import.meta.env.MODE === 'development') {
		apiUrl = import.meta.env.DEV_URL;
	} else {
		apiUrl = import.meta.env.PROD_URL;
	}

	const response = await fetch(`${apiUrl}/api/chess`);
	const chess = await response.json();
	// console.log(JSON.stringify(chess));
	const validatedChess = schema.parse(chess);

	return { post: validatedChess, url: apiUrl };
};
