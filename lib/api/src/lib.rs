pub const MESSAGES_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("messages_descriptor");

tonic::include_proto!("flux.messages");
