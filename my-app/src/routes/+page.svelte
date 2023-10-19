<script lang="ts">
	export let data;
	console.log(data.post.board[0]);
	type Chess = typeof data;
</script>

<h1>Welcome to SvelteKit</h1>
<p>{data.post.board[0][0]}</p>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
