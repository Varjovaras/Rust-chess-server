<script lang="ts">
	interface Props {
		errorMessage: string;
	}

	let { errorMessage }: Props = $props();
</script>

<div>
	{errorMessage}
</div>
