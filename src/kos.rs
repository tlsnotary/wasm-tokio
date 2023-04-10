use mpc_ot::kos::{receiver::Kos15IOReceiver, sender::Kos15IOSender};
use mpc_ot_core::{r_state, s_state};
use utils_aio::duplex::DuplexChannel;

pub fn init_kos() -> (
    Kos15IOSender<s_state::Initialized>,
    Kos15IOReceiver<r_state::Initialized>,
) {
    let (c1, c2) = DuplexChannel::new();
    let sender = Kos15IOSender::new(Box::new(c1));
    let receiver = Kos15IOReceiver::new(Box::new(c2));
    (sender, receiver)
}
