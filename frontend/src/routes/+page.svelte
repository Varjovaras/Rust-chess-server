<script lang="ts">
    import Chessboard from '$lib/components/chess/chessboard.svelte';
    import {startingPosition} from '$lib/components/chess/startingPosition';
    import ErrorMessage from '$lib/components/errorMessage.svelte';
    import ResetButton from '$lib/components/resetButton.svelte';
    import {type Chess, chessSchema} from '$lib/types';
    import type {PageData} from './$types';
    import {getModalStore, type ModalSettings} from '@skeletonlabs/skeleton';
    import TechStack from '$lib/components/techStack.svelte';

    const modalStore = getModalStore();

	const whiteModal: ModalSettings = {
		type: 'alert',
		// Data
		title: 'White won!',
		body: 'White won!',
	};

	const blackModal: ModalSettings = {
		type: 'alert',
		// Data
		title: 'Black won!',
		body: 'Black won!',
	};

	export let data: PageData;
	let chess = data.data.chess;
	const errorMessage = '';
	const apiUrl = data.data.url;

    // const removeHoverEffect = () => {
    //     const buttons = document.querySelectorAll('button');
    //     for (const button of buttons) {
    //         button.classList.remove('hover:bg-red-900', 'hover:bg-gray-600');
    //     }
    // };

	const handleMove = async (startSq: string, endSq: string) => {
		0;
		console.log(`Move from ${startSq} to ${endSq}`);
		try {
            chess = await fetchMove(startSq, endSq);
			if (chess.white_player.victory) {
				modalStore.trigger(whiteModal);
			} else if (chess.black_player.victory) {
				modalStore.trigger(blackModal);
			}
            // removeHoverEffect(); // Remove hover effect from all buttons after the move
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
		if (chess.board === validatedChess.board) {
			console.log('Move was not possible');
		}
		return validatedChess;
	};

	const handleReset = () => {
		console.log('Resetting game');
		chess = startingPosition;
	};
</script>

<ErrorMessage {errorMessage} />
<Chessboard {chess} {handleMove} />
<ResetButton {handleReset} />
<TechStack />
