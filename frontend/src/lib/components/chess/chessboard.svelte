<script lang="ts">
	import {
		handleBoardToFront,
		isWhiteTurn,
		legalMove,
	} from "$lib/components/chess/utils";
	import type { Chess, PossibleMoves, Square as SquareType } from "../../types";
	import ChessSquare from "./chessSquare.svelte";

	interface Props {
		chess: Chess;
		handleMove: (startSq: string, endSq: string) => Promise<void>;
	}

	const { chess, handleMove }: Props = $props();

	let startSq = "";
	let selectedButton: string | null = $state(null);
	let fromSquare = "";
	let toSquare = "";
	let possibleMoves: PossibleMoves = $state([]);

	const boardToFront = $derived(handleBoardToFront(chess.board));
	const whiteTurn = $derived(isWhiteTurn(chess.turn_number));

	// $effect(() => console.log(chess.board[4][6]));

	const handleClick = async (sq: SquareType) => {
		const file = sq.file.toLowerCase();
		const rank = sq.rank + 1;
		const squareId = file + rank;

		if (!fromSquare) {
			if (sq.piece === "None" || !legalMove(sq, whiteTurn)) {
				console.log(
					sq.piece === "None"
						? `No piece on ${squareId}`
						: "Wrong players turn",
				);
				resetSelection();
				return;
			}
			fromSquare = squareId;
			selectedButton = squareId;
			possibleMoves = sq.possible_moves;
		} else {
			toSquare = squareId;
			await handleMove(fromSquare, toSquare);
			resetSelection();
		}
	};

	const resetSelection = () => {
		fromSquare = "";
		toSquare = "";
		selectedButton = null;
		possibleMoves = [];
	};

	const handleDragStart = (sq: SquareType, event: DragEvent) => {
		// event.preventDefault();
		if (sq.piece === "None") {
			console.log(`No piece on ${sq.file}${sq.rank}`);
			event.dataTransfer?.clearData(); // Prevent dragging
			return;
		}
		if (!legalMove(sq, whiteTurn)) {
			event.dataTransfer?.clearData(); // Prevent dragging
			return;
		}
		const file = sq.file.toLowerCase();
		const rank = sq.rank + 1;
		const squareId = file + rank;

		if (event.dataTransfer) {
			event.dataTransfer.setData("text/plain", squareId);
			event.dataTransfer.effectAllowed = "move";
		}

		selectedButton = squareId;
		possibleMoves = sq.possible_moves;
		startSq = squareId;
	};

	const handleDrop = (event: DragEvent) => {
		event.preventDefault();

		const targetElement = event.target as HTMLElement;
		const endSq = targetElement.id.slice(0, 2);

		if (startSq && endSq && startSq !== endSq) {
			handleMove(startSq, endSq);
		}

		resetSelection();
	};

	const handleTouchStart = (event: TouchEvent, sq: SquareType) => {
		// Prevent default to stop scrolling and other native behaviors
		event.preventDefault();

		// Ensure we're working with the button element
		const targetButton = event.currentTarget as HTMLButtonElement;

		if (sq.piece === "None" || !legalMove(sq, whiteTurn)) {
			console.log(
				sq.piece === "None"
					? `No piece on ${sq.file}${sq.rank}`
					: "Wrong player's turn",
			);
			resetSelection();
			return;
		}

		const file = sq.file.toLowerCase();
		const rank = sq.rank + 1;
		const squareId = file + rank;

		// Store initial touch position
		const touch = event.touches[0];
		const startX = touch.clientX;
		const startY = touch.clientY;

		let moveStarted = false;
		let touchMoveListener: ((event: TouchEvent) => void) | null = null;
		let touchEndListener: ((event: TouchEvent) => void) | null = null;

		touchMoveListener = (moveEvent: TouchEvent) => {
			const currentTouch = moveEvent.touches[0];
			const deltaX = Math.abs(currentTouch.clientX - startX);
			const deltaY = Math.abs(currentTouch.clientY - startY);

			// If movement is significant, start drag
			if (deltaX > 10 || deltaY > 10) {
				moveStarted = true;
				fromSquare = squareId;
				selectedButton = squareId;
				possibleMoves = sq.possible_moves;
				startSq = squareId;

				// Remove listeners
				if (touchMoveListener && touchEndListener) {
					targetButton.removeEventListener("touchmove", touchMoveListener);
					targetButton.removeEventListener("touchend", touchEndListener);
				}
			}
		};

		touchEndListener = (endEvent: TouchEvent) => {
			// Remove listeners
			if (touchMoveListener && touchEndListener) {
				targetButton.removeEventListener("touchmove", touchMoveListener);
				targetButton.removeEventListener("touchend", touchEndListener);
			}

			// If no move was started, perform click
			if (!moveStarted) {
				handleClick(sq);
			}
		};

		// Add event listeners
		targetButton.addEventListener("touchmove", touchMoveListener);
		targetButton.addEventListener("touchend", touchEndListener);
	};

	const handleTouchMove = (event: TouchEvent) => {
		// Prevent default scrolling
		event.preventDefault();
	};

	const handleTouchEnd = async (event: TouchEvent) => {
		if (startSq) {
			const touch = event.changedTouches[0];
			const targetElement = document.elementFromPoint(
				touch.clientX,
				touch.clientY,
			) as HTMLElement;

			if (targetElement?.id) {
				const endSq = targetElement.id.slice(0, 2);

				// Only move if start and end squares are different
				if (endSq && endSq !== startSq) {
					await handleMove(startSq, endSq);
				}
			}

			// Reset selection
			resetSelection();
		}
	};
</script>

<div class="flex justify-center items-center py-8">
	<div class="flex flex-col justify-center items-center">
		<div class="grid grid-cols-8 gap-0">
			{#each boardToFront as row, i}
				{#each row as sq, j}
					<ChessSquare
						{chess}
						{sq}
						{selectedButton}
						{possibleMoves}
						{handleClick}
						{handleDragStart}
						{handleDrop}
						{handleTouchStart}
						{handleTouchMove}
						{handleTouchEnd}
					/>
				{/each}
			{/each}
		</div>
	</div>
</div>
