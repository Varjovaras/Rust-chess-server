<script lang="ts">
	import { z } from 'zod';
	import Chessboard from './chessboard.svelte';
	import type { ChessBoard } from './types';
	export let data;

	const board = data.post.board;
	let chessboard: ChessBoard = [[]];
	for (let i = 7; i >= 0; i--) {
		let arr = [];
		for (let j = 7; j >= 0; j--) {
			arr.push(board[j][i]);
		}
		chessboard.push(arr.reverse());
	}
</script>

<h1>Shakking And Sniping :D</h1>
<Chessboard {board} />

<!-- <p>{data.post.board[0][0]}</p> -->
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
