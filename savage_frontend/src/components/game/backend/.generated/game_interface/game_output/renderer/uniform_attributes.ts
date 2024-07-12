/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { floatArray, uint32Array } from "./gl_types";

export const protobufPackage = "renderer";

export interface UniformAttributes {
  float: { [key: string]: number };
  vec2: { [key: string]: floatArray };
  vec3: { [key: string]: floatArray };
  vec4: { [key: string]: floatArray };
  int: { [key: string]: number };
  ivec2: { [key: string]: uint32Array };
  ivec3: { [key: string]: uint32Array };
  ivec4: { [key: string]: uint32Array };
  mat4: { [key: string]: floatArray };
}

export interface UniformAttributes_FloatEntry {
  key: string;
  value: number;
}

export interface UniformAttributes_Vec2Entry {
  key: string;
  value: floatArray | undefined;
}

export interface UniformAttributes_Vec3Entry {
  key: string;
  value: floatArray | undefined;
}

export interface UniformAttributes_Vec4Entry {
  key: string;
  value: floatArray | undefined;
}

export interface UniformAttributes_IntEntry {
  key: string;
  value: number;
}

export interface UniformAttributes_Ivec2Entry {
  key: string;
  value: uint32Array | undefined;
}

export interface UniformAttributes_Ivec3Entry {
  key: string;
  value: uint32Array | undefined;
}

export interface UniformAttributes_Ivec4Entry {
  key: string;
  value: uint32Array | undefined;
}

export interface UniformAttributes_Mat4Entry {
  key: string;
  value: floatArray | undefined;
}

export interface CameraUniformAttributes {
  /** mat4 */
  cameraView?:
    | string
    | undefined;
  /** mat4 */
  cameraProj?:
    | string
    | undefined;
  /** vec3 */
  cameraForward?: string | undefined;
}

export interface SharedUniformAttributes {
  camera?: CameraUniformAttributes | undefined;
}

function createBaseUniformAttributes(): UniformAttributes {
  return { float: {}, vec2: {}, vec3: {}, vec4: {}, int: {}, ivec2: {}, ivec3: {}, ivec4: {}, mat4: {} };
}

export const UniformAttributes = {
  encode(message: UniformAttributes, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    Object.entries(message.float).forEach(([key, value]) => {
      UniformAttributes_FloatEntry.encode({ key: key as any, value }, writer.uint32(10).fork()).ldelim();
    });
    Object.entries(message.vec2).forEach(([key, value]) => {
      UniformAttributes_Vec2Entry.encode({ key: key as any, value }, writer.uint32(18).fork()).ldelim();
    });
    Object.entries(message.vec3).forEach(([key, value]) => {
      UniformAttributes_Vec3Entry.encode({ key: key as any, value }, writer.uint32(26).fork()).ldelim();
    });
    Object.entries(message.vec4).forEach(([key, value]) => {
      UniformAttributes_Vec4Entry.encode({ key: key as any, value }, writer.uint32(34).fork()).ldelim();
    });
    Object.entries(message.int).forEach(([key, value]) => {
      UniformAttributes_IntEntry.encode({ key: key as any, value }, writer.uint32(42).fork()).ldelim();
    });
    Object.entries(message.ivec2).forEach(([key, value]) => {
      UniformAttributes_Ivec2Entry.encode({ key: key as any, value }, writer.uint32(50).fork()).ldelim();
    });
    Object.entries(message.ivec3).forEach(([key, value]) => {
      UniformAttributes_Ivec3Entry.encode({ key: key as any, value }, writer.uint32(58).fork()).ldelim();
    });
    Object.entries(message.ivec4).forEach(([key, value]) => {
      UniformAttributes_Ivec4Entry.encode({ key: key as any, value }, writer.uint32(66).fork()).ldelim();
    });
    Object.entries(message.mat4).forEach(([key, value]) => {
      UniformAttributes_Mat4Entry.encode({ key: key as any, value }, writer.uint32(74).fork()).ldelim();
    });
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UniformAttributes {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUniformAttributes();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          const entry1 = UniformAttributes_FloatEntry.decode(reader, reader.uint32());
          if (entry1.value !== undefined) {
            message.float[entry1.key] = entry1.value;
          }
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          const entry2 = UniformAttributes_Vec2Entry.decode(reader, reader.uint32());
          if (entry2.value !== undefined) {
            message.vec2[entry2.key] = entry2.value;
          }
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          const entry3 = UniformAttributes_Vec3Entry.decode(reader, reader.uint32());
          if (entry3.value !== undefined) {
            message.vec3[entry3.key] = entry3.value;
          }
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          const entry4 = UniformAttributes_Vec4Entry.decode(reader, reader.uint32());
          if (entry4.value !== undefined) {
            message.vec4[entry4.key] = entry4.value;
          }
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          const entry5 = UniformAttributes_IntEntry.decode(reader, reader.uint32());
          if (entry5.value !== undefined) {
            message.int[entry5.key] = entry5.value;
          }
          continue;
        case 6:
          if (tag !== 50) {
            break;
          }

          const entry6 = UniformAttributes_Ivec2Entry.decode(reader, reader.uint32());
          if (entry6.value !== undefined) {
            message.ivec2[entry6.key] = entry6.value;
          }
          continue;
        case 7:
          if (tag !== 58) {
            break;
          }

          const entry7 = UniformAttributes_Ivec3Entry.decode(reader, reader.uint32());
          if (entry7.value !== undefined) {
            message.ivec3[entry7.key] = entry7.value;
          }
          continue;
        case 8:
          if (tag !== 66) {
            break;
          }

          const entry8 = UniformAttributes_Ivec4Entry.decode(reader, reader.uint32());
          if (entry8.value !== undefined) {
            message.ivec4[entry8.key] = entry8.value;
          }
          continue;
        case 9:
          if (tag !== 74) {
            break;
          }

          const entry9 = UniformAttributes_Mat4Entry.decode(reader, reader.uint32());
          if (entry9.value !== undefined) {
            message.mat4[entry9.key] = entry9.value;
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

  fromJSON(object: any): UniformAttributes {
    return {
      float: isObject(object.float)
        ? Object.entries(object.float).reduce<{ [key: string]: number }>((acc, [key, value]) => {
          acc[key] = Number(value);
          return acc;
        }, {})
        : {},
      vec2: isObject(object.vec2)
        ? Object.entries(object.vec2).reduce<{ [key: string]: floatArray }>((acc, [key, value]) => {
          acc[key] = floatArray.fromJSON(value);
          return acc;
        }, {})
        : {},
      vec3: isObject(object.vec3)
        ? Object.entries(object.vec3).reduce<{ [key: string]: floatArray }>((acc, [key, value]) => {
          acc[key] = floatArray.fromJSON(value);
          return acc;
        }, {})
        : {},
      vec4: isObject(object.vec4)
        ? Object.entries(object.vec4).reduce<{ [key: string]: floatArray }>((acc, [key, value]) => {
          acc[key] = floatArray.fromJSON(value);
          return acc;
        }, {})
        : {},
      int: isObject(object.int)
        ? Object.entries(object.int).reduce<{ [key: string]: number }>((acc, [key, value]) => {
          acc[key] = Number(value);
          return acc;
        }, {})
        : {},
      ivec2: isObject(object.ivec2)
        ? Object.entries(object.ivec2).reduce<{ [key: string]: uint32Array }>((acc, [key, value]) => {
          acc[key] = uint32Array.fromJSON(value);
          return acc;
        }, {})
        : {},
      ivec3: isObject(object.ivec3)
        ? Object.entries(object.ivec3).reduce<{ [key: string]: uint32Array }>((acc, [key, value]) => {
          acc[key] = uint32Array.fromJSON(value);
          return acc;
        }, {})
        : {},
      ivec4: isObject(object.ivec4)
        ? Object.entries(object.ivec4).reduce<{ [key: string]: uint32Array }>((acc, [key, value]) => {
          acc[key] = uint32Array.fromJSON(value);
          return acc;
        }, {})
        : {},
      mat4: isObject(object.mat4)
        ? Object.entries(object.mat4).reduce<{ [key: string]: floatArray }>((acc, [key, value]) => {
          acc[key] = floatArray.fromJSON(value);
          return acc;
        }, {})
        : {},
    };
  },

  toJSON(message: UniformAttributes): unknown {
    const obj: any = {};
    if (message.float) {
      const entries = Object.entries(message.float);
      if (entries.length > 0) {
        obj.float = {};
        entries.forEach(([k, v]) => {
          obj.float[k] = v;
        });
      }
    }
    if (message.vec2) {
      const entries = Object.entries(message.vec2);
      if (entries.length > 0) {
        obj.vec2 = {};
        entries.forEach(([k, v]) => {
          obj.vec2[k] = floatArray.toJSON(v);
        });
      }
    }
    if (message.vec3) {
      const entries = Object.entries(message.vec3);
      if (entries.length > 0) {
        obj.vec3 = {};
        entries.forEach(([k, v]) => {
          obj.vec3[k] = floatArray.toJSON(v);
        });
      }
    }
    if (message.vec4) {
      const entries = Object.entries(message.vec4);
      if (entries.length > 0) {
        obj.vec4 = {};
        entries.forEach(([k, v]) => {
          obj.vec4[k] = floatArray.toJSON(v);
        });
      }
    }
    if (message.int) {
      const entries = Object.entries(message.int);
      if (entries.length > 0) {
        obj.int = {};
        entries.forEach(([k, v]) => {
          obj.int[k] = Math.round(v);
        });
      }
    }
    if (message.ivec2) {
      const entries = Object.entries(message.ivec2);
      if (entries.length > 0) {
        obj.ivec2 = {};
        entries.forEach(([k, v]) => {
          obj.ivec2[k] = uint32Array.toJSON(v);
        });
      }
    }
    if (message.ivec3) {
      const entries = Object.entries(message.ivec3);
      if (entries.length > 0) {
        obj.ivec3 = {};
        entries.forEach(([k, v]) => {
          obj.ivec3[k] = uint32Array.toJSON(v);
        });
      }
    }
    if (message.ivec4) {
      const entries = Object.entries(message.ivec4);
      if (entries.length > 0) {
        obj.ivec4 = {};
        entries.forEach(([k, v]) => {
          obj.ivec4[k] = uint32Array.toJSON(v);
        });
      }
    }
    if (message.mat4) {
      const entries = Object.entries(message.mat4);
      if (entries.length > 0) {
        obj.mat4 = {};
        entries.forEach(([k, v]) => {
          obj.mat4[k] = floatArray.toJSON(v);
        });
      }
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UniformAttributes>, I>>(base?: I): UniformAttributes {
    return UniformAttributes.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UniformAttributes>, I>>(object: I): UniformAttributes {
    const message = createBaseUniformAttributes();
    message.float = Object.entries(object.float ?? {}).reduce<{ [key: string]: number }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = globalThis.Number(value);
      }
      return acc;
    }, {});
    message.vec2 = Object.entries(object.vec2 ?? {}).reduce<{ [key: string]: floatArray }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = floatArray.fromPartial(value);
      }
      return acc;
    }, {});
    message.vec3 = Object.entries(object.vec3 ?? {}).reduce<{ [key: string]: floatArray }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = floatArray.fromPartial(value);
      }
      return acc;
    }, {});
    message.vec4 = Object.entries(object.vec4 ?? {}).reduce<{ [key: string]: floatArray }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = floatArray.fromPartial(value);
      }
      return acc;
    }, {});
    message.int = Object.entries(object.int ?? {}).reduce<{ [key: string]: number }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = globalThis.Number(value);
      }
      return acc;
    }, {});
    message.ivec2 = Object.entries(object.ivec2 ?? {}).reduce<{ [key: string]: uint32Array }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = uint32Array.fromPartial(value);
      }
      return acc;
    }, {});
    message.ivec3 = Object.entries(object.ivec3 ?? {}).reduce<{ [key: string]: uint32Array }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = uint32Array.fromPartial(value);
      }
      return acc;
    }, {});
    message.ivec4 = Object.entries(object.ivec4 ?? {}).reduce<{ [key: string]: uint32Array }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = uint32Array.fromPartial(value);
      }
      return acc;
    }, {});
    message.mat4 = Object.entries(object.mat4 ?? {}).reduce<{ [key: string]: floatArray }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = floatArray.fromPartial(value);
      }
      return acc;
    }, {});
    return message;
  },
};

function createBaseUniformAttributes_FloatEntry(): UniformAttributes_FloatEntry {
  return { key: "", value: 0 };
}

export const UniformAttributes_FloatEntry = {
  encode(message: UniformAttributes_FloatEntry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== 0) {
      writer.uint32(21).float(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UniformAttributes_FloatEntry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUniformAttributes_FloatEntry();
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
          if (tag !== 21) {
            break;
          }

          message.value = reader.float();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UniformAttributes_FloatEntry {
    return {
      key: isSet(object.key) ? globalThis.String(object.key) : "",
      value: isSet(object.value) ? globalThis.Number(object.value) : 0,
    };
  },

  toJSON(message: UniformAttributes_FloatEntry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== 0) {
      obj.value = message.value;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UniformAttributes_FloatEntry>, I>>(base?: I): UniformAttributes_FloatEntry {
    return UniformAttributes_FloatEntry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UniformAttributes_FloatEntry>, I>>(object: I): UniformAttributes_FloatEntry {
    const message = createBaseUniformAttributes_FloatEntry();
    message.key = object.key ?? "";
    message.value = object.value ?? 0;
    return message;
  },
};

function createBaseUniformAttributes_Vec2Entry(): UniformAttributes_Vec2Entry {
  return { key: "", value: undefined };
}

export const UniformAttributes_Vec2Entry = {
  encode(message: UniformAttributes_Vec2Entry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      floatArray.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UniformAttributes_Vec2Entry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUniformAttributes_Vec2Entry();
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

          message.value = floatArray.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UniformAttributes_Vec2Entry {
    return {
      key: isSet(object.key) ? globalThis.String(object.key) : "",
      value: isSet(object.value) ? floatArray.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: UniformAttributes_Vec2Entry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = floatArray.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UniformAttributes_Vec2Entry>, I>>(base?: I): UniformAttributes_Vec2Entry {
    return UniformAttributes_Vec2Entry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UniformAttributes_Vec2Entry>, I>>(object: I): UniformAttributes_Vec2Entry {
    const message = createBaseUniformAttributes_Vec2Entry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null)
      ? floatArray.fromPartial(object.value)
      : undefined;
    return message;
  },
};

function createBaseUniformAttributes_Vec3Entry(): UniformAttributes_Vec3Entry {
  return { key: "", value: undefined };
}

export const UniformAttributes_Vec3Entry = {
  encode(message: UniformAttributes_Vec3Entry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      floatArray.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UniformAttributes_Vec3Entry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUniformAttributes_Vec3Entry();
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

          message.value = floatArray.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UniformAttributes_Vec3Entry {
    return {
      key: isSet(object.key) ? globalThis.String(object.key) : "",
      value: isSet(object.value) ? floatArray.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: UniformAttributes_Vec3Entry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = floatArray.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UniformAttributes_Vec3Entry>, I>>(base?: I): UniformAttributes_Vec3Entry {
    return UniformAttributes_Vec3Entry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UniformAttributes_Vec3Entry>, I>>(object: I): UniformAttributes_Vec3Entry {
    const message = createBaseUniformAttributes_Vec3Entry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null)
      ? floatArray.fromPartial(object.value)
      : undefined;
    return message;
  },
};

function createBaseUniformAttributes_Vec4Entry(): UniformAttributes_Vec4Entry {
  return { key: "", value: undefined };
}

export const UniformAttributes_Vec4Entry = {
  encode(message: UniformAttributes_Vec4Entry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      floatArray.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UniformAttributes_Vec4Entry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUniformAttributes_Vec4Entry();
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

          message.value = floatArray.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UniformAttributes_Vec4Entry {
    return {
      key: isSet(object.key) ? globalThis.String(object.key) : "",
      value: isSet(object.value) ? floatArray.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: UniformAttributes_Vec4Entry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = floatArray.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UniformAttributes_Vec4Entry>, I>>(base?: I): UniformAttributes_Vec4Entry {
    return UniformAttributes_Vec4Entry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UniformAttributes_Vec4Entry>, I>>(object: I): UniformAttributes_Vec4Entry {
    const message = createBaseUniformAttributes_Vec4Entry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null)
      ? floatArray.fromPartial(object.value)
      : undefined;
    return message;
  },
};

function createBaseUniformAttributes_IntEntry(): UniformAttributes_IntEntry {
  return { key: "", value: 0 };
}

export const UniformAttributes_IntEntry = {
  encode(message: UniformAttributes_IntEntry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== 0) {
      writer.uint32(16).uint32(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UniformAttributes_IntEntry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUniformAttributes_IntEntry();
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
          if (tag !== 16) {
            break;
          }

          message.value = reader.uint32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UniformAttributes_IntEntry {
    return {
      key: isSet(object.key) ? globalThis.String(object.key) : "",
      value: isSet(object.value) ? globalThis.Number(object.value) : 0,
    };
  },

  toJSON(message: UniformAttributes_IntEntry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== 0) {
      obj.value = Math.round(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UniformAttributes_IntEntry>, I>>(base?: I): UniformAttributes_IntEntry {
    return UniformAttributes_IntEntry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UniformAttributes_IntEntry>, I>>(object: I): UniformAttributes_IntEntry {
    const message = createBaseUniformAttributes_IntEntry();
    message.key = object.key ?? "";
    message.value = object.value ?? 0;
    return message;
  },
};

function createBaseUniformAttributes_Ivec2Entry(): UniformAttributes_Ivec2Entry {
  return { key: "", value: undefined };
}

export const UniformAttributes_Ivec2Entry = {
  encode(message: UniformAttributes_Ivec2Entry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      uint32Array.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UniformAttributes_Ivec2Entry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUniformAttributes_Ivec2Entry();
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

          message.value = uint32Array.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UniformAttributes_Ivec2Entry {
    return {
      key: isSet(object.key) ? globalThis.String(object.key) : "",
      value: isSet(object.value) ? uint32Array.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: UniformAttributes_Ivec2Entry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = uint32Array.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UniformAttributes_Ivec2Entry>, I>>(base?: I): UniformAttributes_Ivec2Entry {
    return UniformAttributes_Ivec2Entry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UniformAttributes_Ivec2Entry>, I>>(object: I): UniformAttributes_Ivec2Entry {
    const message = createBaseUniformAttributes_Ivec2Entry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null)
      ? uint32Array.fromPartial(object.value)
      : undefined;
    return message;
  },
};

function createBaseUniformAttributes_Ivec3Entry(): UniformAttributes_Ivec3Entry {
  return { key: "", value: undefined };
}

export const UniformAttributes_Ivec3Entry = {
  encode(message: UniformAttributes_Ivec3Entry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      uint32Array.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UniformAttributes_Ivec3Entry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUniformAttributes_Ivec3Entry();
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

          message.value = uint32Array.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UniformAttributes_Ivec3Entry {
    return {
      key: isSet(object.key) ? globalThis.String(object.key) : "",
      value: isSet(object.value) ? uint32Array.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: UniformAttributes_Ivec3Entry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = uint32Array.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UniformAttributes_Ivec3Entry>, I>>(base?: I): UniformAttributes_Ivec3Entry {
    return UniformAttributes_Ivec3Entry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UniformAttributes_Ivec3Entry>, I>>(object: I): UniformAttributes_Ivec3Entry {
    const message = createBaseUniformAttributes_Ivec3Entry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null)
      ? uint32Array.fromPartial(object.value)
      : undefined;
    return message;
  },
};

function createBaseUniformAttributes_Ivec4Entry(): UniformAttributes_Ivec4Entry {
  return { key: "", value: undefined };
}

export const UniformAttributes_Ivec4Entry = {
  encode(message: UniformAttributes_Ivec4Entry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      uint32Array.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UniformAttributes_Ivec4Entry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUniformAttributes_Ivec4Entry();
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

          message.value = uint32Array.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UniformAttributes_Ivec4Entry {
    return {
      key: isSet(object.key) ? globalThis.String(object.key) : "",
      value: isSet(object.value) ? uint32Array.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: UniformAttributes_Ivec4Entry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = uint32Array.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UniformAttributes_Ivec4Entry>, I>>(base?: I): UniformAttributes_Ivec4Entry {
    return UniformAttributes_Ivec4Entry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UniformAttributes_Ivec4Entry>, I>>(object: I): UniformAttributes_Ivec4Entry {
    const message = createBaseUniformAttributes_Ivec4Entry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null)
      ? uint32Array.fromPartial(object.value)
      : undefined;
    return message;
  },
};

function createBaseUniformAttributes_Mat4Entry(): UniformAttributes_Mat4Entry {
  return { key: "", value: undefined };
}

export const UniformAttributes_Mat4Entry = {
  encode(message: UniformAttributes_Mat4Entry, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      floatArray.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UniformAttributes_Mat4Entry {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUniformAttributes_Mat4Entry();
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

          message.value = floatArray.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): UniformAttributes_Mat4Entry {
    return {
      key: isSet(object.key) ? globalThis.String(object.key) : "",
      value: isSet(object.value) ? floatArray.fromJSON(object.value) : undefined,
    };
  },

  toJSON(message: UniformAttributes_Mat4Entry): unknown {
    const obj: any = {};
    if (message.key !== "") {
      obj.key = message.key;
    }
    if (message.value !== undefined) {
      obj.value = floatArray.toJSON(message.value);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<UniformAttributes_Mat4Entry>, I>>(base?: I): UniformAttributes_Mat4Entry {
    return UniformAttributes_Mat4Entry.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<UniformAttributes_Mat4Entry>, I>>(object: I): UniformAttributes_Mat4Entry {
    const message = createBaseUniformAttributes_Mat4Entry();
    message.key = object.key ?? "";
    message.value = (object.value !== undefined && object.value !== null)
      ? floatArray.fromPartial(object.value)
      : undefined;
    return message;
  },
};

function createBaseCameraUniformAttributes(): CameraUniformAttributes {
  return { cameraView: undefined, cameraProj: undefined, cameraForward: undefined };
}

export const CameraUniformAttributes = {
  encode(message: CameraUniformAttributes, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.cameraView !== undefined) {
      writer.uint32(10).string(message.cameraView);
    }
    if (message.cameraProj !== undefined) {
      writer.uint32(18).string(message.cameraProj);
    }
    if (message.cameraForward !== undefined) {
      writer.uint32(26).string(message.cameraForward);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CameraUniformAttributes {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCameraUniformAttributes();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.cameraView = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.cameraProj = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.cameraForward = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): CameraUniformAttributes {
    return {
      cameraView: isSet(object.cameraView) ? globalThis.String(object.cameraView) : undefined,
      cameraProj: isSet(object.cameraProj) ? globalThis.String(object.cameraProj) : undefined,
      cameraForward: isSet(object.cameraForward) ? globalThis.String(object.cameraForward) : undefined,
    };
  },

  toJSON(message: CameraUniformAttributes): unknown {
    const obj: any = {};
    if (message.cameraView !== undefined) {
      obj.cameraView = message.cameraView;
    }
    if (message.cameraProj !== undefined) {
      obj.cameraProj = message.cameraProj;
    }
    if (message.cameraForward !== undefined) {
      obj.cameraForward = message.cameraForward;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<CameraUniformAttributes>, I>>(base?: I): CameraUniformAttributes {
    return CameraUniformAttributes.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<CameraUniformAttributes>, I>>(object: I): CameraUniformAttributes {
    const message = createBaseCameraUniformAttributes();
    message.cameraView = object.cameraView ?? undefined;
    message.cameraProj = object.cameraProj ?? undefined;
    message.cameraForward = object.cameraForward ?? undefined;
    return message;
  },
};

function createBaseSharedUniformAttributes(): SharedUniformAttributes {
  return { camera: undefined };
}

export const SharedUniformAttributes = {
  encode(message: SharedUniformAttributes, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.camera !== undefined) {
      CameraUniformAttributes.encode(message.camera, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SharedUniformAttributes {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSharedUniformAttributes();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.camera = CameraUniformAttributes.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SharedUniformAttributes {
    return { camera: isSet(object.camera) ? CameraUniformAttributes.fromJSON(object.camera) : undefined };
  },

  toJSON(message: SharedUniformAttributes): unknown {
    const obj: any = {};
    if (message.camera !== undefined) {
      obj.camera = CameraUniformAttributes.toJSON(message.camera);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<SharedUniformAttributes>, I>>(base?: I): SharedUniformAttributes {
    return SharedUniformAttributes.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<SharedUniformAttributes>, I>>(object: I): SharedUniformAttributes {
    const message = createBaseSharedUniformAttributes();
    message.camera = (object.camera !== undefined && object.camera !== null)
      ? CameraUniformAttributes.fromPartial(object.camera)
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

function isObject(value: any): boolean {
  return typeof value === "object" && value !== null;
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
