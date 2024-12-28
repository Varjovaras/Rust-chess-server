<script lang="ts">
	import {
		getSquareFromString,
		handleBoardToFront,
		isMoveLegal,
		isPossibleToMovePiece,
		isWhiteTurn,
	} from "$lib/components/chess/utils";
	import type {
		Chess,
		PossibleMoves,
		Square,
		Square as SquareType,
	} from "../../types";
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
			if (sq.piece === "None" || !isPossibleToMovePiece(sq, whiteTurn)) {
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
			event.dataTransfer?.clearData();
			return;
		}
		if (!isPossibleToMovePiece(sq, whiteTurn)) {
			event.dataTransfer?.clearData();
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

	const handleTouchStart = (event: TouchEvent, sq: Square) => {
		const file = sq.file.toLowerCase();
		const rank = sq.rank + 1;
		const squareId = file + rank;

		console.log("Touch start:", {
			piece: sq.piece,
			whiteTurn,
			isPossibleToMove: isPossibleToMovePiece(sq, whiteTurn),
			fromSquare,
		});

		// If no piece is currently selected
		if (!fromSquare) {
			// Only select if it's a valid piece to move
			if (sq.piece !== "None" && isPossibleToMovePiece(sq, whiteTurn)) {
				fromSquare = squareId;
				selectedButton = squareId;
				possibleMoves = sq.possible_moves;
			}
		}
		// If a piece is already selected
		else {
			// If clicking on the same square, deselect
			if (fromSquare === squareId) {
				resetSelection();
			}
			// If clicking on a different square, attempt to move
			else {
				toSquare = squareId;
				handleMove(fromSquare, toSquare);
				resetSelection();
			}
		}
	};

	const handleTouchMove = (event: TouchEvent) => {
		// Prevent default scrolling
		event.preventDefault();
	};

	const handleTouchEnd = async (event: TouchEvent) => {
		// // This can be left mostly empty or used for cleanup
		// if (fromSquare && toSquare) {
		// 	await handleMove(fromSquare, toSquare);
		// 	resetSelection();
		// }
	};
</script>

<div class="flex justify-center items-center py-8 touch-none">
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
