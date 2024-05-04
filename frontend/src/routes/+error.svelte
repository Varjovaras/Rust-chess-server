<script lang="ts">
	import { page } from '$app/stores';
	import { chessSchema, type Chess } from '$lib/types';
	console.log($page.error);

	export let apiUrl: string;
</script>

<h1 class="text-indigo-50">{$page.status}: {$page.error?.message}</h1>
