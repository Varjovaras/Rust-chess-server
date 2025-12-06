<script lang="ts">
    import {
        getSquareFromString,
        handleBoardToFront,
        isPossibleToMovePiece,
        isWhiteTurn,
        isInPossibleMoves,
    } from "$lib/components/chess/utils";
    import { onMount } from "svelte";

    import type {
        Chess,
        PossibleMoves,
        Square,
        Square as SquareType,
    } from "../../types";
    import ChessSquare from "./chessSquare.svelte";
    import { PIECE_MOVE_DURATION } from "$lib/constants/animation";

    interface Props {
        chess: Chess;
        handleMove: (startSq: string, endSq: string) => Promise<void>;
    }

    const { chess, handleMove }: Props = $props();

    let startSq = "";
    let selectedButton: string | null = $state(null);
    let currentDragStartSquare = "";
    let fromSquare = "";
    let toSquare = "";
    let possibleMoves: PossibleMoves = $state([]);
    let lastTouchX = 0;
    let lastTouchY = 0;
    const touchMoveThreshold = 10;
    let movingPieceElement: HTMLElement | null = null;

    // Animation state
    let animatingFrom = $state("");
    let animatingTo = $state("");

    // Keep track of moves for undo feature
    let moveHistory = $state<string[][]>([]);

    const boardToFront = $derived(handleBoardToFront(chess.board));
    const whiteTurn = $derived(isWhiteTurn(chess.turn_number));

    onMount(() => {
        // Setup code if needed
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

            const fromSq = getSquareFromString(fromSquare, chess);
            if (!fromSq) {
                resetSelection();
                return;
            }

            // Check if move is valid using isInPossibleMoves
            const isValidMove = isInPossibleMoves(fromSq, sq, possibleMoves);

            if (isValidMove) {
                // Trigger CSS animation
                animatingFrom = fromSquare;
                animatingTo = toSquare;

                // Add move to history
                moveHistory = [...moveHistory, [fromSquare, toSquare]];

                // Send move to backend
                await handleMove(fromSquare, toSquare);

                // Clear animation after it completes
                setTimeout(() => {
                    animatingFrom = "";
                    animatingTo = "";
                }, PIECE_MOVE_DURATION);
            } else {
                showInvalidMoveAnimation(toSquare);
            }
            resetSelection();
        }
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

        const targetElement = event.target as HTMLElement;
        const squareElement = targetElement.closest("button");

        if (!squareElement) return;

        const endSq = squareElement.id;
        const effectiveStartSq = currentDragStartSquare || startSq;

        console.log("Drop on square:", endSq);
        console.log("Start square was:", effectiveStartSq);

        if (effectiveStartSq && endSq && effectiveStartSq !== endSq) {
            const startSquare = getSquareFromString(effectiveStartSq, chess);
            const endSquare = getSquareFromString(endSq, chess);

            if (!startSquare || !endSquare) {
                console.error(
                    "Could not find squares for:",
                    effectiveStartSq,
                    endSq,
                );
                resetSelection();
                return;
            }

            // Check if move is valid using isInPossibleMoves
            const isValid = isInPossibleMoves(
                startSquare,
                endSquare,
                startSquare.possible_moves,
            );

            if (isValid) {
                // Trigger CSS animation
                animatingFrom = effectiveStartSq;
                animatingTo = endSq;

                moveHistory = [...moveHistory, [effectiveStartSq, endSq]];

                // Send move to backend
                await handleMove(effectiveStartSq, endSq);

                // Clear animation after it completes
                setTimeout(() => {
                    animatingFrom = "";
                    animatingTo = "";
                }, PIECE_MOVE_DURATION);
            } else {
                showInvalidMoveAnimation(effectiveStartSq);
            }
        } else {
            console.warn("Invalid drag-drop operation", {
                startSq: effectiveStartSq,
                endSq,
            });
        }

        resetSelection();
        currentDragStartSquare = "";
        movingPieceElement = null;
    };

    const handleTouchStart = (event: TouchEvent, sq: Square) => {
        event.preventDefault();

        const file = sq.file.toLowerCase();
        const rank = sq.rank + 1;
        const squareId = file + rank;

        if (
            !fromSquare &&
            sq.piece !== "None" &&
            isPossibleToMovePiece(sq, whiteTurn)
        ) {
            fromSquare = squareId;
            selectedButton = squareId;
            possibleMoves = sq.possible_moves;

            lastTouchX = event.touches[0].clientX;
            lastTouchY = event.touches[0].clientY;

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
        event.preventDefault();
    };

    const handleTouchEnd = async (event: TouchEvent) => {
        if (!fromSquare) return;

        const touch = event.changedTouches[0];
        const elementAtPoint = document.elementFromPoint(
            touch.clientX,
            touch.clientY,
        );
        if (!elementAtPoint) return;

        const destinationSquare = elementAtPoint.closest("button");
        if (!destinationSquare) return;

        const endSq = destinationSquare.id;
        const effectiveStartSq = fromSquare;

        console.log("Touch end on square:", endSq, "from:", effectiveStartSq);

        if (effectiveStartSq && endSq && effectiveStartSq !== endSq) {
            const startSquare = getSquareFromString(effectiveStartSq, chess);
            const endSquare = getSquareFromString(endSq, chess);

            if (!startSquare || !endSquare) {
                resetSelection();
                return;
            }

            // Check if move is valid using isInPossibleMoves
            const isValid = isInPossibleMoves(
                startSquare,
                endSquare,
                possibleMoves,
            );

            if (isValid) {
                animatingFrom = effectiveStartSq;
                animatingTo = endSq;

                moveHistory = [...moveHistory, [effectiveStartSq, endSq]];

                // Send move to backend
                await handleMove(effectiveStartSq, endSq);

                // Clear animation after it completes
                setTimeout(() => {
                    animatingFrom = "";
                    animatingTo = "";
                }, PIECE_MOVE_DURATION);
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

    const resetSelection = () => {
        fromSquare = "";
        toSquare = "";
        selectedButton = null;
        possibleMoves = [];
    };

    const handleDragStart = (sq: Square, event: DragEvent) => {
        const file = sq.file.toLowerCase();
        const rank = sq.rank + 1;
        const squareId = file + rank;

        console.log("Drag start from square:", squareId);

        if (sq.piece === "None" || !isPossibleToMovePiece(sq, whiteTurn)) {
            event.preventDefault();
            showInvalidMoveAnimation(squareId);
            return;
        }

        if (event.dataTransfer) {
            event.dataTransfer.setData("text/plain", squareId);
            event.dataTransfer.effectAllowed = "move";

            const pieceImg = event.target as HTMLElement;
            if (pieceImg) {
                const dragImage = pieceImg.cloneNode(true) as HTMLElement;
                dragImage.classList.add("drag-image");
                document.body.appendChild(dragImage);

                const rect = pieceImg.getBoundingClientRect();
                event.dataTransfer.setDragImage(
                    dragImage,
                    rect.width / 2,
                    rect.height / 2,
                );

                setTimeout(() => {
                    if (dragImage.parentNode) {
                        dragImage.parentNode.removeChild(dragImage);
                    }
                }, 0);
            }
        }

        fromSquare = squareId;
        selectedButton = squareId;
        possibleMoves = sq.possible_moves;
        startSq = squareId;
        currentDragStartSquare = squareId;
        movingPieceElement = event.target as HTMLElement;
    };
</script>

<div class="flex justify-center items-center py-4 sm:py-6 md:py-8 touch-none">
    <div class="flex flex-col justify-center items-center">
        <div
            class="grid grid-cols-8 gap-0 w-[80vw] max-w-[90vw] sm:w-[56vw] sm:max-w-[80vw] md:w-[36vw] md:max-w-[65vw] lg:w-[32vw] lg:max-w-[50vw]"
        >
            {#each boardToFront as row, i}
                {#each row as sq, j}
                    <ChessSquare
                        {chess}
                        {sq}
                        {selectedButton}
                        {possibleMoves}
                        {animatingFrom}
                        {animatingTo}
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
    </div>
</div>
