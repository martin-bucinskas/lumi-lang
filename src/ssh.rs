use async_trait::async_trait;
use russh::server::{Msg, Session};
use russh::{server, Channel, ChannelId, CryptoVec};
use russh_keys::key::PublicKey;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use log::{info, trace};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Server {
  pub clients: Arc<Mutex<HashMap<(usize, ChannelId), server::Handle>>>,
  pub id: usize,
  buffers: HashMap<ChannelId, Vec<u8>>,
}

impl Server {
  pub fn new() -> Self {
    Server {
      clients: Arc::new(Mutex::new(HashMap::new())),
      id: 0,
      buffers: HashMap::new(),
    }
  }

  async fn post(&mut self, data: CryptoVec) {
    info!("POST");
    let log_message = format!("Got data: {}", String::from_utf8_lossy(&data));
    let mut clients = self.clients.lock().await;
    for ((id, channel), ref mut s) in clients.iter_mut() {
      if *id != self.id {
        let _ = s.data(*channel, data.clone()).await;
      }
    }
  }

  async fn handle_buffer(&mut self, channel_id: ChannelId, session: &mut Session) -> Result<(), anyhow::Error> {
    if let Some(buffer) = self.buffers.remove(&channel_id) {
      let data = CryptoVec::from(buffer);
      info!("Handling buffer: {:?}", String::from_utf8_lossy(&data));
      // session.data(channel_id, data);
      let message = "\n\r>>> ".to_string();
      session.data(channel_id, CryptoVec::from(message));

      // todo: handle commands, etc here.
    }

    Ok(())
  }
}

impl server::Server for Server {
  type Handler = Self;

  fn new_client(&mut self, peer_addr: Option<SocketAddr>) -> Self::Handler {
    info!("New client: {:?}", peer_addr);
    let server = self.clone();
    self.id += 1;
    server
  }
}

#[async_trait]
impl server::Handler for Server {
  type Error = anyhow::Error;

  async fn auth_publickey(&mut self, _: &str, _: &PublicKey) -> Result<server::Auth, Self::Error> {
    info!("Auth_publickey");
    Ok(server::Auth::Accept)
  }

  async fn channel_close(&mut self, channel: ChannelId, _session: &mut Session) -> Result<(), Self::Error> {
    info!("Channel_close");
    let mut clients = self.clients.lock().await;
    clients.remove(&(self.id, channel));
    Ok(())
  }

  async fn channel_open_session(
    &mut self,
    channel: Channel<Msg>,
    session: &mut Session,
  ) -> Result<bool, Self::Error> {
    {
      info!("Channel_open_session");
      let mut clients = self.clients.lock().await;
      clients.insert((self.id, channel.id()), session.handle());
      session.data(channel.id(), CryptoVec::from(">>> ".to_string()));
    }
    Ok(true)
  }

  async fn data(
    &mut self,
    channel: ChannelId,
    data: &[u8],
    session: &mut Session,
  ) -> Result<(), Self::Error> {
    let ctrl_c_pressed = data.iter().any(|&b| b == 3);
    if ctrl_c_pressed {
      trace!("Ctrl-C pressed, closing channel {}", channel);
      session.close(channel);
      return Ok(());
    }

    let backspace_pressed = data.iter().any(|&b| b == 8);
    if backspace_pressed {
      if let Some(buffer) = self.buffers.get_mut(&channel) {
        if !buffer.is_empty() {
          buffer.pop();
        }
      }
    } else {
      // add data to the buffer
      self.buffers
        .entry(channel)
        .or_insert(Vec::new())
        .extend(data);
    }

    if let Some(buffer) = self.buffers.get(&channel) {
      session.data(channel, CryptoVec::from(data.to_vec()));
      if let Some(&last_byte) = buffer.last() {
        if last_byte == 13 {
          self.buffers.get_mut(&channel).unwrap().pop();
          self.handle_buffer(channel, session).await?;
        }
      }
    }

    Ok(())
  }

  async fn tcpip_forward(
    &mut self,
    address: &str,
    port: &mut u32,
    session: &mut Session,
  ) -> Result<bool, Self::Error> {
    info!("TCPIP_FORWARD");
    let handle = session.handle();
    let address = address.to_string();
    let port = *port;
    tokio::spawn(async move {
      let channel = handle
        .channel_open_forwarded_tcpip(address, port, "1.2.3.4", 1234)
        .await
        .unwrap();
      let _ = channel.data(&b"Hello from a forwarded port"[..]).await;
      let _ = channel.eof().await;
    });
    Ok(true)
  }

  async fn shell_request(&mut self, channel: ChannelId, session: &mut Session) -> Result<(), Self::Error> {
    // TODO: print some statistics and metrics
    //   print last login time, number of users, etc
    let banner_message = "Lumi - v0.1.0\n\r\n\r>>> ".to_string();
    session.data(channel, CryptoVec::from(banner_message));
    info!("Shell session started for channel: {}", channel);
    Ok(())
  }
}
