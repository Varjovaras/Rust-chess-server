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
</script>

<div class="w-full h-full flex items-center justify-center content-end">
  {#if sq.piece !== 'None'}
    <img
      class={`
        piece-image
        object-contain ${pieceSize}
        transition-all duration-300
        ${isAnimating ? 'piece-moving' : ''}
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
    .piece-moving {
       will-change: transform;
       transition: transform 150ms cubic-bezier(0.4, 0, 0.2, 1),
                   opacity 150ms cubic-bezier(0.4, 0, 0.2, 1);
     }


     .piece-image {
       transition: all 150ms cubic-bezier(0.4, 0, 0.2, 1);
       pointer-events: auto;
       user-select: none;
       transform-origin: center;
     }


     .dragging {
       opacity: 0.8; /* Increase from 0.5 to 0.8 */
       transform: scale(1.1);
       transition: all 150ms ease-out;
     }

     .can-drag:hover {
       transform: scale(1.05);
       transition: transform 150ms ease-out;
     }
</style>
