<script lang="ts">
	import { page } from '$app/stores';
	import { chessSchema, type Chess } from '$lib/types';
	console.log($page.error);

	export let apiUrl: string;

	const fun = async () => {
		try {
			console.log('Resetting board');
			const response = await fetch(`${apiUrl}/api/chess`);
			const data = await response.json();
			const validatedChess: Chess = chessSchema.parse(data.chess);
			return validatedChess;
		} catch (error) {
			console.error(error);
		}
	};
</script>

<h1 class="text-indigo-50">{$page.status}: {$page.error?.message}</h1>
