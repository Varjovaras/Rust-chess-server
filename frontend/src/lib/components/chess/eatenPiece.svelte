<script lang="ts">
    interface Props {
        imgSrc: string;
        alt: string;
        amount: number;
    }
    const { imgSrc, alt, amount }: Props = $props();
</script>

<li class="flex flex-row justify-evenly">
    <img src={imgSrc} {alt} />
    {amount}
</li>
