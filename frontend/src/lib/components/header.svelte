<script lang="ts">
	import { AppBar } from '@skeletonlabs/skeleton';
</script>

<AppBar
	gridColumns="grid-cols-3"
	slotDefault="place-self-center"
	slotTrail="place-content-end"
>
	<svelte:fragment slot="lead">
		<img
			class="transform scale-x-[-1]"
			src="/pieces/white_knight.svg"
			alt="icon representing white horse"
		/>
	</svelte:fragment>
	<h1 class="h2">Chess app</h1>
	<svelte:fragment slot="trail">(actions)</svelte:fragment>
</AppBar>
