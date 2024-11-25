<script lang="ts">
	import {
		handleBoardToFront,
		isWhiteTurn,
		legalMove,
	} from '$lib/components/chess/utils';
	import type {
		Chess,
		PiecesEaten,
		PossibleMoves,
		Square as SquareType,
	} from '../../types';
	import ChessSquare from './chessSquare.svelte';

	interface Props {
		chess: Chess;
		handleMove: (startSq: string, endSq: string) => Promise<void>;
		piecesEaten: PiecesEaten;
	}

	const { chess, handleMove, piecesEaten }: Props = $props();

	let startSq = '';
	let selectedButton: string | null = $state(null);
	let fromSquare = '';
	let toSquare = '';
	let possibleMoves: PossibleMoves = $state([]);

	const boardToFront = $derived(handleBoardToFront(chess.board));
	const whiteTurn = $derived(isWhiteTurn(chess.turn_number));

	// $effect(() => console.log(chess.board[4][6]));

	const handleClick = async (sq: SquareType) => {
		const file = sq.file.toLowerCase();
		const rank = sq.rank + 1;
		const squareId = file + rank;

		if (!fromSquare) {
			if (sq.piece === 'None' || !legalMove(sq, whiteTurn)) {
				console.log(
					sq.piece === 'None'
						? `No piece on ${squareId}`
						: 'Wrong players turn',
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
		fromSquare = '';
		toSquare = '';
		selectedButton = null;
		possibleMoves = [];
	};

	const handleDragStart = (sq: SquareType) => {
		toSquare = '';
		if (sq.piece === 'None') {
			console.log(`No piece on ${sq.file}${sq.rank}`);
			return;
		}
		if (!legalMove(sq, whiteTurn)) {
			return;
		}
		const file = sq.file.toLowerCase();
		const rank = sq.rank + 1;
		const squareId = file + rank;
		selectedButton = squareId;
		possibleMoves = sq.possible_moves;
		startSq = squareId;
		disableScrolling();
	};

	const handleDrop = (event: DragEvent) => {
		event.preventDefault();
		const targetElement = event.target as HTMLElement;
		handleMove(startSq, targetElement.id[0] + targetElement.id[1]);
		resetSelection();
		enableScrolling();
	};

	const handleTouchStart = (event: TouchEvent, sq: SquareType) => {
		event.preventDefault();
		console.log('start handle touch');
		disableScrolling();
		handleDragStart(sq);
	};

	type TouchPosition = {
		x: number;
		y: number;
	};

	let lastKnownTouchPosition: null | TouchPosition = null;

	const handleTouchMove = (event: TouchEvent) => {
		event.preventDefault();
		lastKnownTouchPosition = {
			x: event.touches[0].clientX,
			y: event.touches[0].clientY,
		};
	};

	const handleTouchEnd = async (event: TouchEvent) => {
		event.preventDefault();
		if (lastKnownTouchPosition) {
			const targetElement = document.elementFromPoint(
				lastKnownTouchPosition.x,
				lastKnownTouchPosition.y,
			) as HTMLElement;
			const endSq = targetElement.id[0] + targetElement.id[1];
			if (endSq !== startSq) {
				await handleMove(startSq, endSq);
				resetSelection();
			}
		}
		lastKnownTouchPosition = null;
		enableScrolling();
	};

	const disableScrolling = () => {
		document.body.style.overflow = 'hidden';
	};

	const enableScrolling = () => {
		document.body.style.overflow = '';
	};
</script>

<div class="flex justify-center items-center py-16">
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
