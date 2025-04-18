import { z } from "zod";

export const pieceTypeSchema = z.object({
	Pawn: z.string().optional(),
	Knight: z.string().optional(),
	Bishop: z.string().optional(),
	Rook: z.string().optional(),
	Queen: z.string().optional(),
	King: z.string().optional(),
	None: z.literal("None").optional(),
});

// const pieceTypeSchema = z.enum([
// 	"Pawn",
// 	"Knight",
// 	"Bishop",
// 	"Rook",
// 	"Queen",
// 	"King",
// 	"None",
// ]);

export const squareSchema = z.object({
	file: z.string(),
	rank: z.number(),
	color: z.string(),
	piece: z.union([pieceTypeSchema, z.literal("None")]),
	possible_moves: z.array(
		z.tuple([
			z.tuple([z.number(), z.number()]),
			z.tuple([z.number(), z.number()]),
			z.tuple([z.number(), z.number()]),
		]),
	),
});

export const possibleMoveSchema = z.array(
	z.tuple([
		z.tuple([z.number(), z.number()]),
		z.tuple([z.number(), z.number()]),
		z.tuple([z.number(), z.number()]),
	]),
);

export const boardSchema = z.array(z.array(squareSchema));

export const castlingSchema = z.object({
	kingside: z.boolean(),
	queenside: z.boolean(),
});

export const playerSchema = z.object({
	color: z.string(),
	in_check: z.boolean(),
	victory: z.boolean(),
	castling: castlingSchema,
});

const latestMoveSchema = z
	.array(z.union([squareSchema, z.string()]))
	.nullable();

const fileSchema = z.string().length(1);
const rankSchema = z.number();
const moveSchema = z.tuple([
	z.tuple([fileSchema, rankSchema]),
	z.tuple([fileSchema, rankSchema]),
	z.tuple([z.number(), z.number()]),
]);
export const listOfMovesSchema = z.array(moveSchema);

export const gameStateSchema = z.enum([
	"InProgress",
	"WhiteVictory",
	"BlackVictory",
	"Draw",
]);

export const piecesEatenSchema = z.object({
	white: z.array(pieceTypeSchema),
	black: z.array(pieceTypeSchema),
});

export const chessSchema = z.object({
	board: boardSchema,
	turn_number: z.number(),
	latest_move: latestMoveSchema,
	// castling: castlingSchema,
	players: z.tuple([playerSchema, playerSchema]),
	gamestate: gameStateSchema,
	fifty_move_rule: z.number(),
	list_of_moves: listOfMovesSchema,
	pieces_eaten: piecesEatenSchema,
});

export type Chess = z.TypeOf<typeof chessSchema>;
export type ChessBoard = z.TypeOf<typeof boardSchema>;
export type Piece = z.TypeOf<typeof pieceTypeSchema>;
export type Square = z.TypeOf<typeof squareSchema>;
export type ListOfMoves = z.TypeOf<typeof listOfMovesSchema>;
export type GameState = z.TypeOf<typeof gameStateSchema>;
export type PossibleMoves = z.TypeOf<typeof possibleMoveSchema>;
export type Castling = z.TypeOf<typeof castlingSchema>;
export type PiecesEaten = z.TypeOf<typeof piecesEatenSchema>;
