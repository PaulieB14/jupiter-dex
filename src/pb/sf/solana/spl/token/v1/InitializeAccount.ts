// Code generated by protoc-gen-as. DO NOT EDIT.
// Versions:
//   protoc-gen-as v1.3.3

import { Writer, Reader } from "as-proto/assembly";
import { InitializeAccountInstruction } from "./InitializeAccount/InitializeAccountInstruction";
import { InitializeAccountAccounts } from "./InitializeAccount/InitializeAccountAccounts";
import { InitializeAccountVersion } from "./InitializeAccount/InitializeAccountVersion";

export class InitializeAccount {
  static encode(message: InitializeAccount, writer: Writer): void {
    writer.uint32(8);
    writer.int32(message.version);

    const instruction = message.instruction;
    if (instruction !== null) {
      writer.uint32(18);
      writer.fork();
      InitializeAccountInstruction.encode(instruction, writer);
      writer.ldelim();
    }

    const accounts = message.accounts;
    if (accounts !== null) {
      writer.uint32(26);
      writer.fork();
      InitializeAccountAccounts.encode(accounts, writer);
      writer.ldelim();
    }
  }

  static decode(reader: Reader, length: i32): InitializeAccount {
    const end: usize = length < 0 ? reader.end : reader.ptr + length;
    const message = new InitializeAccount();

    while (reader.ptr < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.version = reader.int32();
          break;

        case 2:
          message.instruction = InitializeAccountInstruction.decode(
            reader,
            reader.uint32()
          );
          break;

        case 3:
          message.accounts = InitializeAccountAccounts.decode(
            reader,
            reader.uint32()
          );
          break;

        default:
          reader.skipType(tag & 7);
          break;
      }
    }

    return message;
  }

  version: InitializeAccountVersion;
  instruction: InitializeAccountInstruction | null;
  accounts: InitializeAccountAccounts | null;

  constructor(
    version: InitializeAccountVersion = 0,
    instruction: InitializeAccountInstruction | null = null,
    accounts: InitializeAccountAccounts | null = null
  ) {
    this.version = version;
    this.instruction = instruction;
    this.accounts = accounts;
  }
}
