import { z } from 'zod';

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
export type ChessBoard = z.TypeOf<typeof boardSchema>;
export type Piece = z.TypeOf<typeof pieceSchema>;
