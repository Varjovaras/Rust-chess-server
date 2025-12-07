<script lang="ts">
    import EatenPiece from "./eatenPiece.svelte";
    import { base } from "$app/paths";
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

<ul class="flex flex-row p-2 justify-start items-start w-full h-20">
    <li class="w-10">
        <EatenPiece
            imgSrc="{base}/pieces/{color}_queen.svg"
            alt="{color} queen"
            amount={pieces.queens || 0}
        />
    </li>
    <li class="w-10">
        <EatenPiece
            imgSrc="{base}/pieces/{color}_rook.svg"
            alt="{color} rook"
            amount={pieces.rooks || 0}
        />
    </li>
    <li class="w-10">
        <EatenPiece
            imgSrc="{base}/pieces/{color}_knight.svg"
            alt="{color} knight"
            amount={pieces.knights || 0}
        />
    </li>
    <li class="w-10">
        <EatenPiece
            imgSrc="{base}/pieces/{color}_bishop.svg"
            alt="{color} bishop"
            amount={pieces.bishops || 0}
        />
    </li>
    <li class="w-10">
        <EatenPiece
            imgSrc="{base}/pieces/{color}_pawn.svg"
            alt="{color} pawn"
            amount={pieces.pawns || 0}
        />
    </li>
</ul>
