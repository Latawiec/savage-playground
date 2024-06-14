/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { ClientID } from "./common";
import { Any } from "./google/protobuf/any";

export const protobufPackage = "client_input";

export interface ClientInput {
  clientId: ClientID | undefined;
  gameInputMessage: Any | undefined;
}

function createBaseClientInput(): ClientInput {
  return { clientId: undefined, gameInputMessage: undefined };
}

export const ClientInput = {
  encode(message: ClientInput, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.clientId !== undefined) {
      ClientID.encode(message.clientId, writer.uint32(10).fork()).ldelim();
    }
    if (message.gameInputMessage !== undefined) {
      Any.encode(message.gameInputMessage, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ClientInput {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseClientInput();
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

  fromJSON(object: any): ClientInput {
    return {
      clientId: isSet(object.clientId) ? ClientID.fromJSON(object.clientId) : undefined,
      gameInputMessage: isSet(object.gameInputMessage) ? Any.fromJSON(object.gameInputMessage) : undefined,
    };
  },

  toJSON(message: ClientInput): unknown {
    const obj: any = {};
    if (message.clientId !== undefined) {
      obj.clientId = ClientID.toJSON(message.clientId);
    }
    if (message.gameInputMessage !== undefined) {
      obj.gameInputMessage = Any.toJSON(message.gameInputMessage);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ClientInput>, I>>(base?: I): ClientInput {
    return ClientInput.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ClientInput>, I>>(object: I): ClientInput {
    const message = createBaseClientInput();
    message.clientId = (object.clientId !== undefined && object.clientId !== null)
      ? ClientID.fromPartial(object.clientId)
      : undefined;
    message.gameInputMessage = (object.gameInputMessage !== undefined && object.gameInputMessage !== null)
      ? Any.fromPartial(object.gameInputMessage)
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
