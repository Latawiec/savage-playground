/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { Any } from "./google/protobuf/any";

export const protobufPackage = "host_instance";

export interface ClientID {
  value: string;
}

/**
 * Origin: Client
 * Target: Instance
 */
export interface ClientMessage {
  clientId: ClientID | undefined;
  gameInputMessage: Any | undefined;
}

/**
 * Origin: Instance
 * Target: Client
 */
export interface InstanceDirectMessage {
  clientId: ClientID | undefined;
  gameOutputMessage: Any | undefined;
}

/**
 * Origin: Instance
 * Target: All Clients
 */
export interface InstanceBroadcast {
  gameOutputMessage: Any | undefined;
}

export interface InstanceMessage {
  directMessages: InstanceDirectMessage[];
  broadcast?: InstanceBroadcast | undefined;
}

function createBaseClientID(): ClientID {
  return { value: "" };
}

export const ClientID = {
  encode(message: ClientID, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.value !== "") {
      writer.uint32(10).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ClientID {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseClientID();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.value = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ClientID {
    return { value: isSet(object.value) ? globalThis.String(object.value) : "" };
  },

  toJSON(message: ClientID): unknown {
    const obj: any = {};
    if (message.value !== "") {
      obj.value = message.value;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ClientID>, I>>(base?: I): ClientID {
    return ClientID.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ClientID>, I>>(object: I): ClientID {
    const message = createBaseClientID();
    message.value = object.value ?? "";
    return message;
  },
};

function createBaseClientMessage(): ClientMessage {
  return { clientId: undefined, gameInputMessage: undefined };
}

export const ClientMessage = {
  encode(message: ClientMessage, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.clientId !== undefined) {
      ClientID.encode(message.clientId, writer.uint32(10).fork()).ldelim();
    }
    if (message.gameInputMessage !== undefined) {
      Any.encode(message.gameInputMessage, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ClientMessage {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseClientMessage();
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

          message.gameInputMessage = Any.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ClientMessage {
    return {
      clientId: isSet(object.clientId) ? ClientID.fromJSON(object.clientId) : undefined,
      gameInputMessage: isSet(object.gameInputMessage) ? Any.fromJSON(object.gameInputMessage) : undefined,
    };
  },

  toJSON(message: ClientMessage): unknown {
    const obj: any = {};
    if (message.clientId !== undefined) {
      obj.clientId = ClientID.toJSON(message.clientId);
    }
    if (message.gameInputMessage !== undefined) {
      obj.gameInputMessage = Any.toJSON(message.gameInputMessage);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ClientMessage>, I>>(base?: I): ClientMessage {
    return ClientMessage.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ClientMessage>, I>>(object: I): ClientMessage {
    const message = createBaseClientMessage();
    message.clientId = (object.clientId !== undefined && object.clientId !== null)
      ? ClientID.fromPartial(object.clientId)
      : undefined;
    message.gameInputMessage = (object.gameInputMessage !== undefined && object.gameInputMessage !== null)
      ? Any.fromPartial(object.gameInputMessage)
      : undefined;
    return message;
  },
};

function createBaseInstanceDirectMessage(): InstanceDirectMessage {
  return { clientId: undefined, gameOutputMessage: undefined };
}

export const InstanceDirectMessage = {
  encode(message: InstanceDirectMessage, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.clientId !== undefined) {
      ClientID.encode(message.clientId, writer.uint32(10).fork()).ldelim();
    }
    if (message.gameOutputMessage !== undefined) {
      Any.encode(message.gameOutputMessage, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): InstanceDirectMessage {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseInstanceDirectMessage();
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

  fromJSON(object: any): InstanceDirectMessage {
    return {
      clientId: isSet(object.clientId) ? ClientID.fromJSON(object.clientId) : undefined,
      gameOutputMessage: isSet(object.gameOutputMessage) ? Any.fromJSON(object.gameOutputMessage) : undefined,
    };
  },

  toJSON(message: InstanceDirectMessage): unknown {
    const obj: any = {};
    if (message.clientId !== undefined) {
      obj.clientId = ClientID.toJSON(message.clientId);
    }
    if (message.gameOutputMessage !== undefined) {
      obj.gameOutputMessage = Any.toJSON(message.gameOutputMessage);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<InstanceDirectMessage>, I>>(base?: I): InstanceDirectMessage {
    return InstanceDirectMessage.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<InstanceDirectMessage>, I>>(object: I): InstanceDirectMessage {
    const message = createBaseInstanceDirectMessage();
    message.clientId = (object.clientId !== undefined && object.clientId !== null)
      ? ClientID.fromPartial(object.clientId)
      : undefined;
    message.gameOutputMessage = (object.gameOutputMessage !== undefined && object.gameOutputMessage !== null)
      ? Any.fromPartial(object.gameOutputMessage)
      : undefined;
    return message;
  },
};

function createBaseInstanceBroadcast(): InstanceBroadcast {
  return { gameOutputMessage: undefined };
}

export const InstanceBroadcast = {
  encode(message: InstanceBroadcast, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameOutputMessage !== undefined) {
      Any.encode(message.gameOutputMessage, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): InstanceBroadcast {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseInstanceBroadcast();
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

  fromJSON(object: any): InstanceBroadcast {
    return { gameOutputMessage: isSet(object.gameOutputMessage) ? Any.fromJSON(object.gameOutputMessage) : undefined };
  },

  toJSON(message: InstanceBroadcast): unknown {
    const obj: any = {};
    if (message.gameOutputMessage !== undefined) {
      obj.gameOutputMessage = Any.toJSON(message.gameOutputMessage);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<InstanceBroadcast>, I>>(base?: I): InstanceBroadcast {
    return InstanceBroadcast.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<InstanceBroadcast>, I>>(object: I): InstanceBroadcast {
    const message = createBaseInstanceBroadcast();
    message.gameOutputMessage = (object.gameOutputMessage !== undefined && object.gameOutputMessage !== null)
      ? Any.fromPartial(object.gameOutputMessage)
      : undefined;
    return message;
  },
};

function createBaseInstanceMessage(): InstanceMessage {
  return { directMessages: [], broadcast: undefined };
}

export const InstanceMessage = {
  encode(message: InstanceMessage, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.directMessages) {
      InstanceDirectMessage.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    if (message.broadcast !== undefined) {
      InstanceBroadcast.encode(message.broadcast, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): InstanceMessage {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseInstanceMessage();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.directMessages.push(InstanceDirectMessage.decode(reader, reader.uint32()));
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.broadcast = InstanceBroadcast.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): InstanceMessage {
    return {
      directMessages: globalThis.Array.isArray(object?.directMessages)
        ? object.directMessages.map((e: any) => InstanceDirectMessage.fromJSON(e))
        : [],
      broadcast: isSet(object.broadcast) ? InstanceBroadcast.fromJSON(object.broadcast) : undefined,
    };
  },

  toJSON(message: InstanceMessage): unknown {
    const obj: any = {};
    if (message.directMessages?.length) {
      obj.directMessages = message.directMessages.map((e) => InstanceDirectMessage.toJSON(e));
    }
    if (message.broadcast !== undefined) {
      obj.broadcast = InstanceBroadcast.toJSON(message.broadcast);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<InstanceMessage>, I>>(base?: I): InstanceMessage {
    return InstanceMessage.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<InstanceMessage>, I>>(object: I): InstanceMessage {
    const message = createBaseInstanceMessage();
    message.directMessages = object.directMessages?.map((e) => InstanceDirectMessage.fromPartial(e)) || [];
    message.broadcast = (object.broadcast !== undefined && object.broadcast !== null)
      ? InstanceBroadcast.fromPartial(object.broadcast)
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
