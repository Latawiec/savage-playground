/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { Assets } from "./assets";
import { SharedUniformAttributes, UniformAttributes } from "./uniform_attributes";
import { VertexAttributes } from "./vertex_attributes";

export const protobufPackage = "renderer";

export interface Drawable {
  layer?: number | undefined;
  vertexAttributes?: VertexAttributes | undefined;
  uniformAttributes?: UniformAttributes | undefined;
  sharedAttributes?: SharedUniformAttributes | undefined;
  assets?: Assets | undefined;
}

function createBaseDrawable(): Drawable {
  return {
    layer: undefined,
    vertexAttributes: undefined,
    uniformAttributes: undefined,
    sharedAttributes: undefined,
    assets: undefined,
  };
}

export const Drawable = {
  encode(message: Drawable, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.layer !== undefined) {
      writer.uint32(8).uint32(message.layer);
    }
    if (message.vertexAttributes !== undefined) {
      VertexAttributes.encode(message.vertexAttributes, writer.uint32(18).fork()).ldelim();
    }
    if (message.uniformAttributes !== undefined) {
      UniformAttributes.encode(message.uniformAttributes, writer.uint32(26).fork()).ldelim();
    }
    if (message.sharedAttributes !== undefined) {
      SharedUniformAttributes.encode(message.sharedAttributes, writer.uint32(34).fork()).ldelim();
    }
    if (message.assets !== undefined) {
      Assets.encode(message.assets, writer.uint32(42).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Drawable {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDrawable();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.layer = reader.uint32();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.vertexAttributes = VertexAttributes.decode(reader, reader.uint32());
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.uniformAttributes = UniformAttributes.decode(reader, reader.uint32());
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.sharedAttributes = SharedUniformAttributes.decode(reader, reader.uint32());
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.assets = Assets.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Drawable {
    return {
      layer: isSet(object.layer) ? globalThis.Number(object.layer) : undefined,
      vertexAttributes: isSet(object.vertexAttributes) ? VertexAttributes.fromJSON(object.vertexAttributes) : undefined,
      uniformAttributes: isSet(object.uniformAttributes)
        ? UniformAttributes.fromJSON(object.uniformAttributes)
        : undefined,
      sharedAttributes: isSet(object.sharedAttributes)
        ? SharedUniformAttributes.fromJSON(object.sharedAttributes)
        : undefined,
      assets: isSet(object.assets) ? Assets.fromJSON(object.assets) : undefined,
    };
  },

  toJSON(message: Drawable): unknown {
    const obj: any = {};
    if (message.layer !== undefined) {
      obj.layer = Math.round(message.layer);
    }
    if (message.vertexAttributes !== undefined) {
      obj.vertexAttributes = VertexAttributes.toJSON(message.vertexAttributes);
    }
    if (message.uniformAttributes !== undefined) {
      obj.uniformAttributes = UniformAttributes.toJSON(message.uniformAttributes);
    }
    if (message.sharedAttributes !== undefined) {
      obj.sharedAttributes = SharedUniformAttributes.toJSON(message.sharedAttributes);
    }
    if (message.assets !== undefined) {
      obj.assets = Assets.toJSON(message.assets);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Drawable>, I>>(base?: I): Drawable {
    return Drawable.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Drawable>, I>>(object: I): Drawable {
    const message = createBaseDrawable();
    message.layer = object.layer ?? undefined;
    message.vertexAttributes = (object.vertexAttributes !== undefined && object.vertexAttributes !== null)
      ? VertexAttributes.fromPartial(object.vertexAttributes)
      : undefined;
    message.uniformAttributes = (object.uniformAttributes !== undefined && object.uniformAttributes !== null)
      ? UniformAttributes.fromPartial(object.uniformAttributes)
      : undefined;
    message.sharedAttributes = (object.sharedAttributes !== undefined && object.sharedAttributes !== null)
      ? SharedUniformAttributes.fromPartial(object.sharedAttributes)
      : undefined;
    message.assets = (object.assets !== undefined && object.assets !== null)
      ? Assets.fromPartial(object.assets)
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
