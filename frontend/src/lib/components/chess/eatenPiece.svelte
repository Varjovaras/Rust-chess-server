<script lang="ts">
    interface Props {
        imgSrc: string;
        alt: string;
        amount: number;
    }
    //bug with making move where im not going to the right square, probably dragging
    const { imgSrc, alt, amount }: Props = $props();
</script>

<div class="flex flex-row justify-center items-center">
    <div class="flex flex-col items-center gap-1">
        {#if amount > 0}
            <img src={imgSrc} {alt} class="h-12" />
            <span class="text-sm font-serif"
                >{amount > 1 ? `×${amount}` : ""}</span
            >
        {:else}
            <div class="h-12"></div>
        {/if}
    </div>
</div>
