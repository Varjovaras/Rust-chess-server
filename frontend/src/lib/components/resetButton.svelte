<script lang="ts">
	export let handleReset: () => void;
</script>

<button
	on:click={handleReset}
	class="btn variant-ringed-primary font-mono m-8 w-1/2 mx-auto"
>
	Reset game
</button>
