<script lang="ts">
import { returnCorrectPieceColor } from "$lib/components/chess/utils";
import type { Square } from "$lib/types";
import { cubicOut } from "svelte/easing";
import { fade, fly } from "svelte/transition";

interface Props {
	sq: Square;
	isAnimating?: boolean;
	isSource?: boolean;
	isDestination?: boolean;
}

const {
	sq,
	isAnimating = false,
	isSource = false,
	isDestination = false,
} = $props();

const pieceSize = $derived(`
    w-[5vw] h-[5vw]
    sm:w-[10vw] sm:h-[10vw]
    min-w-6 min-h-6
    max-w-16 max-h-16
    aspect-square
  `);

let isDragging = $state(false);

const handleDragStart = (event: DragEvent) => {
	isDragging = true;
	if (event.dataTransfer) {
		event.dataTransfer.effectAllowed = "move";
	}
};

const handleDragEnd = () => {
	isDragging = false;
};

let animationClass = $state("");

$effect(() => {
	if (isAnimating) {
		animationClass = isSource ? "source-animating" : "destination-animating";
	} else {
		animationClass = "";
	}
});
</script>

<div class="w-full h-full flex items-center justify-center content-end">
{#if sq.piece !== 'None'}
    <img
        class={`
            piece-image
            object-contain ${pieceSize}
            transition-all
            ${animationClass}
            ${isDragging ? 'piece-dragging' : 'can-drag'}
            hover:scale-105
        `}
        src={returnCorrectPieceColor(sq.piece)}
        alt={`${sq.piece} piece`}
        draggable="true"
        ondragstart={handleDragStart}
        ondragend={handleDragEnd}
        style="object-fit: contain; width: 100%; height: 100%;"
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
</style>
