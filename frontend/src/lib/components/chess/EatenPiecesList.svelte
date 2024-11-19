<script lang="ts">
    import EatenPiece from "./eatenPiece.svelte";
    interface Props {
        color: "white" | "black";
        pieces: {
            pawns: number;
            knights: number;
            bishops: number;
            rooks: number;
            queens: number;
            kings: number;
        };
    }

    let { color, pieces }: Props = $props();
</script>

<ul
    class="flex flex-row m-4 p-2 variant-outline-primary content-center justify-center bg-surface-700"
>
    <li class="">
        <EatenPiece
            imgSrc="/pieces/{color}_queen.svg"
            alt="{color} queen"
            amount={pieces.queens || 0}
        />
    </li>
    <li class="">
        <EatenPiece
            imgSrc="/pieces/{color}_rook.svg"
            alt="{color} rook"
            amount={pieces.rooks || 0}
        />
    </li>
    <li class="">
        <EatenPiece
            imgSrc="/pieces/{color}_knight.svg"
            alt="{color} knight"
            amount={pieces.knights || 0}
        />
    </li>
    <li class="">
        <EatenPiece
            imgSrc="/pieces/{color}_bishop.svg"
            alt="{color} bishop"
            amount={pieces.bishops || 0}
        />
    </li>
    <li class="">
        <EatenPiece
            imgSrc="/pieces/{color}_pawn.svg"
            alt="{color} pawn"
            amount={pieces.pawns || 0}
        />
    </li>
</ul>
