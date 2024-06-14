/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { Assets } from "./assets";
import { Window } from "./window";

export const protobufPackage = "settings";

export interface Snapshot {
  assets?: Assets | undefined;
  window?: Window | undefined;
}

function createBaseSnapshot(): Snapshot {
  return { assets: undefined, window: undefined };
}

export const Snapshot = {
  encode(message: Snapshot, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.assets !== undefined) {
      Assets.encode(message.assets, writer.uint32(10).fork()).ldelim();
    }
    if (message.window !== undefined) {
      Window.encode(message.window, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Snapshot {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSnapshot();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.assets = Assets.decode(reader, reader.uint32());
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.window = Window.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Snapshot {
    return {
      assets: isSet(object.assets) ? Assets.fromJSON(object.assets) : undefined,
      window: isSet(object.window) ? Window.fromJSON(object.window) : undefined,
    };
  },

  toJSON(message: Snapshot): unknown {
    const obj: any = {};
    if (message.assets !== undefined) {
      obj.assets = Assets.toJSON(message.assets);
    }
    if (message.window !== undefined) {
      obj.window = Window.toJSON(message.window);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Snapshot>, I>>(base?: I): Snapshot {
    return Snapshot.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Snapshot>, I>>(object: I): Snapshot {
    const message = createBaseSnapshot();
    message.assets = (object.assets !== undefined && object.assets !== null)
      ? Assets.fromPartial(object.assets)
      : undefined;
    message.window = (object.window !== undefined && object.window !== null)
      ? Window.fromPartial(object.window)
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
