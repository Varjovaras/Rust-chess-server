<script lang="ts">
	import Square from './square.svelte';
	import type { Chess, ChessBoard, Square as SquareType } from './types';
	export let chess: Chess;
	export let handleClick: (sq: SquareType) => void;

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

	const handleDragStart = (event: DragEvent, sq: SquareType) => {
		console.log(event);
		event.dataTransfer.setData('text/plain', JSON.stringify(sq));
	};

	const handleDragOver = (event) => {
		console.log(event);
		event.preventDefault();
	};

	const handleDrop = (event, sq) => {
		console.log(event);
		const data = JSON.parse(event.dataTransfer.getData('text/plain'));
		// Now you can use the data to update your chess board
	};

	const handleDragEnd = (event) => {
		console.log(event);
		event.dataTransfer.clearData();
	};
</script>

<div class="flex justify-center items-center">
	<div class="grid grid-cols-8 gap-0">
		{#each boardToFront as row}
			{#each row as sq}
				{#if chess.white_player.in_check && sq.color === 'White' && sq.piece !== 'None' && sq.piece.King === 'White'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-red-900 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
						on:click|preventDefault={() => handleClick(sq)}
						draggable="true"
						on:dragstart|preventDefault={(event) => handleDragStart(event, sq)}
						on:dragover|preventDefault={handleDragOver}
						on:drop|preventDefault={(event) => handleDrop(event, sq)}
						on:dragend|preventDefault={handleDragEnd}
					>
						<Square {sq} />
					</button>
				{:else if chess.black_player.in_check && sq.piece !== 'None' && sq.piece.King === 'Black'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-red-900 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
						on:click|preventDefault={() => handleClick(sq)}
						draggable="true"
						on:dragstart|preventDefault={(event) => handleDragStart(event, sq)}
						on:dragover|preventDefault={handleDragOver}
						on:drop|preventDefault={(event) => handleDrop(event, sq)}
						on:dragend|preventDefault={handleDragEnd}
					>
						<Square {sq} />
					</button>
				{:else if sq.color === 'White'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-gray-200 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
						on:click|preventDefault={() => handleClick(sq)}
						draggable="true"
						on:dragstart|preventDefault={(event) => handleDragStart(event, sq)}
						on:dragover|preventDefault={handleDragOver}
						on:drop|preventDefault={(event) => handleDrop(event, sq)}
						on:dragend|preventDefault={handleDragEnd}
					>
						<Square {sq} />
					</button>
				{:else}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-gray-400 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
						on:click|preventDefault={() => handleClick(sq)}
						draggable="true"
						on:dragstart|preventDefault={(event) => handleDragStart(event, sq)}
						on:dragover|preventDefault={handleDragOver}
						on:drop|preventDefault={(event) => handleDrop(event, sq)}
						on:dragend|preventDefault={handleDragEnd}
					>
						<Square {sq} />
					</button>
				{/if}
			{/each}
		{/each}
	</div>
</div>
