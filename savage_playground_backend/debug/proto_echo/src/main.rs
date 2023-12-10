use std::io::{Read, Write, self};

use prost::Message;
use room_server_interface::proto::{instance_input, instance_output};

fn main() {

    loop {
        let mut message_len_buff: [u8; std::mem::size_of::<u64>()] = [0, 0, 0, 0, 0, 0, 0, 0];
        io::stdin().read_exact(&mut message_len_buff).unwrap();

        let input_data_len = u64::from_be_bytes(message_len_buff);
        let mut input_message_buff = Vec::<u8>::new();
        input_message_buff.resize(input_data_len as usize, 0);
        io::stdin().read_exact(&mut input_message_buff).unwrap();

        let instance_input_proto = instance_input::InstanceInput::decode(input_message_buff.as_slice()).unwrap();
        let instance_output_proto = instance_output::InstanceOutput {
            instance_output_msg: instance_input_proto.instance_input_msg,
        };
        let mut output_message_buff = Vec::<u8>::new();
        instance_output_proto.encode(&mut output_message_buff).unwrap();
        let output_data_len = output_message_buff.len();

        io::stdout().write(&output_data_len.to_be_bytes()).unwrap();
        io::stdout().write(&output_message_buff).unwrap();
    }
}
