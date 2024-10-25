<script lang="ts">
    interface Props {
        imgSrc: string;
        alt: string;
        amount: number;
    }
    // biome-ignore lint/style/useConst: <explanation>
    let { imgSrc, alt, amount }: Props = $props();
</script>

<li class="flex flex-row justify-evenly">
    <img src={imgSrc} {alt} />
    {amount}
</li>
