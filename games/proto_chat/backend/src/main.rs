use std::io::{self, Read, Write};

#[path = ".gen/proto/mod.rs"]
mod proto_chat;

use game_config::game_args::GameArgs;
use game_interface::proto::{game_input::GameInput, game_output::{GameDirectOutput, GameMessage, GameOutput}};
use prost::Message;
use prost_types::Any;
use proto_chat::proto_chat::{ProtoChatHistory, ProtoChatMessage};
use tracing::info;


fn main() {
    let game_args = GameArgs::new("ProtoChat - communicator that exchanges protobuf messages across stdin/stdout pipes.");
    if let Err(err) = game_args.process() {
        eprintln!("Error: {:?}", err);
        return;
    }

    let mut proto_chat_history = ProtoChatHistory::default();
    
    info!("Let's go!");
    
    loop {
        
        let mut message_len_buff: [u8; std::mem::size_of::<u64>()] = [0, 0, 0, 0, 0, 0, 0, 0];
        if let Err(error) = io::stdin().read_exact(&mut message_len_buff) {
            info!("Couldn't read bytesize: {}", error);
        }

        let input_data_len = u64::from_be_bytes(message_len_buff);
        let mut input_message_buff = Vec::<u8>::new();
        input_message_buff.resize(input_data_len as usize, 0);
        if let Err(error) = io::stdin().read_exact(&mut input_message_buff) {
            info!("Couldn't read message: {}", error);
        }

        info!("Read {} bytes of data.", input_message_buff.len());

        let game_input_proto = GameInput::decode(input_message_buff.as_slice()).unwrap();
        let client_input_proto = game_input_proto.client_input.unwrap();
        let client_id = client_input_proto.sender_id;
        let client_msg = client_input_proto.game_input_message.unwrap();
        let client_msg_type = client_msg.type_url.clone();

        let mut client_output_proto = GameOutput::default();
        match client_msg_type.as_str() {
            "proto_chat.message" => {
                let decoded_res = ProtoChatMessage::decode(client_msg.value.as_slice());
                if let Err(err) = decoded_res {
                    info!("Error: {:?}", err);
                    continue;
                }
                let mut decoded = decoded_res.unwrap();
                decoded.user_id = Some(client_id);
                let as_any = Any {
                    type_url: client_msg_type,
                    value: decoded.encode_to_vec()
                };
                proto_chat_history.history.push(decoded);

                let client_output = GameMessage {
                    message: Some(as_any)
                };
                client_output_proto.broadcast = Some(client_output);
            }
            "proto_chat.request" => {
                if let Ok(proto_chat_request) = proto_chat::proto_chat::ProtoChatRequest::decode(client_msg.value.as_slice()) {
                    for request in &proto_chat_request.requests {
                        match proto_chat::proto_chat::ProtoChatRequestType::try_from(*request).unwrap() {
                            proto_chat::proto_chat::ProtoChatRequestType::History => {
                                let encoded_history = Any {
                                    type_url: "proto_chat.history".to_owned(),
                                    value: proto_chat_history.clone().encode_to_vec()
                                };
                                let client_output = GameMessage {
                                    message: Some(encoded_history)
                                };
                                let direct_message = GameDirectOutput {
                                    receiver_id: client_id,
                                    game_output: Some(client_output)
                                };
                                client_output_proto.direct_messages.push(direct_message);
                            }
                        }
                    }
                }
            }
            _ => {
                info!("Unmatched: {}", client_msg_type);
            }
        };
        let output_encoded = client_output_proto.encode_to_vec();
        let _ = io::stdout().write_all(&output_encoded.len().to_be_bytes());
        let _ = io::stdout().write_all(output_encoded.as_slice());
        let _ = io::stdout().flush();
        info!("Sent {} bytes of data.", output_encoded.len());
    }
}
