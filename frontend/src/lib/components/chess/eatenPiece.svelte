<script lang="ts">
    interface Props {
        imgSrc: string;
        alt: string;
        amount: number;
    }
    const { imgSrc, alt, amount }: Props = $props();
</script>

<div class="flex items-center gap-2 w-8">
    {#if amount > 0}
        <img src={imgSrc} {alt} class="w-10" />
        <span class="text-sm">{amount > 1 ? `×${amount}` : ""}</span>
    {/if}
</div>
