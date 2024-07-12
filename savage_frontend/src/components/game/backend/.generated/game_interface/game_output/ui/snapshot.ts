/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { Any } from "../google/protobuf/any";

export const protobufPackage = "ui";

export interface UiSnapshot {
  typeDesc: string;
  data?: Any | undefined;
}

function createBaseUiSnapshot(): UiSnapshot {
  return { typeDesc: "", data: undefined };
}

export const UiSnapshot = {
  encode(message: UiSnapshot, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.typeDesc !== "") {
      writer.uint32(10).string(message.typeDesc);
    }
    if (message.data !== undefined) {
      Any.encode(message.data, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UiSnapshot {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUiSnapshot();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.typeDesc = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.data = Any.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UiSnapshot {
    return {
      typeDesc: isSet(object.typeDesc) ? globalThis.String(object.typeDesc) : "",
      data: isSet(object.data) ? Any.fromJSON(object.data) : undefined,
    };
  },

  toJSON(message: UiSnapshot): unknown {
    const obj: any = {};
    if (message.typeDesc !== "") {
      obj.typeDesc = message.typeDesc;
    }
    if (message.data !== undefined) {
      obj.data = Any.toJSON(message.data);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UiSnapshot>, I>>(base?: I): UiSnapshot {
    return UiSnapshot.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UiSnapshot>, I>>(object: I): UiSnapshot {
    const message = createBaseUiSnapshot();
    message.typeDesc = object.typeDesc ?? "";
    message.data = (object.data !== undefined && object.data !== null) ? Any.fromPartial(object.data) : undefined;
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
