import { z } from 'zod';
export const pieceSchema = z.object({
	Pawn: z.string().optional(),
	Knight: z.string().optional(),
	Bishop: z.string().optional(),
	Rook: z.string().optional(),
	Queen: z.string().optional(),
	King: z.string().optional(),
	None: z.literal('None').optional()
});

export const squareSchema = z.object({
	file: z.string(),
	rank: z.string(),
	color: z.string(),
	piece: z.union([pieceSchema, z.literal('None')])
});

export const boardSchema = z.array(z.array(squareSchema));

export const castlingSchema = z.object({
	white_king_side_castling: z.boolean(),
	white_queen_side_castling: z.boolean(),
	black_king_side_castling: z.boolean(),
	black_queen_side_castling: z.boolean()
});

export const playerSchema = z.object({
	color: z.string(),
	in_check: z.boolean(),
	victory: z.boolean()
});

const latestMoveSchema = z.array(z.union([squareSchema, z.string()])).nullable();

const moveSchema = z.array(z.union([squareSchema, z.string()]));

const listOfMovesSchema = z.array(moveSchema);

export const gameStateSchema = z.enum(['InProgress', 'WhiteVictory', 'BlackVictory', 'Draw']);

export const schema = z.object({
	board: boardSchema,
	turn_number: z.number(),
	latest_move: latestMoveSchema,
	castling: castlingSchema,
	white_player: playerSchema,
	black_player: playerSchema,
	gamestate: gameStateSchema,
	fifty_move_rule: z.number(),
	list_of_moves: listOfMovesSchema
});

export type Chess = z.TypeOf<typeof schema>;
export type ChessBoard = z.TypeOf<typeof boardSchema>;
export type Piece = z.TypeOf<typeof pieceSchema>;
export type Square = z.TypeOf<typeof squareSchema>;
