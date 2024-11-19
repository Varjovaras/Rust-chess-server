<script lang="ts">
	import Chessboard from '$lib/components/chess/chessboard.svelte';
	import { startingPosition } from '$lib/components/chess/startingPosition';
	import ErrorMessage from '$lib/components/errorMessage.svelte';
	import ResetButton from '$lib/components/resetButton.svelte';
	import { chessSchema } from '$lib/types';
	import { type ModalSettings, getModalStore } from '@skeletonlabs/skeleton';
	import { createWebSocketStore } from '$lib/websocketStore';
	import { onMount, onDestroy } from 'svelte';
	import { env } from '$env/dynamic/public';
	import WebsocketInfo from '$lib/components/websocketInfo.svelte';
	import type { PageData } from './$types';

	interface Props {
		data: PageData;
	}

	const isDevMode = import.meta.env.DEV;
	const apiUrl = isDevMode ? env.PUBLIC_DEV_WS_URL : env.PUBLIC_PROD_WS_URL;
	const { data }: Props = $props();
	console.log(`Status of backend: ${data.status}`);

	const modalStore = getModalStore();
	const ws = createWebSocketStore(apiUrl);

	// biome-ignore lint/suspicious/noExplicitAny: <explanation>
	let websocketMessages: any[] = $state([]);
	let isConnected = $state(false);
	let chess = $state(data.startingPosition);
	const eatenPieces = $derived(chess.pieces_eaten);

	onMount(() => {
		const unsubscribe = ws.subscribe((socket) => {
			if (socket) {
				isConnected = true;
				socket.addEventListener('message', (event) => {
					try {
						const data = JSON.parse(event.data);
						if (data.type === 'initial_state' && data.chess) {
							// Initialize the chess state
							chess = chessSchema.parse(data.chess);
						} else if (data.type === 'update' && data.chess) {
							// Update the chess state
							chess = chessSchema.parse(data.chess);
							// Check for victory conditions
							if (chess.players[0].victory) {
								modalStore.trigger(whiteModal);
							} else if (chess.players[1].victory) {
								modalStore.trigger(blackModal);
							}
						} else if (data.type === 'reset' && data.chess) {
							// Reset the chess state
							chess = chessSchema.parse(data.chess);
						} else {
							websocketMessages = [...websocketMessages, data];
						}
					} catch (error) {
						console.error('Failed to parse WebSocket message:', error);
						isConnected = false;
					}
				});
				console.log('Connected via websocket');
				socket.addEventListener('error', (event) => {
					console.error('WebSocket error:', event);
				});
			} else {
				isConnected = false;
			}
		});
		return () => unsubscribe();
	});

	onDestroy(() => {
		console.log(
			'Component is being destroyed, resetting chess to starting position',
		);
		if (isConnected) {
			ws.send(JSON.stringify({ action: 'reset' }));
		}
		chess = startingPosition;
	});

	const whiteModal: ModalSettings = {
		type: 'alert',
		title: 'White won!',
		body: 'White won!',
	};

	const blackModal: ModalSettings = {
		type: 'alert',
		title: 'Black won!',
		body: 'Black won!',
	};

	const errorMessage = '';

	const handleMove = async (startSq: string, endSq: string) => {
		console.log(`Move from ${startSq} to ${endSq}`);
		const moveRequest = {
			list_of_moves: chess.list_of_moves,
			new_move: [startSq, endSq],
		};
		ws.send(JSON.stringify(moveRequest));
	};

	const handleReset = () => {
		console.log('Resetting game');
		chess = startingPosition;
		const resetRequest = {
			action: 'reset',
		};
		ws.send(JSON.stringify(resetRequest));
	};
</script>

<div class="flex flex-col justify-center content-center py-4">
	<ErrorMessage {errorMessage} />
	<Chessboard {chess} {handleMove} piecesEaten={eatenPieces} />
	<ResetButton {handleReset} />
	{#if isDevMode}
		<WebsocketInfo messages={websocketMessages} {isConnected} />
	{/if}
</div>
