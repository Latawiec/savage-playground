import * as path from 'path';
import { fileURLToPath } from 'url';
import { build_protos_from_dir } from "proto_gen";

// Get the directory name of the current module
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

var args = process.argv.slice(2);

const PROTO_DIR = path.resolve(__dirname, '../..', args[0]);
const OUTPUT_DIR = path.resolve(__dirname, '../..', args[1]);

build_protos_from_dir(PROTO_DIR, OUTPUT_DIR);