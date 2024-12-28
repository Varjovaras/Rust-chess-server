<script lang="ts">
import { env } from "$env/dynamic/public";
import Chessboard from "$lib/components/chess/chessboard.svelte";
import { countEatenPieces } from "$lib/components/chess/eatenPieces";
import { startingPosition } from "$lib/components/chess/startingPosition";

import {
	getPromotionPiece,
	getSquareFromString,
	isInPossibleMoves,
	isPawnPromotion,
} from "$lib/components/chess/utils";
import ErrorMessage from "$lib/components/errorMessage.svelte";
import ResetButton from "$lib/components/resetButton.svelte";
import WebsocketInfo from "$lib/components/websocketInfo.svelte";
import { animationStore } from "$lib/stores/animationStore";
import { type Square, chessSchema } from "$lib/types";
import { createWebSocketStore } from "$lib/websocketStore";
import { type ModalSettings, getModalStore } from "@skeletonlabs/skeleton";
import { onDestroy, onMount } from "svelte";
import type { PageData } from "./$types";

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
const piecesEatenCount = $derived(countEatenPieces(eatenPieces));

onMount(() => {
	const unsubscribe = ws.subscribe((socket) => {
		if (socket) {
			isConnected = true;
			socket.addEventListener("message", (event) => {
				try {
					const data = JSON.parse(event.data);
					if (data.type === "initial_state" && data.chess) {
						// Initialize the chess state
						chess = chessSchema.parse(data.chess);
					} else if (data.type === "update" && data.chess) {
						// Update the chess state
						chess = chessSchema.parse(data.chess);
						// Check for victory conditions
						if (chess.players[0].victory) {
							modalStore.trigger(whiteModal);
						} else if (chess.players[1].victory) {
							modalStore.trigger(blackModal);
						}
					} else if (data.type === "reset" && data.chess) {
						// Reset the chess state
						chess = chessSchema.parse(data.chess);
					} else {
						websocketMessages = [...websocketMessages, data];
					}
				} catch (error) {
					console.error("Failed to parse WebSocket message:", error);
					isConnected = false;
				}
			});
			console.log("Connected via websocket");
			socket.addEventListener("error", (event) => {
				console.error("WebSocket error:", event);
			});
		} else {
			isConnected = false;
		}
	});
	return () => unsubscribe();
});

onDestroy(() => {
	console.log(
		"Component is being destroyed, resetting chess to starting position",
	);
	if (isConnected) {
		ws.send(JSON.stringify({ action: "reset" }));
	}
	chess = startingPosition;
});

const whiteModal: ModalSettings = {
	type: "alert",
	title: "White won!",
	body: "White won!",
};

const blackModal: ModalSettings = {
	type: "alert",
	title: "Black won!",
	body: "Black won!",
};

const errorMessage = "";

const handleMove = async (startSq: string, endSq: string) => {
	const fromSquare = getSquareFromString(startSq, chess);
	const toSquare = getSquareFromString(endSq, chess);

	if (!fromSquare || !toSquare) return;

	// Start animation immediately if the move is in possible moves
	if (isInPossibleMoves(fromSquare, toSquare, fromSquare.possible_moves)) {
		animationStore.startAnimation(startSq, endSq, fromSquare.piece.toString());

		let promotionPiece = [0, 0];
		if (isPawnPromotion(fromSquare, endSq)) {
			promotionPiece = getPromotionPiece(fromSquare.rank, endSq[1]);
		}

		const moveRequest = {
			list_of_moves: chess.list_of_moves,
			new_move: [startSq, endSq, promotionPiece],
		};

		// Send the move request to backend
		ws.send(JSON.stringify(moveRequest));

		// Wait for animation to complete
		await new Promise((resolve) => setTimeout(resolve, 100));

		// End animation
		animationStore.endAnimation();
	} else {
		// Add invalid move animation
		const element = document.getElementById(startSq);
		if (element) {
			element.classList.add("invalid-move");
			setTimeout(() => {
				element.classList.remove("invalid-move");
			}, 100);
		}
	}
};

const handleReset = () => {
	console.log("Resetting game");
	chess = startingPosition;
	const resetRequest = {
		action: "reset",
	};
	ws.send(JSON.stringify(resetRequest));
};
</script>

<div class="flex flex-col items-center justify-center">
	<ErrorMessage {errorMessage} />
	<Chessboard {chess} {handleMove} />
	<ResetButton {handleReset} />
	<!-- <EatenPiecesList color="white" pieces={piecesEatenCount.white} />
	<EatenPiecesList color="black" pieces={piecesEatenCount.black} /> -->
	{#if isDevMode}
		<WebsocketInfo messages={websocketMessages} {isConnected} />
	{/if}
</div>
