<script lang="ts">
	interface Props {
		errorMessage: string;
	}

	const { errorMessage }: Props = $props();
</script>

<div class="hidden">
	{errorMessage}
</div>
