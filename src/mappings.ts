import { BigInt, Bytes, Entity, store, TypedMap, JSONValue } from "@graphprotocol/graph-ts";

export function handleTriggers(data: TypedMap<string, JSONValue>): void {
  const changes = data.get("changes");
  if (!changes) return;

  const changesArray = changes.toArray();
  for (let i = 0; i < changesArray.length; i++) {
    const change = changesArray[i].toObject();
    const entityType = change.get("entity_type");
    if (!entityType || entityType.toString() !== "Account") continue;

    const id = change.get("id");
    const operation = change.get("operation");
    if (!id || !operation) continue;

    if (operation.toBigInt().toI32() === 3) { // DELETE
      store.remove("Account", id.toString());
      continue;
    }

    const fields = change.get("fields");
    if (!fields) continue;

    const entity = new Entity();
    const fieldsArray = fields.toArray();
    
    for (let j = 0; j < fieldsArray.length; j++) {
      const field = fieldsArray[j].toObject();
      const name = field.get("name");
      const value = field.get("value");
      if (!name || !value) continue;

      const typedValue = value.toObject().get("typed_value");
      if (!typedValue) continue;

      switch (name.toString()) {
        case "slot":
        case "lamports":
        case "rentEpoch":
          const int64Value = typedValue.toObject().get("int64_value");
          if (int64Value) {
            entity.setBigInt(name.toString(), int64Value.toBigInt());
          }
          break;
        case "pubkey":
        case "owner":
          const stringValue = typedValue.toObject().get("string_value");
          if (stringValue) {
            entity.setString(name.toString(), stringValue.toString());
          }
          break;
        case "executable":
          const boolValue = typedValue.toObject().get("bool_value");
          if (boolValue) {
            entity.setBoolean(name.toString(), boolValue.toBool());
          }
          break;
        case "data":
          const bytesValue = typedValue.toObject().get("bytes_value");
          if (bytesValue && bytesValue.toString() !== "") {
            entity.setBytes(name.toString(), Bytes.fromByteArray(Bytes.fromHexString(bytesValue.toString())));
          }
          break;
      }
    }
    
    store.set("Account", id.toString(), entity);
  }
}
