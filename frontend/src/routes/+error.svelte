<script lang="ts">
	import { page } from '$app/stores';
</script>

<h1 class=" text-indigo-50">{$page.status}: {$page.error?.message}</h1>
