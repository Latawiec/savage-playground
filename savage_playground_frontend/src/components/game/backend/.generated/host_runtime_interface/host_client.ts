/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { Any } from "./google/protobuf/any";

export const protobufPackage = "host_client";

/**
 * Origin: Client
 * Target: Host
 */
export interface ClientMessage {
  gameInputMessage: Any | undefined;
}

/**
 * Origin: Host
 * Target: Client
 */
export interface HostMessage {
  gameOutputMessage: Any | undefined;
}

function createBaseClientMessage(): ClientMessage {
  return { gameInputMessage: undefined };
}

export const ClientMessage = {
  encode(message: ClientMessage, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameInputMessage !== undefined) {
      Any.encode(message.gameInputMessage, writer.uint32(10).fork()).ldelim();
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
    return { gameInputMessage: isSet(object.gameInputMessage) ? Any.fromJSON(object.gameInputMessage) : undefined };
  },

  toJSON(message: ClientMessage): unknown {
    const obj: any = {};
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
    message.gameInputMessage = (object.gameInputMessage !== undefined && object.gameInputMessage !== null)
      ? Any.fromPartial(object.gameInputMessage)
      : undefined;
    return message;
  },
};

function createBaseHostMessage(): HostMessage {
  return { gameOutputMessage: undefined };
}

export const HostMessage = {
  encode(message: HostMessage, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameOutputMessage !== undefined) {
      Any.encode(message.gameOutputMessage, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): HostMessage {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseHostMessage();
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

  fromJSON(object: any): HostMessage {
    return { gameOutputMessage: isSet(object.gameOutputMessage) ? Any.fromJSON(object.gameOutputMessage) : undefined };
  },

  toJSON(message: HostMessage): unknown {
    const obj: any = {};
    if (message.gameOutputMessage !== undefined) {
      obj.gameOutputMessage = Any.toJSON(message.gameOutputMessage);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<HostMessage>, I>>(base?: I): HostMessage {
    return HostMessage.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<HostMessage>, I>>(object: I): HostMessage {
    const message = createBaseHostMessage();
    message.gameOutputMessage = (object.gameOutputMessage !== undefined && object.gameOutputMessage !== null)
      ? Any.fromPartial(object.gameOutputMessage)
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
