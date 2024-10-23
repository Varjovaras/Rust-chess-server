<script lang="ts">
	import Chessboard from '$lib/components/chess/chessboard.svelte';
	import { startingPosition } from '$lib/components/chess/startingPosition';
	import ErrorMessage from '$lib/components/errorMessage.svelte';
	import ResetButton from '$lib/components/resetButton.svelte';
	import { chessSchema, type Chess } from '$lib/types';
	import { type ModalSettings, getModalStore } from '@skeletonlabs/skeleton';
	import { createWebSocketStore } from '$lib/websocketStore';
	import { onMount, onDestroy } from 'svelte';
	import { env } from '$env/dynamic/public';
	import WebsocketInfo from '$lib/components/websocketInfo.svelte';
	import type { PageData } from './$types';
	import type {
		InitialStateMessage,
		ResetMessage,
		UpdateMessage,
	} from '$lib/websocketTypes';

	let lastSentTime = 0;

	const isDevMode = import.meta.env.DEV;
	const apiUrl = isDevMode ? env.PUBLIC_DEV_WS_URL : env.PUBLIC_PROD_WS_URL;
	export let data: PageData;
	console.log(`Status of backend: ${data.status}`);

	const modalStore = getModalStore();
	const ws = createWebSocketStore(apiUrl);

	// biome-ignore lint/suspicious/noExplicitAny: <explanation>
	let websocketMessages: any[] = [];
	let isConnected = false;
	let chess = data.startingPosition;
	$: eatenPieces = chess.pieces_eaten;

	const setupWebSocket = (socket: WebSocket) => {
		isConnected = true;
		socket.addEventListener('message', (event) =>
			handleWebSocketMessage(event),
		);
		console.log('Connected via websocket');
		socket.addEventListener('error', (event) => {
			console.error('WebSocket error:', event);
		});
	};

	const handleInitialState = (_data: InitialStateMessage) => {
		if (data.startingPosition) {
			chess = chessSchema.parse(data.startingPosition);
		}
	};

	const handleUpdate = (data: UpdateMessage) => {
		if (data.chess) {
			const parsedChess = chessSchema.parse(data.chess);
			chess = parsedChess;
			checkVictoryConditions(parsedChess);
		}
	};

	const handleResetMessage = (data: ResetMessage) => {
		if (data.chess) {
			chess = chessSchema.parse(data.chess);
		}
	};

	const checkVictoryConditions = (chess: Chess) => {
		if (chess.players[0].victory) {
			modalStore.trigger(whiteModal);
		} else if (chess.players[1].victory) {
			modalStore.trigger(blackModal);
		}
	};

	// biome-ignore lint/suspicious/noExplicitAny: <explanation>
	const handleWebSocketMessage = (event: MessageEvent<any>) => {
		const receiveTime = performance.now();
		const duration = receiveTime - lastSentTime;

		// console.log('Last sent time:', lastSentTime);
		// console.log('Receive time:', receiveTime);
		console.log('Round trip duration in ms:', duration);

		try {
			const data = JSON.parse(event.data);

			switch (data.type) {
				case 'initial_state':
					handleInitialState(data);
					break;
				case 'update':
					handleUpdate(data);
					break;
				case 'reset':
					handleResetMessage(data);
					break;
				default:
					websocketMessages = [...websocketMessages, data];
			}
		} catch (error) {
			console.error('Failed to parse WebSocket message:', error);
			isConnected = false;
		}
	};

	onMount(() => {
		const unsubscribe = ws.subscribe((socket) => {
			if (socket) {
				setupWebSocket(socket);
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
		lastSentTime = performance.now();
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
