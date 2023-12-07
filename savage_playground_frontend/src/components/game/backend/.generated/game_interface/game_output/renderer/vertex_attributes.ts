/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "vertex_attributes";

export interface VertexAttributes {
  vertices?: string | undefined;
  namedBuffers: { [key: string]: string };
}

export interface VertexAttributes_NamedBuffersEntry {
  key: string;
  value: string;
}

function createBaseVertexAttributes(): VertexAttributes {
  return { vertices: undefined, namedBuffers: {} };
}

export const VertexAttributes = {
  encode(message: VertexAttributes, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.vertices !== undefined) {
      writer.uint32(10).string(message.vertices);
    }
    Object.entries(message.namedBuffers).forEach(([key, value]) => {
      VertexAttributes_NamedBuffersEntry.encode({ key: key as any, value }, writer.uint32(18).fork()).ldelim();
    });
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): VertexAttributes {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseVertexAttributes();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.vertices = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          const entry2 = VertexAttributes_NamedBuffersEntry.decode(reader, reader.uint32());
          if (entry2.value !== undefined) {
            message.namedBuffers[entry2.key] = entry2.value;
          }
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): VertexAttributes {
    return {
      vertices: isSet(object.vertices) ? globalThis.String(object.vertices) : undefined,
      namedBuffers: isObject(object.namedBuffers)
        ? Object.entries(object.namedBuffers).reduce<{ [key: string]: string }>((acc, [key, value]) => {
          acc[key] = String(value);
          return acc;
        }, {})
        : {},
    };
  },

  toJSON(message: VertexAttributes): unknown {
    const obj: any = {};
    if (message.vertices !== undefined) {
      obj.vertices = message.vertices;
    }
    if (message.namedBuffers) {
      const entries = Object.entries(message.namedBuffers);
      if (entries.length > 0) {
        obj.namedBuffers = {};
        entries.forEach(([k, v]) => {
          obj.namedBuffers[k] = v;
        });
      }
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<VertexAttributes>, I>>(base?: I): VertexAttributes {
    return VertexAttributes.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<VertexAttributes>, I>>(object: I): VertexAttributes {
    const message = createBaseVertexAttributes();
    message.vertices = object.vertices ?? undefined;
    message.namedBuffers = Object.entries(object.namedBuffers ?? {}).reduce<{ [key: string]: string }>(
      (acc, [key, value]) => {
        if (value !== undefined) {
          acc[key] = globalThis.String(value);
        }
        return acc;
      },
      {},
    );
    return message;
  },
};

function createBaseVertexAttributes_NamedBuffersEntry(): VertexAttributes_NamedBuffersEntry {
  return { key: "", value: "" };
}

export const VertexAttributes_NamedBuffersEntry = {
  encode(message: VertexAttributes_NamedBuffersEntry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== "") {
      writer.uint32(18).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): VertexAttributes_NamedBuffersEntry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseVertexAttributes_NamedBuffersEntry();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.key = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
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

  fromJSON(object: any): VertexAttributes_NamedBuffersEntry {
    return {
      key: isSet(object.key) ? globalThis.String(object.key) : "",
      value: isSet(object.value) ? globalThis.String(object.value) : "",
    };
  },

  toJSON(message: VertexAttributes_NamedBuffersEntry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== "") {
      obj.value = message.value;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<VertexAttributes_NamedBuffersEntry>, I>>(
    base?: I,
  ): VertexAttributes_NamedBuffersEntry {
    return VertexAttributes_NamedBuffersEntry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<VertexAttributes_NamedBuffersEntry>, I>>(
    object: I,
  ): VertexAttributes_NamedBuffersEntry {
    const message = createBaseVertexAttributes_NamedBuffersEntry();
    message.key = object.key ?? "";
    message.value = object.value ?? "";
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

function isObject(value: any): boolean {
  return typeof value === "object" && value !== null;
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
