<script lang="ts">
	import {
		handleBoardToFront,
		isWhiteTurn,
		legalMove,
	} from '$lib/components/chess/utils';
	import type { Chess, PossibleMoves, Square as SquareType } from '../../types';
	import ChessSquare from './chessSquare.svelte';

	export let chess: Chess;
	export let handleMove: (startSq: string, endSq: string) => Promise<void>;

	let startSq = '';
	let selectedButton: string | null = null;
	let fromSquare = '';
	let toSquare = '';
	let possibleMoves: PossibleMoves = [];

	$: boardToFront = handleBoardToFront(chess.board);
	$: whiteTurn = isWhiteTurn(chess.turn_number);

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
	};

	const handleDrop = (event: DragEvent) => {
		const targetElement = event.target as HTMLElement;
		handleMove(startSq, targetElement.id[0] + targetElement.id[1]);
		resetSelection();
	};

	const handleTouchStart = (_event: TouchEvent, sq: SquareType) => {
		handleDragStart(sq);
	};

	type TouchPosition = {
		x: number;
		y: number;
	};

	let lastKnownTouchPosition: null | TouchPosition = null;

	const handleTouchMove = (event: TouchEvent) => {
		lastKnownTouchPosition = {
			x: event.touches[0].clientX,
			y: event.touches[0].clientY,
		};
	};

	const handleTouchEnd = async (_event: TouchEvent) => {
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
	};
</script>

<div class="flex justify-center items-center p-4">
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
