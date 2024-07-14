/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { ClientID } from "./common";
import { Any } from "./google/protobuf/any";

export const protobufPackage = "client_output";

export interface ClientOutput {
  gameOutputMessage: Any | undefined;
}

export interface DirectMessage {
  clientId: ClientID | undefined;
  clientOutput: ClientOutput | undefined;
}

export interface RoomBroadcast {
  clientOutput: ClientOutput | undefined;
}

export interface ClientOutputBatch {
  directMessages: DirectMessage[];
  broadcast?: RoomBroadcast | undefined;
}

function createBaseClientOutput(): ClientOutput {
  return { gameOutputMessage: undefined };
}

export const ClientOutput = {
  encode(message: ClientOutput, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameOutputMessage !== undefined) {
      Any.encode(message.gameOutputMessage, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ClientOutput {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseClientOutput();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.gameOutputMessage = Any.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ClientOutput {
    return { gameOutputMessage: isSet(object.gameOutputMessage) ? Any.fromJSON(object.gameOutputMessage) : undefined };
  },

  toJSON(message: ClientOutput): unknown {
    const obj: any = {};
    if (message.gameOutputMessage !== undefined) {
      obj.gameOutputMessage = Any.toJSON(message.gameOutputMessage);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ClientOutput>, I>>(base?: I): ClientOutput {
    return ClientOutput.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ClientOutput>, I>>(object: I): ClientOutput {
    const message = createBaseClientOutput();
    message.gameOutputMessage = (object.gameOutputMessage !== undefined && object.gameOutputMessage !== null)
      ? Any.fromPartial(object.gameOutputMessage)
      : undefined;
    return message;
  },
};

function createBaseDirectMessage(): DirectMessage {
  return { clientId: undefined, clientOutput: undefined };
}

export const DirectMessage = {
  encode(message: DirectMessage, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.clientId !== undefined) {
      ClientID.encode(message.clientId, writer.uint32(10).fork()).ldelim();
    }
    if (message.clientOutput !== undefined) {
      ClientOutput.encode(message.clientOutput, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DirectMessage {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDirectMessage();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.clientId = ClientID.decode(reader, reader.uint32());
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.clientOutput = ClientOutput.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): DirectMessage {
    return {
      clientId: isSet(object.clientId) ? ClientID.fromJSON(object.clientId) : undefined,
      clientOutput: isSet(object.clientOutput) ? ClientOutput.fromJSON(object.clientOutput) : undefined,
    };
  },

  toJSON(message: DirectMessage): unknown {
    const obj: any = {};
    if (message.clientId !== undefined) {
      obj.clientId = ClientID.toJSON(message.clientId);
    }
    if (message.clientOutput !== undefined) {
      obj.clientOutput = ClientOutput.toJSON(message.clientOutput);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<DirectMessage>, I>>(base?: I): DirectMessage {
    return DirectMessage.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<DirectMessage>, I>>(object: I): DirectMessage {
    const message = createBaseDirectMessage();
    message.clientId = (object.clientId !== undefined && object.clientId !== null)
      ? ClientID.fromPartial(object.clientId)
      : undefined;
    message.clientOutput = (object.clientOutput !== undefined && object.clientOutput !== null)
      ? ClientOutput.fromPartial(object.clientOutput)
      : undefined;
    return message;
  },
};

function createBaseRoomBroadcast(): RoomBroadcast {
  return { clientOutput: undefined };
}

export const RoomBroadcast = {
  encode(message: RoomBroadcast, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.clientOutput !== undefined) {
      ClientOutput.encode(message.clientOutput, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RoomBroadcast {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRoomBroadcast();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.clientOutput = ClientOutput.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): RoomBroadcast {
    return { clientOutput: isSet(object.clientOutput) ? ClientOutput.fromJSON(object.clientOutput) : undefined };
  },

  toJSON(message: RoomBroadcast): unknown {
    const obj: any = {};
    if (message.clientOutput !== undefined) {
      obj.clientOutput = ClientOutput.toJSON(message.clientOutput);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<RoomBroadcast>, I>>(base?: I): RoomBroadcast {
    return RoomBroadcast.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<RoomBroadcast>, I>>(object: I): RoomBroadcast {
    const message = createBaseRoomBroadcast();
    message.clientOutput = (object.clientOutput !== undefined && object.clientOutput !== null)
      ? ClientOutput.fromPartial(object.clientOutput)
      : undefined;
    return message;
  },
};

function createBaseClientOutputBatch(): ClientOutputBatch {
  return { directMessages: [], broadcast: undefined };
}

export const ClientOutputBatch = {
  encode(message: ClientOutputBatch, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.directMessages) {
      DirectMessage.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    if (message.broadcast !== undefined) {
      RoomBroadcast.encode(message.broadcast, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ClientOutputBatch {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseClientOutputBatch();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.directMessages.push(DirectMessage.decode(reader, reader.uint32()));
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.broadcast = RoomBroadcast.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ClientOutputBatch {
    return {
      directMessages: globalThis.Array.isArray(object?.directMessages)
        ? object.directMessages.map((e: any) => DirectMessage.fromJSON(e))
        : [],
      broadcast: isSet(object.broadcast) ? RoomBroadcast.fromJSON(object.broadcast) : undefined,
    };
  },

  toJSON(message: ClientOutputBatch): unknown {
    const obj: any = {};
    if (message.directMessages?.length) {
      obj.directMessages = message.directMessages.map((e) => DirectMessage.toJSON(e));
    }
    if (message.broadcast !== undefined) {
      obj.broadcast = RoomBroadcast.toJSON(message.broadcast);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ClientOutputBatch>, I>>(base?: I): ClientOutputBatch {
    return ClientOutputBatch.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ClientOutputBatch>, I>>(object: I): ClientOutputBatch {
    const message = createBaseClientOutputBatch();
    message.directMessages = object.directMessages?.map((e) => DirectMessage.fromPartial(e)) || [];
    message.broadcast = (object.broadcast !== undefined && object.broadcast !== null)
      ? RoomBroadcast.fromPartial(object.broadcast)
      : undefined;
    return message;
  },
};

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
  : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
