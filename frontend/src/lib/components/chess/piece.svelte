<script lang="ts">
    import { returnCorrectPieceColor } from "$lib/components/chess/utils";
    import type { Square } from "$lib/types";
    import {
        PIECE_MOVE_DURATION,
        PIECE_ANIMATION_EASING,
    } from "$lib/constants/animation";
    import { base } from "$app/paths";

    const animationStyle = `--piece-move-duration: ${PIECE_MOVE_DURATION}ms; --piece-animation-easing: ${PIECE_ANIMATION_EASING};`;

    interface Props {
        sq: Square;
        isAnimatingFrom?: boolean;
        isAnimatingTo?: boolean;
        onDragStart?: (event: DragEvent) => void;
    }

    const {
        sq,
        isAnimatingFrom = false,
        isAnimatingTo = false,
        onDragStart,
    }: Props = $props();

    const pieceSize = $derived(`
        w-[80%] h-[80%]
        sm:w-[80%] sm:h-[80%]
        md:w-[80%] md:h-[80%]
        aspect-square
    `);

    let isDragging = $state(false);

    const handleDragStart = (event: DragEvent) => {
        isDragging = true;
        if (event.dataTransfer) {
            event.dataTransfer.effectAllowed = "move";
        }
        if (onDragStart) {
            onDragStart(event);
        }
    };

    const handleDragEnd = () => {
        isDragging = false;
    };

    const animationClass = $derived(() => {
        if (isAnimatingFrom) return "animating-from";
        if (isAnimatingTo) return "animating-to";
        return "";
    });
</script>

<div
    class="w-full h-full flex items-center justify-center content-end pointer-events-none"
>
    {#if sq.piece !== "None"}
        <img
            class={`
                piece-image
                object-contain ${pieceSize}
                ${animationClass()}
                ${isDragging ? "piece-dragging" : "can-drag"}
                pointer-events-auto
            `}
            style={animationStyle}
            src={returnCorrectPieceColor(sq.piece, base)}
            alt={`${sq.piece} piece`}
            draggable={onDragStart ? "true" : "false"}
            ondragstart={handleDragStart}
            ondragend={handleDragEnd}
        />
    {/if}
</div>

<style>
    .piece-image {
        transition:
            transform var(--piece-move-duration, 200ms)
                var(--piece-animation-easing, ease-out),
            opacity var(--piece-move-duration, 200ms)
                var(--piece-animation-easing, ease-out);
    }

    .piece-image:not(.piece-dragging):hover {
        transform: scale(1.05);
    }

    .animating-from {
        opacity: 0;
        transform: scale(0.8);
    }

    .animating-to {
        animation: popIn var(--piece-move-duration, 200ms)
            var(--piece-animation-easing, ease-out);
    }

    .piece-dragging {
        opacity: 0.8;
        transform: scale(1.1);
        cursor: grabbing !important;
    }

    .can-drag {
        cursor: grab;
    }

    @keyframes popIn {
        0% {
            opacity: 0;
            transform: scale(0.8);
        }
        100% {
            opacity: 1;
            transform: scale(1);
        }
    }
</style>
