<script lang="ts">
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
    // import WebsocketInfo from "$lib/components/websocketInfo.svelte";
    import { chessSchema } from "$lib/types";
    import { createWebSocketStore } from "$lib/websocketStore";
    import { type ModalSettings, getModalStore } from "@skeletonlabs/skeleton";
    import { onDestroy, onMount } from "svelte";
    import type { PageData } from "./$types";
    import WelcomeNotification from "$lib/components/WelcomeNotification.svelte";
    import GameInfo from "$lib/components/GameInfo.svelte";
    import { env } from "$env/dynamic/public";

    interface Props {
        data: PageData;
    }

    const { data }: Props = $props();
    console.log(`Status of backend: ${data.status}`);

    const modalStore = getModalStore();

    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    let websocketMessages: any[] = $state([]);
    let isConnected = $state(false);
    let chess = $state(data.startingPosition);
    const eatenPieces = $derived(chess.pieces_eaten);
    const piecesEatenCount = $derived(countEatenPieces(eatenPieces));

    // WebSocket store - initialized in onMount
    let ws: ReturnType<typeof createWebSocketStore>;

    onMount(() => {
        // Construct WebSocket URL based on environment
        const isDevMode = import.meta.env.DEV;
        let apiUrl: string;

        if (isDevMode) {
            // Development: connect directly to backend on localhost:8000
            apiUrl = "ws://localhost:8000/websocket";
        } else {
            // Production: use reverse proxy with relative path
            const protocol =
                window.location.protocol === "https:" ? "wss:" : "ws:";
            // Get the base path from the current URL pathname (e.g., /chess)
            const basePath = window.location.pathname.split("/")[1];
            const basePathPrefix = basePath ? `/${basePath}` : "";
            apiUrl = `${protocol}//${window.location.host}${basePathPrefix}/websocket`;
        }

        console.log(
            `Connecting to WebSocket: ${apiUrl} (dev mode: ${isDevMode})`,
        );

        ws = createWebSocketStore(apiUrl);

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
                        console.error(
                            "Failed to parse WebSocket message:",
                            error,
                        );
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
        if (isConnected && ws) {
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
        console.log(startSq);
        console.log(endSq);
        const fromSquare = getSquareFromString(startSq, chess);
        const toSquare = getSquareFromString(endSq, chess);

        if (!fromSquare || !toSquare) return;

        // Check if move is valid
        if (
            fromSquare.possible_moves &&
            isInPossibleMoves(fromSquare, toSquare, fromSquare.possible_moves)
        ) {
            let promotionPiece = [0, 0];
            if (isPawnPromotion(fromSquare, endSq)) {
                promotionPiece = getPromotionPiece(fromSquare.rank, endSq[1]);
            }

            const moveRequest = {
                list_of_moves: chess.list_of_moves,
                new_move: [startSq, endSq, promotionPiece],
            };

            // Send the move request to backend
            if (ws) {
                ws.send(JSON.stringify(moveRequest));
            }
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
        if (ws) {
            ws.send(JSON.stringify(resetRequest));
        }
    };

    //not implemented undo in backend
    // const handleUndoMove = async () => {
    //     if (isConnected) {
    //         // Send undo request to backend
    //         ws.send(JSON.stringify({ action: "undo" }));
    //     }
    // };
</script>

<div class="container mx-auto px-4 py-8 max-w-7xl min-h-[calc(100vh-200px)]">
    <div
        class="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-12 items-center h-full"
    >
        <div class="flex flex-col items-center justify-center space-y-4">
            <ErrorMessage {errorMessage} />
            <Chessboard {chess} {handleMove} />
            <WelcomeNotification />

            <!-- <EatenPiecesList color="white" pieces={piecesEatenCount.white} />
            <EatenPiecesList color="black" pieces={piecesEatenCount.black} /> -->
            <!-- {#if isDevMode && 2 < 1}
                <WebsocketInfo messages={websocketMessages} {isConnected} />
            {/if} -->
        </div>
        <GameInfo {isConnected} onReset={handleReset} />
    </div>
</div>
