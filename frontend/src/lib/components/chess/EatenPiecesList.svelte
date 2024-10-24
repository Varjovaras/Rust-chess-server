<script lang="ts">
    import EatenPiece from "./eatenPiece.svelte";
    export let color: "white" | "black";
    export let pieces: {
        pawns: number;
        knights: number;
        bishops: number;
        rooks: number;
        queens: number;
        kings: number;
    };
</script>

<div class="hidden md:flex flex-col items-start">
    <ul>
        <!-- //white pieces -->
        {#if pieces.queens && color === "white"}
            <EatenPiece
                imgSrc="/pieces/white_queen.svg"
                alt="white queen"
                amount={pieces.queens}
            />
        {/if}
        {#if pieces.rooks && color === "white"}
            <EatenPiece
                imgSrc="/pieces/white_rook.svg"
                alt="white rook"
                amount={pieces.rooks}
            />
        {/if}

        {#if pieces.knights && color === "white"}
            <EatenPiece
                imgSrc="/pieces/white_knight.svg"
                alt="white knight"
                amount={pieces.knights}
            />
        {/if}
        {#if pieces.bishops && color === "white"}
            <EatenPiece
                imgSrc="/pieces/white_bishop.svg"
                alt="white bishop"
                amount={pieces.bishops}
            />
        {/if}
        {#if pieces.pawns && color === "white"}
            <EatenPiece
                imgSrc="/pieces/white_pawn.svg"
                alt="white pawn"
                amount={pieces.pawns}
            />
        {/if}

        <!-- //black pieces -->
        {#if pieces.queens && color === "black"}
            <EatenPiece
                imgSrc="/pieces/black_queen.svg"
                alt="black queen"
                amount={pieces.queens}
            />
        {/if}
        {#if pieces.rooks && color === "black"}
            <EatenPiece
                imgSrc="/pieces/black_rook.svg"
                alt="black rook"
                amount={pieces.rooks}
            />
        {/if}
        {#if pieces.bishops && color === "black"}
            <EatenPiece
                imgSrc="/pieces/black_bishop.svg"
                alt="black bishop"
                amount={pieces.bishops}
            />
        {/if}
        {#if pieces.knights && color === "black"}
            <EatenPiece
                imgSrc="/pieces/black_knight.svg"
                alt="black knight"
                amount={pieces.knights}
            />
        {/if}
        {#if pieces.pawns && color === "black"}
            <EatenPiece
                imgSrc="/pieces/black_pawn.svg"
                alt="black pawn"
                amount={pieces.pawns}
            />
        {/if}
    </ul>
</div>
