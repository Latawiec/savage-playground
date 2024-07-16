import * as path from 'path';
import { build_protos_from_dir } from "proto_gen";

var args = process.argv.slice(2);

const PROTO_DIR = path.join('.', args[0]);
const OUTPUT_DIR = path.join('.', args[1]);

build_protos_from_dir(PROTO_DIR, OUTPUT_DIR);