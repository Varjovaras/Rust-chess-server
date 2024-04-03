<script lang="ts">
	import { page } from '$app/stores';
	console.log($page.error);
</script>

<h1 class="text-indigo-50">{$page.status}: {$page.error?.message}</h1>
