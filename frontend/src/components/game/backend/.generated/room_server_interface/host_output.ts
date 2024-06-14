/* eslint-disable */
import * as _m0 from "protobufjs/minimal";
import Long = require("long");

export const protobufPackage = "host_output";

export interface HostOutput {
  somethingNoIdeaWhatYet?: number | undefined;
}

function createBaseHostOutput(): HostOutput {
  return { somethingNoIdeaWhatYet: undefined };
}

export const HostOutput = {
  encode(message: HostOutput, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.somethingNoIdeaWhatYet !== undefined) {
      writer.uint32(8).uint64(message.somethingNoIdeaWhatYet);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): HostOutput {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseHostOutput();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.somethingNoIdeaWhatYet = longToNumber(reader.uint64() as Long);
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): HostOutput {
    return {
      somethingNoIdeaWhatYet: isSet(object.somethingNoIdeaWhatYet)
        ? globalThis.Number(object.somethingNoIdeaWhatYet)
        : undefined,
    };
  },

  toJSON(message: HostOutput): unknown {
    const obj: any = {};
    if (message.somethingNoIdeaWhatYet !== undefined) {
      obj.somethingNoIdeaWhatYet = Math.round(message.somethingNoIdeaWhatYet);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<HostOutput>, I>>(base?: I): HostOutput {
    return HostOutput.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<HostOutput>, I>>(object: I): HostOutput {
    const message = createBaseHostOutput();
    message.somethingNoIdeaWhatYet = object.somethingNoIdeaWhatYet ?? undefined;
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

function longToNumber(long: Long): number {
  if (long.gt(globalThis.Number.MAX_SAFE_INTEGER)) {
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  }
  return long.toNumber();
}

if (_m0.util.Long !== Long) {
  _m0.util.Long = Long as any;
  _m0.configure();
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
