<script lang="ts">
	interface Props {
		fromSquare: string;
		toSquare: string;
	}

	let { fromSquare = $bindable(), toSquare = $bindable() }: Props = $props();
</script>

<form class="grid grid-cols-2 gap-4 mt-8">
	<label class="col-span-1"
		>''
		<span class="block">Move from:</span>
		<input type="text" class="block w-full" bind:value={fromSquare} />
	</label>
	<label class="col-span-1">
		<span class="block">Move to:</span>
		<input type="text" class="block w-full" bind:value={toSquare} />
	</label>
</form>
