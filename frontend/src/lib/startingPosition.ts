import type { Chess } from './types';

export const startingPosition: Chess = {
	board: [
		[
			{ file: 'A', rank: 0, color: 'Black', piece: { Rook: 'White' } },
			{ file: 'A', rank: 1, color: 'White', piece: { Pawn: 'White' } },
			{ file: 'A', rank: 2, color: 'Black', piece: 'None' },
			{ file: 'A', rank: 3, color: 'White', piece: 'None' },
			{ file: 'A', rank: 4, color: 'Black', piece: 'None' },
			{ file: 'A', rank: 5, color: 'White', piece: 'None' },
			{ file: 'A', rank: 6, color: 'Black', piece: { Pawn: 'Black' } },
			{ file: 'A', rank: 7, color: 'White', piece: { Rook: 'Black' } }
		],
		[
			{ file: 'B', rank: 0, color: 'White', piece: { Knight: 'White' } },
			{ file: 'B', rank: 1, color: 'Black', piece: { Pawn: 'White' } },
			{ file: 'B', rank: 2, color: 'White', piece: 'None' },
			{ file: 'B', rank: 3, color: 'Black', piece: 'None' },
			{ file: 'B', rank: 4, color: 'White', piece: 'None' },
			{ file: 'B', rank: 5, color: 'Black', piece: 'None' },
			{ file: 'B', rank: 6, color: 'White', piece: { Pawn: 'Black' } },
			{ file: 'B', rank: 7, color: 'Black', piece: { Knight: 'Black' } }
		],
		[
			{ file: 'C', rank: 0, color: 'Black', piece: { Bishop: 'White' } },
			{ file: 'C', rank: 1, color: 'White', piece: { Pawn: 'White' } },
			{ file: 'C', rank: 2, color: 'Black', piece: 'None' },
			{ file: 'C', rank: 3, color: 'White', piece: 'None' },
			{ file: 'C', rank: 4, color: 'Black', piece: 'None' },
			{ file: 'C', rank: 5, color: 'White', piece: 'None' },
			{ file: 'C', rank: 6, color: 'Black', piece: { Pawn: 'Black' } },
			{ file: 'C', rank: 7, color: 'White', piece: { Bishop: 'Black' } }
		],
		[
			{ file: 'D', rank: 0, color: 'White', piece: { Queen: 'White' } },
			{ file: 'D', rank: 1, color: 'Black', piece: { Pawn: 'White' } },
			{ file: 'D', rank: 2, color: 'White', piece: 'None' },
			{ file: 'D', rank: 3, color: 'Black', piece: 'None' },
			{ file: 'D', rank: 4, color: 'White', piece: 'None' },
			{ file: 'D', rank: 5, color: 'Black', piece: 'None' },
			{ file: 'D', rank: 6, color: 'White', piece: { Pawn: 'Black' } },
			{ file: 'D', rank: 7, color: 'Black', piece: { Queen: 'Black' } }
		],
		[
			{ file: 'E', rank: 0, color: 'Black', piece: { King: 'White' } },
			{ file: 'E', rank: 1, color: 'White', piece: { Pawn: 'White' } },
			{ file: 'E', rank: 2, color: 'Black', piece: 'None' },
			{ file: 'E', rank: 3, color: 'White', piece: 'None' },
			{ file: 'E', rank: 4, color: 'Black', piece: 'None' },
			{ file: 'E', rank: 5, color: 'White', piece: 'None' },
			{ file: 'E', rank: 6, color: 'Black', piece: { Pawn: 'Black' } },
			{ file: 'E', rank: 7, color: 'White', piece: { King: 'Black' } }
		],
		[
			{ file: 'F', rank: 0, color: 'White', piece: { Bishop: 'White' } },
			{ file: 'F', rank: 1, color: 'Black', piece: { Pawn: 'White' } },
			{ file: 'F', rank: 2, color: 'White', piece: 'None' },
			{ file: 'F', rank: 3, color: 'Black', piece: 'None' },
			{ file: 'F', rank: 4, color: 'White', piece: 'None' },
			{ file: 'F', rank: 5, color: 'Black', piece: 'None' },
			{ file: 'F', rank: 6, color: 'White', piece: { Pawn: 'Black' } },
			{ file: 'F', rank: 7, color: 'Black', piece: { Bishop: 'Black' } }
		],
		[
			{ file: 'G', rank: 0, color: 'Black', piece: { Knight: 'White' } },
			{ file: 'G', rank: 1, color: 'White', piece: { Pawn: 'White' } },
			{ file: 'G', rank: 2, color: 'Black', piece: 'None' },
			{ file: 'G', rank: 3, color: 'White', piece: 'None' },
			{ file: 'G', rank: 4, color: 'Black', piece: 'None' },
			{ file: 'G', rank: 5, color: 'White', piece: 'None' },
			{ file: 'G', rank: 6, color: 'Black', piece: { Pawn: 'Black' } },
			{ file: 'G', rank: 7, color: 'White', piece: { Knight: 'Black' } }
		],
		[
			{ file: 'H', rank: 0, color: 'White', piece: { Rook: 'White' } },
			{ file: 'H', rank: 1, color: 'Black', piece: { Pawn: 'White' } },
			{ file: 'H', rank: 2, color: 'White', piece: 'None' },
			{ file: 'H', rank: 3, color: 'Black', piece: 'None' },
			{ file: 'H', rank: 4, color: 'White', piece: 'None' },
			{ file: 'H', rank: 5, color: 'Black', piece: 'None' },
			{ file: 'H', rank: 6, color: 'White', piece: { Pawn: 'Black' } },
			{ file: 'H', rank: 7, color: 'Black', piece: { Rook: 'Black' } }
		]
	],
	turn_number: 0,
	latest_move: null,
	castling: {
		white: {
			king: true,
			queen: true
		},
		black: {
			king: true,
			queen: true
		}
	},
	white_player: { color: 'White', in_check: false, victory: false },
	black_player: { color: 'Black', in_check: false, victory: false },
	gamestate: 'InProgress',
	fifty_move_rule: 0,
	list_of_moves: []
};
