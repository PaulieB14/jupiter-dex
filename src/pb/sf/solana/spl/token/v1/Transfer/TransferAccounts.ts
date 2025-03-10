// Code generated by protoc-gen-as. DO NOT EDIT.
// Versions:
//   protoc-gen-as v1.3.3

import { Writer, Reader } from "as-proto/assembly";
import { Signer } from "../Signer";

export class TransferAccounts {
  static encode(message: TransferAccounts, writer: Writer): void {
    writer.uint32(10);
    writer.string(message.source);

    writer.uint32(18);
    writer.string(message.tokenMint);

    writer.uint32(26);
    writer.string(message.destination);

    const signer = message.signer;
    if (signer !== null) {
      writer.uint32(34);
      writer.fork();
      Signer.encode(signer, writer);
      writer.ldelim();
    }
  }

  static decode(reader: Reader, length: i32): TransferAccounts {
    const end: usize = length < 0 ? reader.end : reader.ptr + length;
    const message = new TransferAccounts();

    while (reader.ptr < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.source = reader.string();
          break;

        case 2:
          message.tokenMint = reader.string();
          break;

        case 3:
          message.destination = reader.string();
          break;

        case 4:
          message.signer = Signer.decode(reader, reader.uint32());
          break;

        default:
          reader.skipType(tag & 7);
          break;
      }
    }

    return message;
  }

  source: string;
  tokenMint: string;
  destination: string;
  signer: Signer | null;

  constructor(
    source: string = "",
    tokenMint: string = "",
    destination: string = "",
    signer: Signer | null = null
  ) {
    this.source = source;
    this.tokenMint = tokenMint;
    this.destination = destination;
    this.signer = signer;
  }
}
