use iroha_data_model::prelude::{HasOrigin, Identifiable};
use iroha_data_model_derive::{Filter, IdEqOrdHash};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Filter, Deserialize, Serialize)]
pub enum LayerEvent {
    SubLayer(SubLayerEvent),
    Created(LayerId),
}

pub enum SubLayerEvent {
    Created(SubLayerId),
}

pub struct LayerId {
    name: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubLayerId {
    name: u32,
    parent_id: LayerId,
}

#[derive(Debug, Clone, IdEqOrdHash)]
pub struct Layer {
    id: <Self as Identifiable>::Id,
}

#[derive(Debug, Clone, IdEqOrdHash)]
pub struct SubLayer {
    id: <Self as Identifiable>::Id,
}

impl HasOrigin for LayerEvent {
    type Origin = Layer;

    fn origin_id(&self) -> &<Layer as Identifiable>::Id {
        match self {
            Self::SubLayer(sub_layer) => &sub_layer.origin_id().parent_id,
            Self::Created(id) => id,
        }
    }
}

impl HasOrigin for SubLayerEvent {
    type Origin = SubLayer;

    fn origin_id(&self) -> &<SubLayer as Identifiable>::Id {
        match self {
            Self::Created(id) => id,
        }
    }
}
