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

// type OtherMessageType = Exclude<string, "initial_state" | "update" | "reset">;

// export interface OtherMessage {
//     type: OtherMessageType;
//     [key: string]: unknown;
// }
export type WebSocketMessage =
    | InitialStateMessage
    | UpdateMessage
    | ResetMessage;
// | OtherMessage;
