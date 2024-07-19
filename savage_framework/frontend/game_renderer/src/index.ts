
import path from "path";
import { MemoryAssetStorage } from "./asset_storage/MemoryAssetStorage";

async function main() {
    const asset_storage = new MemoryAssetStorage();
    await asset_storage.append_from_zip_local(path.resolve("./test.zip"));
    const file = await asset_storage.readFile('a/a.txt');
    console.log(`${file}`);
}

main()
