import { defineEnvVars } from '@sveltejs/kit/env';

export const variables = defineEnvVars({
	BACKEND_HOST: { schema: (input) => input ?? '' },
	BACKEND_PORT: { schema: (input) => input ?? '' }
});
