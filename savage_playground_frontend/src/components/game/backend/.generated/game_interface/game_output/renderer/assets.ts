/* eslint-disable */
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "assets";

export interface Texture {
  offset: number;
  asset?: string | undefined;
}

export interface Assets {
  vertexShaderAsset?: string | undefined;
  pixelShaderAsset?: string | undefined;
  meshAsset?: string | undefined;
  textures: Texture[];
}

function createBaseTexture(): Texture {
  return { offset: 0, asset: undefined };
}

export const Texture = {
  encode(message: Texture, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.offset !== 0) {
      writer.uint32(8).uint32(message.offset);
    }
    if (message.asset !== undefined) {
      writer.uint32(18).string(message.asset);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Texture {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTexture();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.offset = reader.uint32();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.asset = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Texture {
    return {
      offset: isSet(object.offset) ? globalThis.Number(object.offset) : 0,
      asset: isSet(object.asset) ? globalThis.String(object.asset) : undefined,
    };
  },

  toJSON(message: Texture): unknown {
    const obj: any = {};
    if (message.offset !== 0) {
      obj.offset = Math.round(message.offset);
    }
    if (message.asset !== undefined) {
      obj.asset = message.asset;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Texture>, I>>(base?: I): Texture {
    return Texture.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Texture>, I>>(object: I): Texture {
    const message = createBaseTexture();
    message.offset = object.offset ?? 0;
    message.asset = object.asset ?? undefined;
    return message;
  },
};

function createBaseAssets(): Assets {
  return { vertexShaderAsset: undefined, pixelShaderAsset: undefined, meshAsset: undefined, textures: [] };
}

export const Assets = {
  encode(message: Assets, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.vertexShaderAsset !== undefined) {
      writer.uint32(10).string(message.vertexShaderAsset);
    }
    if (message.pixelShaderAsset !== undefined) {
      writer.uint32(18).string(message.pixelShaderAsset);
    }
    if (message.meshAsset !== undefined) {
      writer.uint32(26).string(message.meshAsset);
    }
    for (const v of message.textures) {
      Texture.encode(v!, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Assets {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAssets();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.vertexShaderAsset = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.pixelShaderAsset = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.meshAsset = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.textures.push(Texture.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Assets {
    return {
      vertexShaderAsset: isSet(object.vertexShaderAsset) ? globalThis.String(object.vertexShaderAsset) : undefined,
      pixelShaderAsset: isSet(object.pixelShaderAsset) ? globalThis.String(object.pixelShaderAsset) : undefined,
      meshAsset: isSet(object.meshAsset) ? globalThis.String(object.meshAsset) : undefined,
      textures: globalThis.Array.isArray(object?.textures) ? object.textures.map((e: any) => Texture.fromJSON(e)) : [],
    };
  },

  toJSON(message: Assets): unknown {
    const obj: any = {};
    if (message.vertexShaderAsset !== undefined) {
      obj.vertexShaderAsset = message.vertexShaderAsset;
    }
    if (message.pixelShaderAsset !== undefined) {
      obj.pixelShaderAsset = message.pixelShaderAsset;
    }
    if (message.meshAsset !== undefined) {
      obj.meshAsset = message.meshAsset;
    }
    if (message.textures?.length) {
      obj.textures = message.textures.map((e) => Texture.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Assets>, I>>(base?: I): Assets {
    return Assets.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Assets>, I>>(object: I): Assets {
    const message = createBaseAssets();
    message.vertexShaderAsset = object.vertexShaderAsset ?? undefined;
    message.pixelShaderAsset = object.pixelShaderAsset ?? undefined;
    message.meshAsset = object.meshAsset ?? undefined;
    message.textures = object.textures?.map((e) => Texture.fromPartial(e)) || [];
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
