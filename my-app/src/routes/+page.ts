import { z } from 'zod';

// type Board = {
// 	board: Square[][];
// };

// type Player = {
// 	color: string;
// 	in_check: boolean;
// 	victory: boolean;
// };

// type Chess = {
// 	board: Board;
// 	turn_number: number;
// 	// eslint-disable-next-line @typescript-eslint/no-explicit-any
// 	latest_move: any;
// 	castling: {
// 		white_king_side_castling: boolean;
// 		white_queen_side_Castling: boolean;
// 		black_king_side_castling: boolean;
// 		black_queen_side_castling: boolean;
// 	};
// 	white_player: Player;
// 	black_player: Player;
// 	gamestate: string;
// 	fifty_move_rule: number;
// 	list_of_moves: [];
// 	new_move: [][];
// };

// type Square = {
// 	file: string;
// 	rank: string;
// 	color: string;
// 	// eslint-disable-next-line @typescript-eslint/no-explicit-any
// 	piece: any;
// };

export async function load() {
	const response = await fetch(`http://127.0.0.1:8000/chess`);
	// console.log(response);
	const chess = await response.json();
	console.log(chess);
	const ChessSchema = z.object({
		board: z.array(z.array(z.any())),
		turn_number: z.number(),
		latest_move: z.any(),
		castling: z.object({
			white_king_side_castling: z.boolean(),
			white_queen_side_Castling: z.boolean(),
			black_king_side_castling: z.boolean(),
			black_queen_side_castling: z.boolean()
		}),
		white_player: z.object({
			color: z.string(),
			in_check: z.boolean(),
			victory: z.boolean()
		}),
		black_player: z.object({
			color: z.string(),
			in_check: z.boolean(),
			victory: z.boolean()
		}),
		gamestate: z.string(),
		fifty_move_rule: z.number(),
		list_of_moves: z.array(z.any()),
		new_move: z.any()
	});

	console.log(ChessSchema);
	return { post: chess };
}
