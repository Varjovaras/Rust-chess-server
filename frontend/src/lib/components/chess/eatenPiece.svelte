<script lang="ts">
	interface Props {
		imgSrc: string;
		alt: string;
		amount: number;
	}
	//bug with making move where im not going to the right square, probably dragging
	const { imgSrc, alt, amount }: Props = $props();
</script>

<div class="flex flex-row">
	<div class="flex flex-col items-center gap-1 w-full">
		{#if amount > 0}
			<!-- <img src={imgSrc} {alt} class="w-10" /> -->
			<span class="text-sm font-serif">{amount > 1 ? `×${amount}` : ' '}</span>
		{:else}
			<div class="w-10"></div>
		{/if}
	</div>
</div>
