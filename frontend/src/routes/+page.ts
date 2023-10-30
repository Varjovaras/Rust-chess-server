import { schema } from './types';

/** @type {import('./$types').PageLoad} */
export const load = async ({ fetch }) => {
	const response = await fetch(`http://127.0.0.1:8000/api/chess`);
	const chess = await response.json();
	// console.log(JSON.stringify(chess));
	const validatedChess = schema.parse(chess);
	return { post: validatedChess };
};
