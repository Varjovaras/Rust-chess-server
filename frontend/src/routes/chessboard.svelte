<script lang="ts">
	import Square from './square.svelte';
	import type { Chess, ChessBoard, Square as SquareType } from './types';
	export let chess: Chess;
	export let handleClick: (sq: SquareType) => void;
	export let handleMove: (startSq: string, endSq: string) => Promise<void>;
	let startSq = '';

	$: boardToFront = handleBoardToFront(chess.board);

	const handleBoardToFront = (chessboard: ChessBoard): ChessBoard => {
		let boardToFront: ChessBoard = [[]];
		for (let i = 7; i >= 0; i--) {
			let arr = [];
			for (let j = 7; j >= 0; j--) {
				arr.push(chessboard[j][i]);
			}
			boardToFront.push(arr.reverse());
		}
		return boardToFront;
	};

	const handleDragStart = (sq: SquareType) => {
		console.log('Drag started:', sq);
		startSq = sq.file.toLowerCase() + (sq.rank + 1);
	};

	const handleDrop = (event: DragEvent) => {
		console.log('Drop event:', event);
		const targetElement = event.target as HTMLElement;
		console.log(targetElement);
		console.log(targetElement.id);
		if (targetElement.id !== '[object Object]' && targetElement.id !== '') {
			handleMove(startSq, targetElement.id);
		}
	};

	// ...
</script>

<div class="flex justify-center items-center">
	<div class="grid grid-cols-8 gap-0">
		{#each boardToFront as row}
			{#each row as sq}
				{#if chess.white_player.in_check && typeof sq.piece === 'object' && sq.piece.King !== undefined && sq.piece.King === 'White'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-red-900 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
						draggable="true"
						on:dragstart={() => handleDragStart(sq)}
						on:dragover|preventDefault
						on:drop|preventDefault={handleDrop}
						id={sq.file + (sq.rank + 1)}
						on:click|preventDefault={() => handleClick(sq)}
					>
						<Square {sq} />
					</button>
				{:else if chess.black_player.in_check && typeof sq.piece === 'object' && sq.piece.King !== undefined && sq.piece.King === 'Black'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-red-900 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
						draggable="true"
						on:dragstart={() => handleDragStart(sq)}
						on:dragover|preventDefault
						on:drop|preventDefault={handleDrop}
						id={sq.file + (sq.rank + 1)}
						on:click|preventDefault={() => handleClick(sq)}
					>
						<Square {sq} />
					</button>
				{:else if sq.color === 'White'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-gray-200 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
						draggable="true"
						on:dragstart={() => handleDragStart(sq)}
						on:dragover|preventDefault
						on:drop|preventDefault={handleDrop}
						id={sq.file + (sq.rank + 1)}
						on:click|preventDefault={() => handleClick(sq)}
					>
						<Square {sq} />
					</button>
				{:else}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-gray-400 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
						draggable="true"
						on:dragstart={() => handleDragStart(sq)}
						on:dragover|preventDefault
						on:drop|preventDefault={handleDrop}
						id={sq.file + (sq.rank + 1)}
						on:click|preventDefault={() => handleClick(sq)}
					>
						<Square {sq} />
					</button>
				{/if}
			{/each}
		{/each}
	</div>
</div>
