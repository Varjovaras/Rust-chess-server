<script lang="ts">
    export let imgSrc: string;
    export let alt: string;
    export let amount: number;
</script>

<li class="flex flex-row justify-evenly">
    <img src={imgSrc} {alt} />
    {amount}
</li>
