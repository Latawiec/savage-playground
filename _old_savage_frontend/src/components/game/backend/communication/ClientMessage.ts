
export enum MessageType {
    Input = 'input',
}

export interface ClientMessage {
    type: MessageType;
}

export interface InputMessage extends ClientMessage {
    type: MessageType.Input;
    inputState: bigint;
    timestamp?: bigint; // Millis since Epoch.
}
