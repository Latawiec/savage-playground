/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import { Any } from "./google/protobuf/any";

export const protobufPackage = "instance_output";

export interface InstanceOutput {
  instanceOutputMsg: Any | undefined;
}

function createBaseInstanceOutput(): InstanceOutput {
  return { instanceOutputMsg: undefined };
}

export const InstanceOutput = {
  encode(message: InstanceOutput, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.instanceOutputMsg !== undefined) {
      Any.encode(message.instanceOutputMsg, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): InstanceOutput {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseInstanceOutput();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.instanceOutputMsg = Any.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): InstanceOutput {
    return { instanceOutputMsg: isSet(object.instanceOutputMsg) ? Any.fromJSON(object.instanceOutputMsg) : undefined };
  },

  toJSON(message: InstanceOutput): unknown {
    const obj: any = {};
    if (message.instanceOutputMsg !== undefined) {
      obj.instanceOutputMsg = Any.toJSON(message.instanceOutputMsg);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<InstanceOutput>, I>>(base?: I): InstanceOutput {
    return InstanceOutput.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<InstanceOutput>, I>>(object: I): InstanceOutput {
    const message = createBaseInstanceOutput();
    message.instanceOutputMsg = (object.instanceOutputMsg !== undefined && object.instanceOutputMsg !== null)
      ? Any.fromPartial(object.instanceOutputMsg)
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
