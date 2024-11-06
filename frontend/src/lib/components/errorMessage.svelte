<script lang="ts">
    interface Props {
        errorMessage: string;
    }

    const { errorMessage }: Props = $props();
</script>

<div>
    {errorMessage}
</div>
