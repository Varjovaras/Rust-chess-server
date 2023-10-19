import * as z from 'zod';

const pieceSchema = z.object({
	Pawn: z.string().optional(),
	Knight: z.string().optional(),
	Bishop: z.string().optional(),
	Rook: z.string().optional(),
	Queen: z.string().optional(),
	King: z.string().optional(),
	None: z.literal('None').optional()
});

const squareSchema = z.object({
	file: z.string(),
	rank: z.string(),
	color: z.string(),
	piece: z.union([pieceSchema, z.literal('None')])
});

const boardSchema = z.array(z.array(squareSchema));

const castlingSchema = z.object({
	white_king_side_castling: z.boolean(),
	white_queen_side_castling: z.boolean(),
	black_king_side_castling: z.boolean(),
	black_queen_side_castling: z.boolean()
});

const playerSchema = z.object({
	color: z.string(),
	in_check: z.boolean(),
	victory: z.boolean()
});

const gameStateSchema = z.enum(['InProgress', 'WhiteVictory', 'BlackVictory', 'Draw']);

const schema = z.object({
	board: boardSchema,
	turn_number: z.number(),
	latest_move: z.nullable(z.array(z.array(z.string()))),
	castling: castlingSchema,
	white_player: playerSchema,
	black_player: playerSchema,
	gamestate: gameStateSchema,
	fifty_move_rule: z.number(),
	list_of_moves: z.array(z.array(z.string())),
	new_move: z.array(z.array(z.string()))
});

export type Chess = z.TypeOf<typeof schema>;

export async function load() {
	const response = await fetch(`http://127.0.0.1:8000/chess`);
	const chess = await response.json();

	const validatedChess = schema.parse(chess);
	return { post: validatedChess };
}
