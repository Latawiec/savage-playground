const path = require('path');
const build_protos_from_dir = require("proto_gen");

var args = process.argv.slice(2);

const PROTO_DIR = path.join(process.env.PWD, args[0]);
const OUTPUT_DIR = path.join(process.env.PWD, args[1]);

build_protos_from_dir(PROTO_DIR, OUTPUT_DIR);