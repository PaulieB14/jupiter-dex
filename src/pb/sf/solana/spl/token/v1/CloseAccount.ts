// Code generated by protoc-gen-as. DO NOT EDIT.
// Versions:
//   protoc-gen-as v1.3.3

import { Writer, Reader } from "as-proto/assembly";
import { CloseAccountInstruction } from "./CloseAccount/CloseAccountInstruction";
import { CloseAccountAccounts } from "./CloseAccount/CloseAccountAccounts";

export class CloseAccount {
  static encode(message: CloseAccount, writer: Writer): void {
    const instruction = message.instruction;
    if (instruction !== null) {
      writer.uint32(10);
      writer.fork();
      CloseAccountInstruction.encode(instruction, writer);
      writer.ldelim();
    }

    const accounts = message.accounts;
    if (accounts !== null) {
      writer.uint32(18);
      writer.fork();
      CloseAccountAccounts.encode(accounts, writer);
      writer.ldelim();
    }
  }

  static decode(reader: Reader, length: i32): CloseAccount {
    const end: usize = length < 0 ? reader.end : reader.ptr + length;
    const message = new CloseAccount();

    while (reader.ptr < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.instruction = CloseAccountInstruction.decode(
            reader,
            reader.uint32()
          );
          break;

        case 2:
          message.accounts = CloseAccountAccounts.decode(
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

  instruction: CloseAccountInstruction | null;
  accounts: CloseAccountAccounts | null;

  constructor(
    instruction: CloseAccountInstruction | null = null,
    accounts: CloseAccountAccounts | null = null
  ) {
    this.instruction = instruction;
    this.accounts = accounts;
  }
}
