<script lang="ts">
	export let handleReset: () => void;
</script>

<button on:click={handleReset} class="btn variant-ringed-primary font-mono">
	Reset game
</button>
