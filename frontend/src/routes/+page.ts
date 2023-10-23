import { schema } from './types';

/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
	const response = await fetch(`http://127.0.0.1:8000/chess`);
	const chess = await response.json();
	console.log(chess);
	const validatedChess = schema.parse(chess);

	return { post: validatedChess };
}
