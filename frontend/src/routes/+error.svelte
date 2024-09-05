<script lang="ts">
	import { page } from '$app/stores';
	console.log($page.error);
</script>

<h1 class="text-indigo-50">{$page.status}: {$page.error?.message}</h1>
<div>
	if backend is not online, it is probably idle on shuttle.rs servers. It takes
	a few seconds to a minute to boot up
</div>
