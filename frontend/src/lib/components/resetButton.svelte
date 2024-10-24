<script lang="ts">
	interface Props {
		handleReset: () => void;
	}

	let { handleReset }: Props = $props();
</script>

<button
	onclick={handleReset}
	class="btn variant-ringed-primary font-mono m-8 w-1/2 mx-auto"
>
	Reset game
</button>
