use iroha_data_model::prelude::{HasOrigin, Identifiable};
use iroha_data_model_derive::{Filter, IdEqOrdHash};
use serde::{Deserialize, Serialize};
pub enum LayerEvent {
    SubLayer(SubLayerEvent),
    Created(LayerId),
}
#[automatically_derived]
impl ::core::fmt::Debug for LayerEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            LayerEvent::SubLayer(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "SubLayer",
                    &__self_0,
                )
            }
            LayerEvent::Created(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Created",
                    &__self_0,
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for LayerEvent {
    #[inline]
    fn clone(&self) -> LayerEvent {
        match self {
            LayerEvent::SubLayer(__self_0) => {
                LayerEvent::SubLayer(::core::clone::Clone::clone(__self_0))
            }
            LayerEvent::Created(__self_0) => {
                LayerEvent::Created(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for LayerEvent {}
#[automatically_derived]
impl ::core::cmp::PartialEq for LayerEvent {
    #[inline]
    fn eq(&self, other: &LayerEvent) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (LayerEvent::SubLayer(__self_0), LayerEvent::SubLayer(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                (LayerEvent::Created(__self_0), LayerEvent::Created(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
                _ => unsafe { ::core::intrinsics::unreachable() }
            }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for LayerEvent {}
#[automatically_derived]
impl ::core::cmp::Eq for LayerEvent {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<SubLayerEvent>;
        let _: ::core::cmp::AssertParamIsEq<LayerId>;
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for LayerEvent {
    #[inline]
    fn partial_cmp(
        &self,
        other: &LayerEvent,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                match (self, other) {
                    (LayerEvent::SubLayer(__self_0), LayerEvent::SubLayer(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    (LayerEvent::Created(__self_0), LayerEvent::Created(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for LayerEvent {
    #[inline]
    fn cmp(&self, other: &LayerEvent) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
            ::core::cmp::Ordering::Equal => {
                match (self, other) {
                    (LayerEvent::SubLayer(__self_0), LayerEvent::SubLayer(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (LayerEvent::Created(__self_0), LayerEvent::Created(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::hash::Hash for LayerEvent {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state);
        match self {
            LayerEvent::SubLayer(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            LayerEvent::Created(__self_0) => ::core::hash::Hash::hash(__self_0, state),
        }
    }
}
/// Filter for LayerEvent entity
pub struct LayerFilter {
    origin_filter: crate::prelude::FilterOpt<
        crate::prelude::OriginFilter<crate::prelude::LayerEvent>,
    >,
    event_filter: crate::prelude::FilterOpt<LayerEventFilter>,
}
#[automatically_derived]
impl ::core::fmt::Debug for LayerFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "LayerFilter",
            "origin_filter",
            &&self.origin_filter,
            "event_filter",
            &&self.event_filter,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for LayerFilter {
    #[inline]
    fn clone(&self) -> LayerFilter {
        LayerFilter {
            origin_filter: ::core::clone::Clone::clone(&self.origin_filter),
            event_filter: ::core::clone::Clone::clone(&self.event_filter),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for LayerFilter {}
#[automatically_derived]
impl ::core::cmp::PartialEq for LayerFilter {
    #[inline]
    fn eq(&self, other: &LayerFilter) -> bool {
        self.origin_filter == other.origin_filter
            && self.event_filter == other.event_filter
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for LayerFilter {}
#[automatically_derived]
impl ::core::cmp::Eq for LayerFilter {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<
            crate::prelude::FilterOpt<
                crate::prelude::OriginFilter<crate::prelude::LayerEvent>,
            >,
        >;
        let _: ::core::cmp::AssertParamIsEq<crate::prelude::FilterOpt<LayerEventFilter>>;
    }
}
#[allow(clippy::enum_variant_names, missing_docs)]
/// Event filter for LayerEvent entity
pub enum LayerEventFilter {
    ByCreated,
    BySubLayer(crate::prelude::FilterOpt<SubLayerFilter>),
}
#[automatically_derived]
#[allow(clippy::enum_variant_names, missing_docs)]
impl ::core::fmt::Debug for LayerEventFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            LayerEventFilter::ByCreated => {
                ::core::fmt::Formatter::write_str(f, "ByCreated")
            }
            LayerEventFilter::BySubLayer(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "BySubLayer",
                    &__self_0,
                )
            }
        }
    }
}
#[automatically_derived]
#[allow(clippy::enum_variant_names, missing_docs)]
impl ::core::clone::Clone for LayerEventFilter {
    #[inline]
    fn clone(&self) -> LayerEventFilter {
        match self {
            LayerEventFilter::ByCreated => LayerEventFilter::ByCreated,
            LayerEventFilter::BySubLayer(__self_0) => {
                LayerEventFilter::BySubLayer(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[allow(clippy::enum_variant_names, missing_docs)]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for LayerEventFilter {}
#[automatically_derived]
#[allow(clippy::enum_variant_names, missing_docs)]
impl ::core::cmp::PartialEq for LayerEventFilter {
    #[inline]
    fn eq(&self, other: &LayerEventFilter) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
            && match (self, other) {
                (
                    LayerEventFilter::BySubLayer(__self_0),
                    LayerEventFilter::BySubLayer(__arg1_0),
                ) => *__self_0 == *__arg1_0,
                _ => true,
            }
    }
}
#[allow(clippy::enum_variant_names, missing_docs)]
#[automatically_derived]
impl ::core::marker::StructuralEq for LayerEventFilter {}
#[automatically_derived]
#[allow(clippy::enum_variant_names, missing_docs)]
impl ::core::cmp::Eq for LayerEventFilter {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<crate::prelude::FilterOpt<SubLayerFilter>>;
    }
}
pub enum SubLayerEvent {
    Created(SubLayerId),
}
pub struct LayerId {
    name: u32,
}
pub struct SubLayerId {
    name: u32,
    parent_id: LayerId,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubLayerId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "SubLayerId",
            "name",
            &&self.name,
            "parent_id",
            &&self.parent_id,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for SubLayerId {
    #[inline]
    fn clone(&self) -> SubLayerId {
        SubLayerId {
            name: ::core::clone::Clone::clone(&self.name),
            parent_id: ::core::clone::Clone::clone(&self.parent_id),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SubLayerId {}
#[automatically_derived]
impl ::core::cmp::PartialEq for SubLayerId {
    #[inline]
    fn eq(&self, other: &SubLayerId) -> bool {
        self.name == other.name && self.parent_id == other.parent_id
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for SubLayerId {}
#[automatically_derived]
impl ::core::cmp::Eq for SubLayerId {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
        let _: ::core::cmp::AssertParamIsEq<LayerId>;
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for SubLayerId {
    #[inline]
    fn partial_cmp(
        &self,
        other: &SubLayerId,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.name, &other.name) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                ::core::cmp::PartialOrd::partial_cmp(&self.parent_id, &other.parent_id)
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for SubLayerId {
    #[inline]
    fn cmp(&self, other: &SubLayerId) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.name, &other.name) {
            ::core::cmp::Ordering::Equal => {
                ::core::cmp::Ord::cmp(&self.parent_id, &other.parent_id)
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::hash::Hash for SubLayerId {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.name, state);
        ::core::hash::Hash::hash(&self.parent_id, state)
    }
}
pub struct Layer {
    id: <Self as Identifiable>::Id,
}
#[automatically_derived]
impl ::core::fmt::Debug for Layer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "Layer", "id", &&self.id)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Layer {
    #[inline]
    fn clone(&self) -> Layer {
        Layer {
            id: ::core::clone::Clone::clone(&self.id),
        }
    }
}
impl Identifiable for Layer {
    type Id = <Self as Identifiable>::Id;
    #[inline]
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
impl ::core::cmp::PartialOrd for Layer
where
    Self: Identifiable,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl ::core::cmp::Ord for Layer
where
    Self: Identifiable,
{
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        self.id().cmp(other.id())
    }
}
impl ::core::cmp::Eq for Layer
where
    Self: Identifiable,
{}
impl ::core::cmp::PartialEq for Layer
where
    Self: Identifiable,
{
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
impl ::core::hash::Hash for Layer
where
    Self: Identifiable,
{
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
pub struct SubLayer {
    id: <Self as Identifiable>::Id,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubLayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "SubLayer",
            "id",
            &&self.id,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for SubLayer {
    #[inline]
    fn clone(&self) -> SubLayer {
        SubLayer {
            id: ::core::clone::Clone::clone(&self.id),
        }
    }
}
impl Identifiable for SubLayer {
    type Id = <Self as Identifiable>::Id;
    #[inline]
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
impl ::core::cmp::PartialOrd for SubLayer
where
    Self: Identifiable,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl ::core::cmp::Ord for SubLayer
where
    Self: Identifiable,
{
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        self.id().cmp(other.id())
    }
}
impl ::core::cmp::Eq for SubLayer
where
    Self: Identifiable,
{}
impl ::core::cmp::PartialEq for SubLayer
where
    Self: Identifiable,
{
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
impl ::core::hash::Hash for SubLayer
where
    Self: Identifiable,
{
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
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
