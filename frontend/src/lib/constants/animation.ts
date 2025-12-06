/**
 * Animation constants for chess piece movements and transitions
 * All durations are in milliseconds
 */

/**
 * Duration for piece move animations (fade out/in, scale)
 * Used when a piece moves from one square to another
 */
export const PIECE_MOVE_DURATION = 100;

/**
 * Duration for invalid move shake animation
 */
export const INVALID_MOVE_DURATION = 300;

/**
 * Duration for square selection transitions
 */
export const SQUARE_TRANSITION_DURATION = 100;

/**
 * CSS easing function for piece animations
 */
export const PIECE_ANIMATION_EASING = "ease-out";

/**
 * CSS easing function for invalid move shake
 */
export const INVALID_MOVE_EASING = "cubic-bezier(0.36, 0.07, 0.19, 0.97)";
