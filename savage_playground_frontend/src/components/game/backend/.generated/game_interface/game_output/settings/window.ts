/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "window";

export interface Window {
  aspectRatio?: number | undefined;
  rtWidth?: number | undefined;
  rtHeight?: number | undefined;
}

function createBaseWindow(): Window {
  return { aspectRatio: undefined, rtWidth: undefined, rtHeight: undefined };
}

export const Window = {
  encode(message: Window, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.aspectRatio !== undefined) {
      writer.uint32(13).float(message.aspectRatio);
    }
    if (message.rtWidth !== undefined) {
      writer.uint32(16).uint32(message.rtWidth);
    }
    if (message.rtHeight !== undefined) {
      writer.uint32(24).uint32(message.rtHeight);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Window {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseWindow();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 13) {
            break;
          }

          message.aspectRatio = reader.float();
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.rtWidth = reader.uint32();
          continue;
        case 3:
          if (tag !== 24) {
            break;
          }

          message.rtHeight = reader.uint32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Window {
    return {
      aspectRatio: isSet(object.aspectRatio) ? globalThis.Number(object.aspectRatio) : undefined,
      rtWidth: isSet(object.rtWidth) ? globalThis.Number(object.rtWidth) : undefined,
      rtHeight: isSet(object.rtHeight) ? globalThis.Number(object.rtHeight) : undefined,
    };
  },

  toJSON(message: Window): unknown {
    const obj: any = {};
    if (message.aspectRatio !== undefined) {
      obj.aspectRatio = message.aspectRatio;
    }
    if (message.rtWidth !== undefined) {
      obj.rtWidth = Math.round(message.rtWidth);
    }
    if (message.rtHeight !== undefined) {
      obj.rtHeight = Math.round(message.rtHeight);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Window>, I>>(base?: I): Window {
    return Window.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Window>, I>>(object: I): Window {
    const message = createBaseWindow();
    message.aspectRatio = object.aspectRatio ?? undefined;
    message.rtWidth = object.rtWidth ?? undefined;
    message.rtHeight = object.rtHeight ?? undefined;
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
