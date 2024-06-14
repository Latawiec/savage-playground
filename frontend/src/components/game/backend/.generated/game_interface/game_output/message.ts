/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { RendererSnapshot } from "./renderer/snapshot";
import { SettingsSnapshot } from "./settings/snapshot";
import { UiSnapshot } from "./ui/snapshot";

export const protobufPackage = "message";

export interface GameOutputMessage {
  renderer?: RendererSnapshot | undefined;
  settings?: SettingsSnapshot | undefined;
  ui?: UiSnapshot | undefined;
}

function createBaseGameOutputMessage(): GameOutputMessage {
  return { renderer: undefined, settings: undefined, ui: undefined };
}

export const GameOutputMessage = {
  encode(message: GameOutputMessage, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.renderer !== undefined) {
      RendererSnapshot.encode(message.renderer, writer.uint32(10).fork()).ldelim();
    }
    if (message.settings !== undefined) {
      SettingsSnapshot.encode(message.settings, writer.uint32(18).fork()).ldelim();
    }
    if (message.ui !== undefined) {
      UiSnapshot.encode(message.ui, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GameOutputMessage {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGameOutputMessage();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.renderer = RendererSnapshot.decode(reader, reader.uint32());
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.settings = SettingsSnapshot.decode(reader, reader.uint32());
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.ui = UiSnapshot.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): GameOutputMessage {
    return {
      renderer: isSet(object.renderer) ? RendererSnapshot.fromJSON(object.renderer) : undefined,
      settings: isSet(object.settings) ? SettingsSnapshot.fromJSON(object.settings) : undefined,
      ui: isSet(object.ui) ? UiSnapshot.fromJSON(object.ui) : undefined,
    };
  },

  toJSON(message: GameOutputMessage): unknown {
    const obj: any = {};
    if (message.renderer !== undefined) {
      obj.renderer = RendererSnapshot.toJSON(message.renderer);
    }
    if (message.settings !== undefined) {
      obj.settings = SettingsSnapshot.toJSON(message.settings);
    }
    if (message.ui !== undefined) {
      obj.ui = UiSnapshot.toJSON(message.ui);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<GameOutputMessage>, I>>(base?: I): GameOutputMessage {
    return GameOutputMessage.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<GameOutputMessage>, I>>(object: I): GameOutputMessage {
    const message = createBaseGameOutputMessage();
    message.renderer = (object.renderer !== undefined && object.renderer !== null)
      ? RendererSnapshot.fromPartial(object.renderer)
      : undefined;
    message.settings = (object.settings !== undefined && object.settings !== null)
      ? SettingsSnapshot.fromPartial(object.settings)
      : undefined;
    message.ui = (object.ui !== undefined && object.ui !== null) ? UiSnapshot.fromPartial(object.ui) : undefined;
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
