/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { Drawable } from "./drawable";
import { floatArray } from "./gl_types";

export const protobufPackage = "snapshot";

export enum UpdateType {
  Reset = 0,
  Increment = 1,
  UNRECOGNIZED = -1,
}

export function updateTypeFromJSON(object: any): UpdateType {
  switch (object) {
    case 0:
    case "Reset":
      return UpdateType.Reset;
    case 1:
    case "Increment":
      return UpdateType.Increment;
    case -1:
    case "UNRECOGNIZED":
    default:
      return UpdateType.UNRECOGNIZED;
  }
}

export function updateTypeToJSON(object: UpdateType): string {
  switch (object) {
    case UpdateType.Reset:
      return "Reset";
    case UpdateType.Increment:
      return "Increment";
    case UpdateType.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface Entity {
  id: string;
  drawable?: Drawable | undefined;
}

export interface Camera {
  view?: floatArray | undefined;
  proj?: floatArray | undefined;
}

export interface Snapshot {
  updateType?: UpdateType | undefined;
  entities: Entity[];
}

function createBaseEntity(): Entity {
  return { id: "", drawable: undefined };
}

export const Entity = {
  encode(message: Entity, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== "") {
      writer.uint32(10).string(message.id);
    }
    if (message.drawable !== undefined) {
      Drawable.encode(message.drawable, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Entity {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEntity();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.id = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.drawable = Drawable.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Entity {
    return {
      id: isSet(object.id) ? globalThis.String(object.id) : "",
      drawable: isSet(object.drawable) ? Drawable.fromJSON(object.drawable) : undefined,
    };
  },

  toJSON(message: Entity): unknown {
    const obj: any = {};
    if (message.id !== "") {
      obj.id = message.id;
    }
    if (message.drawable !== undefined) {
      obj.drawable = Drawable.toJSON(message.drawable);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Entity>, I>>(base?: I): Entity {
    return Entity.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Entity>, I>>(object: I): Entity {
    const message = createBaseEntity();
    message.id = object.id ?? "";
    message.drawable = (object.drawable !== undefined && object.drawable !== null)
      ? Drawable.fromPartial(object.drawable)
      : undefined;
    return message;
  },
};

function createBaseCamera(): Camera {
  return { view: undefined, proj: undefined };
}

export const Camera = {
  encode(message: Camera, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.view !== undefined) {
      floatArray.encode(message.view, writer.uint32(10).fork()).ldelim();
    }
    if (message.proj !== undefined) {
      floatArray.encode(message.proj, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Camera {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCamera();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.view = floatArray.decode(reader, reader.uint32());
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.proj = floatArray.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Camera {
    return {
      view: isSet(object.view) ? floatArray.fromJSON(object.view) : undefined,
      proj: isSet(object.proj) ? floatArray.fromJSON(object.proj) : undefined,
    };
  },

  toJSON(message: Camera): unknown {
    const obj: any = {};
    if (message.view !== undefined) {
      obj.view = floatArray.toJSON(message.view);
    }
    if (message.proj !== undefined) {
      obj.proj = floatArray.toJSON(message.proj);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Camera>, I>>(base?: I): Camera {
    return Camera.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Camera>, I>>(object: I): Camera {
    const message = createBaseCamera();
    message.view = (object.view !== undefined && object.view !== null)
      ? floatArray.fromPartial(object.view)
      : undefined;
    message.proj = (object.proj !== undefined && object.proj !== null)
      ? floatArray.fromPartial(object.proj)
      : undefined;
    return message;
  },
};

function createBaseSnapshot(): Snapshot {
  return { updateType: undefined, entities: [] };
}

export const Snapshot = {
  encode(message: Snapshot, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.updateType !== undefined) {
      writer.uint32(8).int32(message.updateType);
    }
    for (const v of message.entities) {
      Entity.encode(v!, writer.uint32(18).fork()).ldelim();
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
          if (tag !== 8) {
            break;
          }

          message.updateType = reader.int32() as any;
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.entities.push(Entity.decode(reader, reader.uint32()));
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
      updateType: isSet(object.updateType) ? updateTypeFromJSON(object.updateType) : undefined,
      entities: globalThis.Array.isArray(object?.entities) ? object.entities.map((e: any) => Entity.fromJSON(e)) : [],
    };
  },

  toJSON(message: Snapshot): unknown {
    const obj: any = {};
    if (message.updateType !== undefined) {
      obj.updateType = updateTypeToJSON(message.updateType);
    }
    if (message.entities?.length) {
      obj.entities = message.entities.map((e) => Entity.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Snapshot>, I>>(base?: I): Snapshot {
    return Snapshot.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Snapshot>, I>>(object: I): Snapshot {
    const message = createBaseSnapshot();
    message.updateType = object.updateType ?? undefined;
    message.entities = object.entities?.map((e) => Entity.fromPartial(e)) || [];
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
