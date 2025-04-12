<script lang="ts">
    import {
        getSquareFromString,
        handleBoardToFront,
        isPossibleToMovePiece,
        isWhiteTurn,
    } from "$lib/components/chess/utils";
    import { animationStore } from "$lib/stores/animationStore";
    import { onDestroy, onMount } from "svelte";

    import type {
        Chess,
        PossibleMoves,
        Square,
        Square as SquareType,
    } from "../../types";
    import ChessSquare from "./chessSquare.svelte";

    interface Props {
        chess: Chess;
        handleMove: (startSq: string, endSq: string) => Promise<void>;
        // undoLastMove?: () => Promise<void>;
    }

    const {
        chess,
        handleMove,

        // undoLastMove = undefined
    }: Props = $props();

    let startSq = "";
    let selectedButton: string | null = $state(null);
    let currentDragStartSquare = "";
    let fromSquare = "";
    let toSquare = "";
    let possibleMoves: PossibleMoves = $state([]);
    let lastTouchX = 0;
    let lastTouchY = 0;
    const touchMoveThreshold = 10; // Minimum distance to consider a touch movement
    let movingPieceElement: HTMLElement | null = null;

    // Keep track of moves for undo feature
    let moveHistory = $state<string[][]>([]);

    const boardToFront = $derived(handleBoardToFront(chess.board));
    const whiteTurn = $derived(isWhiteTurn(chess.turn_number));

    // We can subscribe to animation state here if needed
    const animationState = animationStore.subscribe((state) => {
        // You can use this to coordinate board-wide animation effects
    });

    onMount(() => {
        // if (undoLastMove) {
        //     const handleKeyDown = (event: KeyboardEvent) => {
        //         if ((event.ctrlKey || event.metaKey) && event.key === 'z') {
        //             event.preventDefault();
        //             undoLastMove();
        //         }
        //     };
        //     window.addEventListener('keydown', handleKeyDown);
        //     return () => {
        //         window.removeEventListener('keydown', handleKeyDown);
        //     };
        // }
    });

    onDestroy(() => {
        animationState(); // Cleanup subscription
    });

    const handleClick = async (sq: SquareType) => {
        const file = sq.file.toLowerCase();
        const rank = sq.rank + 1;
        const squareId = file + rank;

        if (!fromSquare) {
            // First click - select a piece
            if (sq.piece === "None" || !isPossibleToMovePiece(sq, whiteTurn)) {
                // Invalid selection animation
                showInvalidMoveAnimation(squareId);
                resetSelection();
                return;
            }
            fromSquare = squareId;
            selectedButton = squareId;
            possibleMoves = sq.possible_moves;
        } else {
            // Second click - attempt move
            toSquare = squareId;

            // Check if move is in possible moves list before sending to backend
            const isValidMove = checkValidMove(
                fromSquare,
                toSquare,
                possibleMoves,
            );

            if (isValidMove) {
                // Start frontend animation immediately
                const pieceImg = document.querySelector(`#${fromSquare} img`);

                // Start animation before sending to backend
                animationStore.startAnimation(
                    fromSquare,
                    toSquare,
                    typeof sq.piece === "object"
                        ? JSON.stringify(sq.piece)
                        : "None",
                    pieceImg as HTMLElement,
                );

                // Add move to history for potential undo
                moveHistory = [...moveHistory, [fromSquare, toSquare]];

                // Wait a small delay before sending move to backend
                // This allows the animation to start visually first
                setTimeout(async () => {
                    await handleMove(fromSquare, toSquare);
                }, 50);
            } else {
                showInvalidMoveAnimation(toSquare);
            }
            resetSelection();
        }
    };

    const checkValidMove = (
        from: string,
        to: string,
        moves: PossibleMoves,
    ): boolean => {
        const toFile = to.charAt(0).toUpperCase().charCodeAt(0) - 65; // Convert 'a' to 0, 'b' to 1, etc.
        const toRank = Number.parseInt(to.charAt(1)) - 1;

        return moves.some(
            (move) => move[1][0] === toFile && move[1][1] === toRank,
        );
    };

    const showInvalidMoveAnimation = (squareId: string) => {
        const squareElement = document.getElementById(squareId);
        if (squareElement) {
            squareElement.classList.add("invalid-move");
            setTimeout(() => {
                squareElement.classList.remove("invalid-move");
            }, 300);
        }
    };

    const handleDrop = async (event: DragEvent) => {
        event.preventDefault();

        // Find the closest button (square) element
        const targetElement = event.target as HTMLElement;
        const squareElement = targetElement.closest("button");

        if (!squareElement) return;

        const endSq = squareElement.id;

        // Use the stable currentDragStartSquare instead of possibly-reset startSq
        const effectiveStartSq = currentDragStartSquare || startSq;

        // Add debug logging
        console.log("Drop on square:", endSq);
        console.log("Start square was:", effectiveStartSq);

        if (effectiveStartSq && endSq && effectiveStartSq !== endSq) {
            // Check if this is a valid move before starting animation
            const startSquare = getSquareFromString(effectiveStartSq, chess);
            const toSquareCoords = {
                file: endSq[0].toUpperCase(),
                rank: Number.parseInt(endSq[1]) - 1,
            };

            if (!startSquare) {
                console.error(
                    "Could not find start square for:",
                    effectiveStartSq,
                );
                resetSelection();
                return;
            }

            const isValid = startSquare.possible_moves.some(
                (move) =>
                    move[1][0] === toSquareCoords.file.charCodeAt(0) - 65 &&
                    move[1][1] === toSquareCoords.rank,
            );

            if (isValid) {
                // Start animation immediately
                animationStore.startAnimation(
                    effectiveStartSq,
                    endSq,
                    typeof startSquare.piece === "object"
                        ? JSON.stringify(startSquare.piece)
                        : "None",
                    movingPieceElement as unknown as HTMLElement,
                );

                // Add move to history for potential undo
                moveHistory = [...moveHistory, [effectiveStartSq, endSq]];

                // Small delay for backend sync
                setTimeout(async () => {
                    // IMPORTANT: Use our stable variable instead of possibly-reset startSq
                    await handleMove(effectiveStartSq, endSq);
                }, 50);
            } else {
                showInvalidMoveAnimation(effectiveStartSq);
            }
        } else {
            console.warn("Invalid drag-drop operation", {
                startSq: effectiveStartSq,
                endSq,
            });
        }

        // Reset after processing the move
        resetSelection();
        currentDragStartSquare = ""; // Clear the drag state
        movingPieceElement = null;
    };

    const handleTouchStart = (event: TouchEvent, sq: Square) => {
        // Prevent scrolling when starting a chess move
        event.preventDefault();

        const file = sq.file.toLowerCase();
        const rank = sq.rank + 1;
        const squareId = file + rank;

        // If no piece is currently selected and this is a valid piece to move
        if (
            !fromSquare &&
            sq.piece !== "None" &&
            isPossibleToMovePiece(sq, whiteTurn)
        ) {
            fromSquare = squareId;
            selectedButton = squareId;
            possibleMoves = sq.possible_moves;

            // Store the touch start position
            lastTouchX = event.touches[0].clientX;
            lastTouchY = event.touches[0].clientY;

            // Store the element being touched for animation
            const targetElement = event.target as HTMLElement;
            if (targetElement) {
                const button = targetElement.closest("button");
                if (button) {
                    const pieceImg = button.querySelector("img");
                    if (pieceImg) {
                        movingPieceElement = pieceImg as HTMLElement;
                    }
                }
            }
        }
    };

    const handleTouchMove = (event: TouchEvent) => {
        // Prevent scrolling during drag
        event.preventDefault();

        // Optional: You could implement a visual piece follow the finger
        // by tracking touch position changes and updating a floating element
    };

    const handleTouchEnd = async (event: TouchEvent) => {
        if (!fromSquare) return; // No move in progress

        const touch = event.changedTouches[0];
        const elementAtPoint = document.elementFromPoint(
            touch.clientX,
            touch.clientY,
        );
        if (!elementAtPoint) return;

        const destinationSquare = elementAtPoint.closest("button");
        if (!destinationSquare) return;

        const endSq = destinationSquare.id;
        const effectiveStartSq = fromSquare; // Use fromSquare as it's more stable

        console.log("Touch end on square:", endSq, "from:", effectiveStartSq);

        if (effectiveStartSq && endSq && effectiveStartSq !== endSq) {
            const isValidMove = checkValidMove(
                effectiveStartSq,
                endSq,
                possibleMoves,
            );

            if (isValidMove) {
                animationStore.startAnimation(
                    effectiveStartSq,
                    endSq,
                    "piece",
                    movingPieceElement as unknown as HTMLSelectElement,
                );

                moveHistory = [...moveHistory, [effectiveStartSq, endSq]];

                setTimeout(async () => {
                    await handleMove(effectiveStartSq, endSq);
                }, 50);
            } else {
                showInvalidMoveAnimation(effectiveStartSq);
            }
        } else {
            console.warn("Invalid touch operation", {
                fromSquare: effectiveStartSq,
                endSq,
            });
        }

        resetSelection();
        movingPieceElement = null;
    };

    // Add the undo move function
    // const handleUndoMove = async () => {
    //     if (undoLastMove && moveHistory.length > 0) {
    //         await undoLastMove();
    //         // Remove the last move from history
    //         moveHistory = moveHistory.slice(0, -1);
    //     }
    // };

    const resetSelection = () => {
        fromSquare = "";
        toSquare = "";
        selectedButton = null;
        possibleMoves = [];
    };
    const handleDragStart = (sq: Square, event: DragEvent) => {
        // Don't prevent default here - we want the default drag behavior

        const file = sq.file.toLowerCase();
        const rank = sq.rank + 1;
        const squareId = file + rank;

        // Add debug logging
        console.log("Drag start from square:", squareId);

        // Only allow dragging pieces that can be moved based on current turn
        if (sq.piece === "None" || !isPossibleToMovePiece(sq, whiteTurn)) {
            event.preventDefault();
            showInvalidMoveAnimation(squareId);
            return;
        }

        // Set the drag data
        if (event.dataTransfer) {
            // Using the square ID as data format - make sure this is consistent
            event.dataTransfer.setData("text/plain", squareId);
            event.dataTransfer.effectAllowed = "move";

            // Create a custom drag image (optional)
            const pieceImg = event.target as HTMLElement;
            if (pieceImg) {
                // Create a prettier drag image if desired
                const dragImage = pieceImg.cloneNode(true) as HTMLElement;
                dragImage.classList.add("drag-image");
                document.body.appendChild(dragImage);

                // Fix: Use numeric values instead of accessing width/height properties
                const rect = pieceImg.getBoundingClientRect();
                event.dataTransfer.setDragImage(
                    dragImage,
                    rect.width / 2,
                    rect.height / 2,
                );

                // Clean up the drag image after drag starts
                setTimeout(() => {
                    if (dragImage.parentNode) {
                        dragImage.parentNode.removeChild(dragImage);
                    }
                }, 0);
            }
        }

        // Store the starting square information - ensure these are properly set
        fromSquare = squareId;
        selectedButton = squareId;
        possibleMoves = sq.possible_moves;
        startSq = squareId; // This is critical

        // IMPORTANT: Store in a separate variable that won't be reset
        currentDragStartSquare = squareId;

        // Store the element being dragged for animation
        movingPieceElement = event.target as HTMLElement;
    };
</script>

<div class="flex justify-center items-center py-8 touch-none">
    <div class="flex flex-col justify-center items-center">
        <div class="grid grid-cols-8 gap-0">
            {#each boardToFront as row, i}
                {#each row as sq, j}
                    <ChessSquare
                        {chess}
                        {sq}
                        {selectedButton}
                        {possibleMoves}
                        {handleClick}
                        {handleDragStart}
                        {handleDrop}
                        {handleTouchStart}
                        {handleTouchMove}
                        {handleTouchEnd}
                    />
                {/each}
            {/each}
        </div>

        <!-- {#if undoLastMove}
            <div class="mt-4">
                <button
                    class="btn variant-ghost-primary"
                    on:click={handleUndoMove}
                    disabled={moveHistory.length === 0}
                >
                    Undo Last Move
                </button>
            </div>
        {/if} -->
    </div>
</div>
