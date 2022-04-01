use {
    crate::rpc_subscriptions::RpcSubscriptions,
    crossbeam_channel::RecvTimeoutError,
    solana_client::rpc_response::SlotUpdate,
    solana_ledger::blockstore::CompletedSlotsReceiver,
    solana_sdk::timing::timestamp,
    std::{
        sync::Arc,
        thread::{Builder, JoinHandle},
    },
};

pub struct RpcCompletedSlotsService;
impl RpcCompletedSlotsService {
    pub fn spawn(
        completed_slots_receiver: CompletedSlotsReceiver,
        rpc_subscriptions: Arc<RpcSubscriptions>,
    ) -> JoinHandle<()> {
        Builder::new()
            .name("solana-rpc-completed-slots-service".to_string())
<<<<<<< HEAD
            .spawn(move || {
                for slots in completed_slots_receiver.iter() {
                    for slot in slots {
                        rpc_subscriptions.notify_slot_update(SlotUpdate::Completed {
                            slot,
                            timestamp: timestamp(),
                        });
=======
            .spawn(move || loop {
                // received exit signal, shutdown the service
                if exit.load(Ordering::Relaxed) {
                    break;
                }

                match completed_slots_receiver
                    .recv_timeout(Duration::from_millis(COMPLETE_SLOT_REPORT_SLEEP_MS))
                {
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => {
                        info!("RpcCompletedSlotService channel disconnected, exiting.");
                        break;
                    }
                    Ok(slots) => {
                        for slot in slots {
                            rpc_subscriptions.notify_slot_update(SlotUpdate::Completed {
                                slot,
                                timestamp: timestamp(),
                            });
                        }
>>>>>>> c9a476e24 (handle channel disconnect (#24036))
                    }
                }
            })
            .unwrap()
    }
}
