import type { Chess } from './types';

export const startingPosition: Chess = {
	board: [
		[
			{
				file: 'A',
				rank: 0,
				color: 'Black',
				piece: { Rook: 'White' },
				possible_moves: [],
			},
			{
				file: 'A',
				rank: 1,
				color: 'White',
				piece: { Pawn: 'White' },
				possible_moves: [
					[
						[0, 1],
						[0, 2],
					],
				],
			},
			{ file: 'A', rank: 2, color: 'Black', piece: 'None', possible_moves: [] },
			{ file: 'A', rank: 3, color: 'White', piece: 'None', possible_moves: [] },
			{ file: 'A', rank: 4, color: 'Black', piece: 'None', possible_moves: [] },
			{ file: 'A', rank: 5, color: 'White', piece: 'None', possible_moves: [] },
			{
				file: 'A',
				rank: 6,
				color: 'Black',
				piece: { Pawn: 'Black' },
				possible_moves: [
					[
						[0, 6],
						[0, 5],
					],
				],
			},
			{
				file: 'A',
				rank: 7,
				color: 'White',
				piece: { Rook: 'Black' },
				possible_moves: [],
			},
		],
		[
			{
				file: 'B',
				rank: 0,
				color: 'White',
				piece: { Knight: 'White' },
				possible_moves: [
					[
						[1, 0],
						[0, 2],
					],
					[
						[1, 0],
						[2, 2],
					],
				],
			},
			{
				file: 'B',
				rank: 1,
				color: 'Black',
				piece: { Pawn: 'White' },
				possible_moves: [
					[
						[1, 1],
						[1, 2],
					],
				],
			},
			{ file: 'B', rank: 2, color: 'White', piece: 'None', possible_moves: [] },
			{ file: 'B', rank: 3, color: 'Black', piece: 'None', possible_moves: [] },
			{ file: 'B', rank: 4, color: 'White', piece: 'None', possible_moves: [] },
			{ file: 'B', rank: 5, color: 'Black', piece: 'None', possible_moves: [] },
			{
				file: 'B',
				rank: 6,
				color: 'White',
				piece: { Pawn: 'Black' },
				possible_moves: [
					[
						[1, 6],
						[1, 5],
					],
				],
			},
			{
				file: 'B',
				rank: 7,
				color: 'Black',
				piece: { Knight: 'Black' },
				possible_moves: [
					[
						[1, 7],
						[0, 5],
					],
					[
						[1, 7],
						[2, 5],
					],
				],
			},
		],
		[
			{
				file: 'C',
				rank: 0,
				color: 'Black',
				piece: { Bishop: 'White' },
				possible_moves: [],
			},
			{
				file: 'C',
				rank: 1,
				color: 'White',
				piece: { Pawn: 'White' },
				possible_moves: [
					[
						[2, 1],
						[2, 2],
					],
				],
			},
			{ file: 'C', rank: 2, color: 'Black', piece: 'None', possible_moves: [] },
			{ file: 'C', rank: 3, color: 'White', piece: 'None', possible_moves: [] },
			{ file: 'C', rank: 4, color: 'Black', piece: 'None', possible_moves: [] },
			{ file: 'C', rank: 5, color: 'White', piece: 'None', possible_moves: [] },
			{
				file: 'C',
				rank: 6,
				color: 'Black',
				piece: { Pawn: 'Black' },
				possible_moves: [
					[
						[2, 6],
						[2, 5],
					],
				],
			},
			{
				file: 'C',
				rank: 7,
				color: 'White',
				piece: { Bishop: 'Black' },
				possible_moves: [],
			},
		],
		[
			{
				file: 'D',
				rank: 0,
				color: 'White',
				piece: { Queen: 'White' },
				possible_moves: [],
			},
			{
				file: 'D',
				rank: 1,
				color: 'Black',
				piece: { Pawn: 'White' },
				possible_moves: [
					[
						[3, 1],
						[3, 2],
					],
				],
			},
			{ file: 'D', rank: 2, color: 'White', piece: 'None', possible_moves: [] },
			{ file: 'D', rank: 3, color: 'Black', piece: 'None', possible_moves: [] },
			{ file: 'D', rank: 4, color: 'White', piece: 'None', possible_moves: [] },
			{ file: 'D', rank: 5, color: 'Black', piece: 'None', possible_moves: [] },
			{
				file: 'D',
				rank: 6,
				color: 'White',
				piece: { Pawn: 'Black' },
				possible_moves: [
					[
						[3, 6],
						[3, 5],
					],
				],
			},
			{
				file: 'D',
				rank: 7,
				color: 'Black',
				piece: { Queen: 'Black' },
				possible_moves: [],
			},
		],
		[
			{
				file: 'E',
				rank: 0,
				color: 'Black',
				piece: { King: 'White' },
				possible_moves: [],
			},
			{
				file: 'E',
				rank: 1,
				color: 'White',
				piece: { Pawn: 'White' },
				possible_moves: [
					[
						[4, 1],
						[4, 2],
					],
				],
			},
			{ file: 'E', rank: 2, color: 'Black', piece: 'None', possible_moves: [] },
			{ file: 'E', rank: 3, color: 'White', piece: 'None', possible_moves: [] },
			{ file: 'E', rank: 4, color: 'Black', piece: 'None', possible_moves: [] },
			{ file: 'E', rank: 5, color: 'White', piece: 'None', possible_moves: [] },
			{
				file: 'E',
				rank: 6,
				color: 'Black',
				piece: { Pawn: 'Black' },
				possible_moves: [
					[
						[4, 6],
						[4, 5],
					],
				],
			},
			{
				file: 'E',
				rank: 7,
				color: 'White',
				piece: { King: 'Black' },
				possible_moves: [],
			},
		],
		[
			{
				file: 'F',
				rank: 0,
				color: 'White',
				piece: { Bishop: 'White' },
				possible_moves: [],
			},
			{
				file: 'F',
				rank: 1,
				color: 'Black',
				piece: { Pawn: 'White' },
				possible_moves: [
					[
						[5, 1],
						[5, 2],
					],
				],
			},
			{ file: 'F', rank: 2, color: 'White', piece: 'None', possible_moves: [] },
			{ file: 'F', rank: 3, color: 'Black', piece: 'None', possible_moves: [] },
			{ file: 'F', rank: 4, color: 'White', piece: 'None', possible_moves: [] },
			{ file: 'F', rank: 5, color: 'Black', piece: 'None', possible_moves: [] },
			{
				file: 'F',
				rank: 6,
				color: 'White',
				piece: { Pawn: 'Black' },
				possible_moves: [
					[
						[5, 6],
						[5, 5],
					],
				],
			},
			{
				file: 'F',
				rank: 7,
				color: 'Black',
				piece: { Bishop: 'Black' },
				possible_moves: [],
			},
		],
		[
			{
				file: 'G',
				rank: 0,
				color: 'Black',
				piece: { Knight: 'White' },
				possible_moves: [
					[
						[6, 0],
						[5, 2],
					],
					[
						[6, 0],
						[7, 2],
					],
				],
			},
			{
				file: 'G',
				rank: 1,
				color: 'White',
				piece: { Pawn: 'White' },
				possible_moves: [
					[
						[6, 1],
						[6, 2],
					],
				],
			},
			{ file: 'G', rank: 2, color: 'Black', piece: 'None', possible_moves: [] },
			{ file: 'G', rank: 3, color: 'White', piece: 'None', possible_moves: [] },
			{ file: 'G', rank: 4, color: 'Black', piece: 'None', possible_moves: [] },
			{ file: 'G', rank: 5, color: 'White', piece: 'None', possible_moves: [] },
			{
				file: 'G',
				rank: 6,
				color: 'Black',
				piece: { Pawn: 'Black' },
				possible_moves: [
					[
						[6, 6],
						[6, 5],
					],
				],
			},
			{
				file: 'G',
				rank: 7,
				color: 'White',
				piece: { Knight: 'Black' },
				possible_moves: [
					[
						[6, 7],
						[5, 5],
					],
					[
						[6, 7],
						[7, 5],
					],
				],
			},
		],
		[
			{
				file: 'H',
				rank: 0,
				color: 'White',
				piece: { Rook: 'White' },
				possible_moves: [],
			},
			{
				file: 'H',
				rank: 1,
				color: 'Black',
				piece: { Pawn: 'White' },
				possible_moves: [
					[
						[7, 1],
						[7, 2],
					],
				],
			},
			{ file: 'H', rank: 2, color: 'White', piece: 'None', possible_moves: [] },
			{ file: 'H', rank: 3, color: 'Black', piece: 'None', possible_moves: [] },
			{ file: 'H', rank: 4, color: 'White', piece: 'None', possible_moves: [] },
			{ file: 'H', rank: 5, color: 'Black', piece: 'None', possible_moves: [] },
			{
				file: 'H',
				rank: 6,
				color: 'White',
				piece: { Pawn: 'Black' },
				possible_moves: [
					[
						[7, 6],
						[7, 5],
					],
				],
			},
			{
				file: 'H',
				rank: 7,
				color: 'Black',
				piece: { Rook: 'Black' },
				possible_moves: [],
			},
		],
	],
	turn_number: 0,
	latest_move: null,
	castling: {
		white: { king: true, queen: true },
		black: { king: true, queen: true },
	},
	white_player: { color: 'White', in_check: false, victory: false },
	black_player: { color: 'Black', in_check: false, victory: false },
	gamestate: 'InProgress',
	fifty_move_rule: 0,
	list_of_moves: [],
};
