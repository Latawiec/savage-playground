/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { Any } from "./google/protobuf/any";

export const protobufPackage = "instance_input";

export interface InstanceInput {
  instanceInputMsg: Any | undefined;
}

function createBaseInstanceInput(): InstanceInput {
  return { instanceInputMsg: undefined };
}

export const InstanceInput = {
  encode(message: InstanceInput, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.instanceInputMsg !== undefined) {
      Any.encode(message.instanceInputMsg, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): InstanceInput {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseInstanceInput();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.instanceInputMsg = Any.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): InstanceInput {
    return { instanceInputMsg: isSet(object.instanceInputMsg) ? Any.fromJSON(object.instanceInputMsg) : undefined };
  },

  toJSON(message: InstanceInput): unknown {
    const obj: any = {};
    if (message.instanceInputMsg !== undefined) {
      obj.instanceInputMsg = Any.toJSON(message.instanceInputMsg);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<InstanceInput>, I>>(base?: I): InstanceInput {
    return InstanceInput.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<InstanceInput>, I>>(object: I): InstanceInput {
    const message = createBaseInstanceInput();
    message.instanceInputMsg = (object.instanceInputMsg !== undefined && object.instanceInputMsg !== null)
      ? Any.fromPartial(object.instanceInputMsg)
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
