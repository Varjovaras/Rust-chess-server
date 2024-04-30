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
	rank: z.number(),
	color: z.string(),
	piece: z.union([pieceSchema, z.literal('None')])
});

export const boardSchema = z.array(z.array(squareSchema));

export const castlingSchema = z.object({
	white: z.object({
		king: z.boolean(),
		queen: z.boolean()
	}),
	black: z.object({
		king: z.boolean(),
		queen: z.boolean()
	})
});

export const playerSchema = z.object({
	color: z.string(),
	in_check: z.boolean(),
	victory: z.boolean()
});

const latestMoveSchema = z.array(z.union([squareSchema, z.string()])).nullable();

const fileSchema = z.string().length(1);
const rankSchema = z.number();
const moveSchema = z.tuple([z.tuple([fileSchema, rankSchema]), z.tuple([fileSchema, rankSchema])]);
export const listOfMovesSchema = z.array(moveSchema);

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
export type ListOfMoves = z.TypeOf<typeof listOfMovesSchema>;
export type GameState = z.TypeOf<typeof gameStateSchema>;
// type Castling = z.TypeOf<typeof castlingSchema>;
