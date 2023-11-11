
export enum MessageType {
    Input = "input",
}

export interface ClientMessage {
    type: MessageType;
}

export interface InputMessage extends ClientMessage {
    type: MessageType.Input;
    input_state: bigint;
    timestamp?: bigint; // Millis since Epoch.
}