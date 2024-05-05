<script lang="ts">
	import Chessboard from '$lib/components/chessboard.svelte';
	import GameOver from '$lib/components/gameOver.svelte';
	import Improvements from '$lib/components/improvements.svelte';
	import MoveGuide from '$lib/components/moveGuide.svelte';
	import ResetButton from '$lib/components/resetButton.svelte';
	import { startingPosition } from '$lib/startingPosition';
	import { chessSchema, type Chess } from '../lib/types';
	import type { PageData } from './$types';

	export let data: PageData;
	let chess = data.data.chess;
	const apiUrl = data.data.url;

	const handleMove = async (startSq: string, endSq: string) => {
		console.log(`Move from ${startSq} to ${endSq}`);
		try {
			const newChess = await fetchMove(startSq, endSq);
			chess = newChess;
		} catch (error) {
			console.error(error);
		}
	};

	const fetchMove = async (startSq: string, endSq: string): Promise<Chess> => {
		const newMove: [string, string] = [startSq, endSq];
		const response = await fetch(`${apiUrl}/api/chess`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify({
				list_of_moves: chess.list_of_moves,
				new_move: newMove,
			}),
		});
		const data = await response.json();
		const validatedChess = chessSchema.parse(data.chess);
		return validatedChess;
	};

	const handleReset =
		// async
		() => {
			try {
				console.log('Resetting game');
				chess = startingPosition;
			} catch (error) {
				console.error(error);
			}
		};
</script>

<MoveGuide />
<GameOver {chess} />
<Chessboard {chess} {handleMove} />
<ResetButton {handleReset} />
<Improvements />
