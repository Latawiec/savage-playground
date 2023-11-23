/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "gl_types";

export interface floatArray {
  values: number[];
}

export interface uint32Array {
  values: number[];
}

function createBasefloatArray(): floatArray {
  return { values: [] };
}

export const floatArray = {
  encode(message: floatArray, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    writer.uint32(10).fork();
    for (const v of message.values) {
      writer.float(v);
    }
    writer.ldelim();
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): floatArray {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasefloatArray();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag === 13) {
            message.values.push(reader.float());

            continue;
          }

          if (tag === 10) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.values.push(reader.float());
            }

            continue;
          }

          break;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): floatArray {
    return {
      values: globalThis.Array.isArray(object?.values) ? object.values.map((e: any) => globalThis.Number(e)) : [],
    };
  },

  toJSON(message: floatArray): unknown {
    const obj: any = {};
    if (message.values?.length) {
      obj.values = message.values;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<floatArray>, I>>(base?: I): floatArray {
    return floatArray.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<floatArray>, I>>(object: I): floatArray {
    const message = createBasefloatArray();
    message.values = object.values?.map((e) => e) || [];
    return message;
  },
};

function createBaseuint32Array(): uint32Array {
  return { values: [] };
}

export const uint32Array = {
  encode(message: uint32Array, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    writer.uint32(10).fork();
    for (const v of message.values) {
      writer.uint32(v);
    }
    writer.ldelim();
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): uint32Array {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseuint32Array();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag === 8) {
            message.values.push(reader.uint32());

            continue;
          }

          if (tag === 10) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.values.push(reader.uint32());
            }

            continue;
          }

          break;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): uint32Array {
    return {
      values: globalThis.Array.isArray(object?.values) ? object.values.map((e: any) => globalThis.Number(e)) : [],
    };
  },

  toJSON(message: uint32Array): unknown {
    const obj: any = {};
    if (message.values?.length) {
      obj.values = message.values.map((e) => Math.round(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<uint32Array>, I>>(base?: I): uint32Array {
    return uint32Array.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<uint32Array>, I>>(object: I): uint32Array {
    const message = createBaseuint32Array();
    message.values = object.values?.map((e) => e) || [];
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
