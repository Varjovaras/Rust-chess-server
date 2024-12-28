import { writable } from "svelte/store";

interface AnimationState {
	fromSquare: string;
	toSquare: string;
	piece: string;
	isAnimating: boolean;
}

export const createAnimationStore = () => {
	const { subscribe, set, update } = writable<AnimationState>({
		fromSquare: "",
		toSquare: "",
		piece: "",
		isAnimating: false,
	});

	return {
		subscribe,
		startAnimation: (fromSquare: string, toSquare: string, piece: string) => {
			update((state) => ({
				...state,
				fromSquare,
				toSquare,
				piece,
				isAnimating: true,
			}));
		},
		endAnimation: () => {
			set({ fromSquare: "", toSquare: "", piece: "", isAnimating: false });
		},
	};
};

export const animationStore = createAnimationStore();
