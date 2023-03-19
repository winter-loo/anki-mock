use crate::pb::cards::cards_service::Service as CardsService;
use crate::pb::backend::ServiceIndex as ServiceIndex;
use crate::pb::*;
use crate::pb;
use prost::Message;
use crate::error::OrInvalid;

pub struct Backend {
    server: bool
}

pub fn init_backend(init_msg: &[u8]) -> std::result::Result<Backend, String> {
    let input: pb::backend::BackendInit = match pb::backend::BackendInit::decode(init_msg) {
        Ok(req) => req,
        Err(_) => return Err("couldn't decode init request".into()),
    };

    Ok(Backend::new(input.server))
}

impl Backend {
    fn new(server: bool) -> Self {
        Backend { server: server }
    }

    pub fn run_method(
        &self,
        service: u32,
        method: u32,
        input: &[u8],
    ) -> std::result::Result<Vec<u8>, Vec<u8>> {
        ServiceIndex::from_i32(service as i32)
            .or_invalid("invalid service")
            .and_then(|service| match service {
                ServiceIndex::Cards => CardsService::run_method(self, method, input),
            })
            .map_err(|err| {
                let backend_err = err.into_protobuf();
                let mut bytes = Vec::new();
                backend_err.encode(&mut bytes).unwrap();
                bytes
            })
    }
}

impl CardsService for Backend {
    fn get_card(&self, input: cards::CardId) -> crate::error::Result<cards::Card> {
        Ok(cards::Card { id: 1, note_id: 1, deck_id: 1 })
    }

    fn remove_cards(
        &self,
        input: cards::RemoveCardsRequest,
    ) -> crate::error::Result<cards::CardServiceResponse> {
        todo!()
    }

    fn set_deck(
        &self,
        input: cards::SetDeckRequest,
    ) -> crate::error::Result<cards::CardServiceResponse> {
        todo!()
    }
}
