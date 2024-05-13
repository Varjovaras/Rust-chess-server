<script lang="ts">
    import type { Chess } from "$lib/types";

    export let chess: Chess;
</script>

<div class="">
    {#if chess.white_player.victory}
        <h3 class="h3 text-red-500">White won</h3>
    {:else if chess.black_player.victory}
        <h3 class="h3 text-red-500">Black won</h3>
    {/if}
</div>
