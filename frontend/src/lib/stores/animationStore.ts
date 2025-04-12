import { writable } from "svelte/store";

interface AnimationState {
    fromSquare: string;
    toSquare: string;
    piece: string;
    isAnimating: boolean;
    animationId: number;
    // Add these new properties
    movingPieceElement: HTMLElement | null;
    startPosition: { x: number; y: number } | null;
    endPosition: { x: number; y: number } | null;
}

export const createAnimationStore = () => {
    const { subscribe, set, update } = writable<AnimationState>({
        fromSquare: "",
        toSquare: "",
        piece: "",
        isAnimating: false,
        animationId: 0,
        movingPieceElement: null,
        startPosition: null,
        endPosition: null,
    });

    return {
        subscribe,
        startAnimation: (
            fromSquare: string,
            toSquare: string,
            piece: string,
            movingElement?: HTMLElement,
        ) => {
            // Calculate positions if element is provided
            let startPos = null;
            let endPos = null;

            if (movingElement) {
                const fromElement = document.getElementById(fromSquare);
                const toElement = document.getElementById(toSquare);

                if (fromElement && toElement) {
                    const fromRect = fromElement.getBoundingClientRect();
                    const toRect = toElement.getBoundingClientRect();

                    startPos = { x: fromRect.left, y: fromRect.top };
                    endPos = { x: toRect.left, y: toRect.top };
                }
            }

            update((state) => ({
                ...state,
                fromSquare,
                toSquare,
                piece,
                isAnimating: true,
                animationId: state.animationId + 1,
                movingPieceElement: movingElement || null,
                startPosition: startPos,
                endPosition: endPos,
            }));

            // Automatically end animation after duration
            setTimeout(() => {
                update((state) => {
                    if (
                        state.fromSquare === fromSquare &&
                        state.toSquare === toSquare
                    ) {
                        return {
                            ...state,
                            isAnimating: false,
                            movingPieceElement: null,
                            startPosition: null,
                            endPosition: null,
                        };
                    }
                    return state;
                });
            }, 300); // Match your CSS transition duration
        },
        endAnimation: () => {
            set({
                fromSquare: "",
                toSquare: "",
                piece: "",
                isAnimating: false,
                animationId: 0,
                movingPieceElement: null,
                startPosition: null,
                endPosition: null,
            });
        },
    };
};

export const animationStore = createAnimationStore();
