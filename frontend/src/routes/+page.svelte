<script lang="ts">
	import Chessboard from '$lib/components/chess/chessboard.svelte';
	import { startingPosition } from '$lib/components/chess/startingPosition';
	import ErrorMessage from '$lib/components/errorMessage.svelte';
	import ResetButton from '$lib/components/resetButton.svelte';
	import { type Chess, chessSchema } from '$lib/types';
	import { type ModalSettings, getModalStore } from '@skeletonlabs/skeleton';
	import type { PageData } from './$types';
	import { createWebSocketStore } from '$lib/websocketStore';
	import { onMount, onDestroy } from 'svelte';

	const modalStore = getModalStore();
	const ws = createWebSocketStore('ws://localhost:8000/websocket'); // biome-ignore lint/suspicious/noExplicitAny: <explanation>
	let messages: any[] = [];
	let isConnected = false;

	onMount(() => {
		const unsubscribe = ws.subscribe((socket) => {
			if (socket) {
				isConnected = true;
				socket.addEventListener('message', (event) => {
					try {
						const data = JSON.parse(event.data);
						if (data.chess) {
							// Update the chess state
							chess = chessSchema.parse(data.chess);

							// Check for victory conditions
							if (chess.white_player.victory) {
								modalStore.trigger(whiteModal);
							} else if (chess.black_player.victory) {
								modalStore.trigger(blackModal);
							}
						} else {
							// Handle other types of messages (e.g., client count, API status)
							messages = [...messages, data];
						}
					} catch (error) {
						console.error('Failed to parse WebSocket message:', error);
					}
				});
				socket.addEventListener('error', (event) => {
					console.error('WebSocket error:', event);
				});
			} else {
				isConnected = false;
			}
		});

		return () => unsubscribe();
	});

	function sendMessage() {
		ws.send('Hello, WebSocket!');
	}

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

	const handleMove = async (startSq: string, endSq: string) => {
		console.log(`Move from ${startSq} to ${endSq}`);
		const moveRequest = {
			list_of_moves: chess.list_of_moves,
			new_move: [startSq, endSq],
		};
		console.log(moveRequest);
		ws.send(JSON.stringify(moveRequest));
	};

	const handleReset = () => {
		console.log('Resetting game');
		chess = startingPosition;
	};
</script>

<div class="flex flex-col justify-center content-center py-4">
	<ErrorMessage {errorMessage} />
	<Chessboard {chess} {handleMove} />
	<ResetButton {handleReset} />
</div>
<button on:click={sendMessage}>Send Message</button>
<ul>
	{#each messages as message}
		<li>
			Clients: {message.clients_count}, Time: {new Date(
				message.dateTime,
			).toLocaleString()}, API Status: {message.is_up ? 'Up' : 'Down'}
		</li>
	{/each}
</ul>
<div>Connection status: {isConnected ? 'Connected' : 'Disconnected'}</div>
