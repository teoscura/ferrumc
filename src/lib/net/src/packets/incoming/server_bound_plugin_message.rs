use std::io::Read;
use std::sync::Arc;
use tracing::debug;
use ferrumc_macros::{packet};
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use crate::packets::IncomingPacket;
use crate::{NetResult, ServerState};

#[derive(Debug)]
#[packet(packet_id = 0x02, state = "configuration")]
pub struct ServerBoundPluginMessage {
    channel: String,
    data: Vec<u8>
}

pub struct ClientMinecraftBrand {
    pub brand: String
}


impl NetDecode for ServerBoundPluginMessage {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let channel = <String>::decode(reader, opts)?;
        let mut buf = Vec::<u8>::new();
        reader.read_to_end(&mut buf)?;

        Ok(Self {
            channel,
            data: buf
        })
    }
}

impl IncomingPacket for ServerBoundPluginMessage {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        debug!("Received plugin message: {:?}", self);

        if self.channel == "minecraft:brand" {
            let brand = String::from_utf8(self.data)?;
            debug!("Client brand: {}", brand);
            
            state.universe.add_component(conn_id, ClientMinecraftBrand { brand });
        }

        Ok(())
    }
}