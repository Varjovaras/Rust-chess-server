<script lang="ts">
    import { returnCorrectPieceColor } from "$lib/components/chess/utils";
    import { animationStore } from "$lib/stores/animationStore";
    import type { Square } from "$lib/types";
    import { onMount } from "svelte";

    interface Props {
        sq: Square;
        isAnimating?: boolean;
        isSource?: boolean;
    }

    const { sq, isAnimating = false, isSource = false }: Props = $props();

    const pieceSize = $derived(`
        w-[5vw] h-[5vw]
        sm:w-[10vw] sm:h-[10vw]
        min-w-6 min-h-6
        max-w-16 max-h-16
        aspect-square
    `);

    let isDragging = $state(false);
    let pieceElement: HTMLImageElement;

    onMount(() => {
        // Add reference to the element for potential direct manipulation
        if (pieceElement) {
            pieceElement.dataset.file = sq.file;
            pieceElement.dataset.rank = sq.rank.toString();
        }
    });

    const handleDragStart = (event: DragEvent) => {
        isDragging = true;
        if (event.dataTransfer) {
            event.dataTransfer.effectAllowed = "move";
        }
    };

    const handleDragEnd = () => {
        isDragging = false;
    };

    // Enhanced animation class logic
    let animationClass = $state("");
    let customStyle = $state("");

    $effect(() => {
        if (isAnimating) {
            animationClass = isSource
                ? "source-animating"
                : "destination-animating";

            // Check if this is the moving piece in the animation store
            const animState = $animationStore;
            const squareId = `${sq.file.toLowerCase()}${sq.rank + 1}`;

            if (
                animState.isAnimating &&
                animState.fromSquare === squareId &&
                !isSource
            ) {
                // This is the piece that should be visually moving
                animationClass += " piece-moving";

                if (animState.startPosition && animState.endPosition) {
                    // Apply FLIP animation technique
                    const deltaX =
                        animState.endPosition.x - animState.startPosition.x;
                    const deltaY =
                        animState.endPosition.y - animState.startPosition.y;
                    customStyle = `--move-x: ${deltaX}px; --move-y: ${deltaY}px;`;
                }
            }
        } else {
            animationClass = "";
            customStyle = "";
        }
    });
</script>

<div class="w-full h-full flex items-center justify-center content-end">
    {#if sq.piece !== "None"}
        <img
            bind:this={pieceElement}
            class={`
                piece-image
                object-contain ${pieceSize}
                transition-all
                ${animationClass}
                ${isDragging ? "piece-dragging" : "can-drag"}
                hover:scale-105
            `}
            style={customStyle}
            src={returnCorrectPieceColor(sq.piece)}
            alt={`${sq.piece} piece`}
            draggable="true"
            ondragstart={handleDragStart}
            ondragend={handleDragEnd}
        />
    {/if}
</div>

<style>
    .piece-image {
        will-change: transform, opacity;
        transition: all 300ms cubic-bezier(0.4, 0, 0.2, 1);
        backface-visibility: hidden;
        transform-style: preserve-3d;
    }

    .source-animating {
        transform: scale(0.9);
        opacity: 0.7;
    }

    .destination-animating {
        animation: slideIn 300ms cubic-bezier(0.4, 0, 0.2, 1) forwards;
    }

    .piece-moving {
        position: absolute;
        z-index: 100;
        animation: moveToDestination 300ms cubic-bezier(0.4, 0, 0.2, 1) forwards;
    }

    .piece-dragging {
        opacity: 0.8;
        transform: scale(1.1);
        cursor: grabbing;
    }

    .can-drag {
        cursor: grab;
    }

    .can-drag:hover {
        transform: scale(1.05);
    }

    @keyframes slideIn {
        from {
            opacity: 0;
            transform: scale(0.95);
        }
        to {
            opacity: 1;
            transform: scale(1);
        }
    }

    @keyframes moveToDestination {
        from {
            transform: translate(0, 0) scale(1);
        }
        to {
            transform: translate(var(--move-x, 0px), var(--move-y, 0px))
                scale(1);
        }
    }
</style>
