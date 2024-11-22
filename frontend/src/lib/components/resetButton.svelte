<script lang="ts">
	interface Props {
		handleReset: () => void;
	}

	const { handleReset }: Props = $props();
</script>

<button
	onclick={handleReset}
	class="btn variant-ringed-primary font-mono m-4 w-1/2 mx-auto"
>
	Reset game
</button>
