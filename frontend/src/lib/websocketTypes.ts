import type { Chess } from "./types";

export interface InitialStateMessage {
    type: "initial_state";
    chess: Chess;
}

export interface UpdateMessage {
    type: "update";
    chess: Chess;
}

export interface ResetMessage {
    type: "reset";
    chess: Chess;
}

export interface OtherMessage {
    type: string;
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    [key: string]: any;
}

export type WebSocketMessage =
    | InitialStateMessage
    | UpdateMessage
    | ResetMessage
    | OtherMessage;
