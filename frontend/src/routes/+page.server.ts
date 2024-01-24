import { schema } from './types';

import { env } from '$env/dynamic/public';
import { startingPosition } from '$lib/startingPosition';

export const load = async () => {
	let apiUrl: string;
	if (import.meta.env.MODE === 'development') {
		apiUrl = env.PUBLIC_DEV_URL;
	} else {
		apiUrl = env.PUBLIC_PROD_URL;
	}

	const chess = startingPosition;
	const validatedChess = schema.parse(chess);

	return { post: validatedChess, url: apiUrl };
};
