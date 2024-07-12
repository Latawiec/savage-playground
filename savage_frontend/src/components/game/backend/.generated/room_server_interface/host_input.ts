/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { ClientID } from "./common";

export const protobufPackage = "host_input";

export interface HostInput {
  joined?: ClientID | undefined;
  left?: ClientID | undefined;
}

function createBaseHostInput(): HostInput {
  return { joined: undefined, left: undefined };
}

export const HostInput = {
  encode(message: HostInput, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.joined !== undefined) {
      ClientID.encode(message.joined, writer.uint32(10).fork()).ldelim();
    }
    if (message.left !== undefined) {
      ClientID.encode(message.left, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): HostInput {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseHostInput();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.joined = ClientID.decode(reader, reader.uint32());
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.left = ClientID.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): HostInput {
    return {
      joined: isSet(object.joined) ? ClientID.fromJSON(object.joined) : undefined,
      left: isSet(object.left) ? ClientID.fromJSON(object.left) : undefined,
    };
  },

  toJSON(message: HostInput): unknown {
    const obj: any = {};
    if (message.joined !== undefined) {
      obj.joined = ClientID.toJSON(message.joined);
    }
    if (message.left !== undefined) {
      obj.left = ClientID.toJSON(message.left);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<HostInput>, I>>(base?: I): HostInput {
    return HostInput.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<HostInput>, I>>(object: I): HostInput {
    const message = createBaseHostInput();
    message.joined = (object.joined !== undefined && object.joined !== null)
      ? ClientID.fromPartial(object.joined)
      : undefined;
    message.left = (object.left !== undefined && object.left !== null) ? ClientID.fromPartial(object.left) : undefined;
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
