use std::collections::BTreeMap;

use itertools::Itertools;
use nohash_hasher::IntMap;
use smallvec::SmallVec;

use re_chunk_store::LatestAtQuery;
use re_entity_db::{EntityPath, TimeInt, Timeline};
use re_log_types::StoreKind;
use re_types::ComponentName;

use crate::{
    DataResultTree, QueryRange, SpaceViewHighlights, SpaceViewId, ViewContext,
    ViewSystemIdentifier, ViewerContext,
};

/// Path to a specific entity in a specific store used for overrides.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OverridePath {
    // NOTE: StoreKind is easier to work with than a `StoreId`` or full `ChunkStore` but
    // might still be ambiguous when we have multiple stores active at a time.
    pub store_kind: StoreKind,
    pub path: EntityPath,
}

impl OverridePath {
    pub fn blueprint_path(path: EntityPath) -> Self {
        Self {
            store_kind: StoreKind::Blueprint,
            path,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropertyOverrides {
    /// An alternative store and entity path to use for the specified component.
    ///
    /// These are resolved overrides, i.e. the result of recursive override propagation + individual overrides.
    // TODO(jleibs): Consider something like `tinymap` for this.
    // TODO(andreas): Should be a `Cow` to not do as many clones.
    // TODO(andreas): Track recursive vs resolved (== individual + recursive) overrides.
    //                  Recursive here meaning inherited + own recursive, i.e. not just what's on the path.
    //                  What is logged on *this* entity can be inferred from walking up the tree.
    pub resolved_component_overrides: IntMap<ComponentName, OverridePath>,

    /// `EntityPath` in the Blueprint store where updated overrides should be written back
    /// for properties that apply recursively.
    pub recursive_override_path: EntityPath,

    /// `EntityPath` in the Blueprint store where updated overrides should be written back
    /// for properties that apply to the individual entity only.
    pub individual_override_path: EntityPath,

    /// What range is queried on the chunk store.
    pub query_range: QueryRange,
}

pub type SmallVisualizerSet = SmallVec<[ViewSystemIdentifier; 4]>;

/// This is the primary mechanism through which data is passed to a `SpaceView`.
///
/// It contains everything necessary to properly use this data in the context of the
/// `ViewSystem`s that it is a part of.
#[derive(Clone, Debug)]
pub struct DataResult {
    /// Where to retrieve the data from.
    // TODO(jleibs): This should eventually become a more generalized (StoreView + EntityPath) reference to handle
    // multi-RRD or blueprint-static data references.
    pub entity_path: EntityPath,

    /// Which `ViewSystems`s to pass the `DataResult` to.
    pub visualizers: SmallVisualizerSet,

    /// If true, this path is not actually included in the query results and is just here
    /// because of a common prefix.
    ///
    /// If this is true, `visualizers` must be empty.
    pub tree_prefix_only: bool,

    /// The accumulated property overrides for this `DataResult`.
    pub property_overrides: Option<PropertyOverrides>,
}

impl DataResult {
    pub const INDIVIDUAL_OVERRIDES_PREFIX: &'static str = "individual_overrides";
    pub const RECURSIVE_OVERRIDES_PREFIX: &'static str = "recursive_overrides";

    #[inline]
    pub fn recursive_override_path(&self) -> Option<&EntityPath> {
        self.property_overrides
            .as_ref()
            .map(|p| &p.recursive_override_path)
    }

    #[inline]
    pub fn individual_override_path(&self) -> Option<&EntityPath> {
        self.property_overrides
            .as_ref()
            .map(|p| &p.individual_override_path)
    }

    /// Saves a recursive override OR clears both (!) individual & recursive overrides if the override is due to a parent recursive override or a default value.
    // TODO(andreas): Does not take individual overrides into account yet.
    // TODO(andreas): This should have a unit test, but the delayed override write makes it hard to test.
    pub fn save_recursive_override_or_clear_if_redundant<C: re_types::Component + Eq + Default>(
        &self,
        ctx: &ViewerContext<'_>,
        data_result_tree: &DataResultTree,
        desired_override: &C,
    ) {
        re_tracing::profile_function!();

        // TODO(jleibs): Make it impossible for this to happen with different type structure
        // This should never happen unless we're doing something with a partially processed
        // query.
        let (Some(recursive_override_path), Some(individual_override_path)) = (
            self.recursive_override_path(),
            self.individual_override_path(),
        ) else {
            re_log::warn!(
                "Tried to save override for {:?} but it has no override path",
                self.entity_path
            );
            return;
        };

        if let Some(current_resolved_override) = self.lookup_override::<C>(ctx) {
            // Do nothing if the resolved override is already the same as the new override.
            if &current_resolved_override == desired_override {
                return;
            }

            // TODO(andreas): Assumes this is a recursive override
            let parent_recursive_override = self
                .entity_path
                .parent()
                .and_then(|parent_path| data_result_tree.lookup_result_by_path(&parent_path))
                .and_then(|data_result| data_result.lookup_override::<C>(ctx));

            // If the parent has a recursive override that is the same as the new override,
            // clear both individual and recursive override at the current path.
            // (at least one of them has to be set, otherwise the current resolved override would be the same as the desired override)
            //
            // Another case for clearing
            if parent_recursive_override.as_ref() == Some(desired_override)
                || (parent_recursive_override.is_none() && desired_override == &C::default())
            {
                // TODO(andreas): It might be that only either of these two are necessary, in that case we shouldn't clear both.
                ctx.save_empty_blueprint_component::<C>(recursive_override_path);
                ctx.save_empty_blueprint_component::<C>(individual_override_path);
            } else {
                ctx.save_blueprint_component(recursive_override_path, desired_override);
            }
        } else {
            // No override at all so far, simply set it.
            ctx.save_blueprint_component(recursive_override_path, desired_override);
        }
    }

    /// Saves a recursive override, does not take into current or default values.
    ///
    /// Ignores individual overrides and current value.
    pub fn save_recursive_override<C: re_types::Component + Eq>(
        &self,
        ctx: &ViewerContext<'_>,
        desired_override: &C,
    ) {
        re_tracing::profile_function!();

        // TODO(jleibs): Make it impossible for this to happen with different type structure
        // This should never happen unless we're doing something with a partially processed
        // query.
        let Some(recursive_override_path) = self.recursive_override_path() else {
            re_log::warn!(
                "Tried to save override for {:?} but it has no override path",
                self.entity_path
            );
            return;
        };

        ctx.save_blueprint_component(recursive_override_path, desired_override);
    }

    /// Saves a recursive override, does not take into current or default values.
    ///
    /// Ignores individual overrides and current value.
    pub fn save_individual_override<C: re_types::Component>(
        &self,
        ctx: &ViewerContext<'_>,
        desired_override: &C,
    ) {
        re_tracing::profile_function!();

        // TODO(jleibs): Make it impossible for this to happen with different type structure
        // This should never happen unless we're doing something with a partially processed
        // query.
        let Some(override_path) = self.individual_override_path() else {
            re_log::warn!(
                "Tried to save override for {:?} but it has no override path",
                self.entity_path
            );
            return;
        };

        ctx.save_blueprint_component(override_path, desired_override);
    }

    /// Clears the recursive override for a given component
    pub fn clear_recursive_override<C: re_types::Component>(&self, ctx: &ViewerContext<'_>) {
        // TODO(jleibs): Make it impossible for this to happen with different type structure
        // This should never happen unless we're doing something with a partially processed
        // query.
        let Some(recursive_override_path) = self.recursive_override_path() else {
            re_log::warn!(
                "Tried to save override for {:?} but it has no override path",
                self.entity_path
            );
            return;
        };

        ctx.save_empty_blueprint_component::<C>(recursive_override_path);
    }

    /// Clears the recursive override for a given component
    pub fn clear_individual_override<C: re_types::Component>(&self, ctx: &ViewerContext<'_>) {
        // TODO(jleibs): Make it impossible for this to happen with different type structure
        // This should never happen unless we're doing something with a partially processed
        // query.
        let Some(individual_override_path) = self.individual_override_path() else {
            re_log::warn!(
                "Tried to save override for {:?} but it has no override path",
                self.entity_path
            );
            return;
        };

        ctx.save_empty_blueprint_component::<C>(individual_override_path);
    }

    fn lookup_override<C: 'static + re_types::Component>(
        &self,
        ctx: &ViewerContext<'_>,
    ) -> Option<C> {
        self.property_overrides
            .as_ref()
            .and_then(|p| p.resolved_component_overrides.get(&C::name()))
            .and_then(|OverridePath { store_kind, path }| match store_kind {
                // TODO(#5607): what should happen if the promise is still pending?
                StoreKind::Blueprint => ctx
                    .store_context
                    .blueprint
                    .latest_at_component::<C>(path, ctx.blueprint_query),
                StoreKind::Recording => ctx
                    .recording()
                    .latest_at_component::<C>(path, &ctx.current_query()),
            })
            .map(|c| c.value)
    }

    /// Returns from which entity path an override originates from.
    ///
    /// Returns None if there was no override at all.
    /// Note that if this returns the current path, the override might be either an individual or recursive override.
    pub fn component_override_source(
        &self,
        result_tree: &DataResultTree,
        component_name: ComponentName,
    ) -> Option<EntityPath> {
        re_tracing::profile_function!();

        // If we don't have a resolved override, clearly nothing overrode this.
        let active_override = self
            .property_overrides
            .as_ref()
            .and_then(|p| p.resolved_component_overrides.get(&component_name))?;

        // Walk up the tree to find the highest ancestor which has a matching override.
        // This must be the ancestor we inherited the override from. Note that `active_override`
        // is a `(StoreKind, EntityPath)`, not a value.
        let mut override_source = self.entity_path.clone();
        while let Some(parent_path) = override_source.parent() {
            if result_tree
                .lookup_result_by_path(&parent_path)
                .and_then(|data_result| data_result.property_overrides.as_ref())
                .map_or(true, |property_overrides| {
                    // TODO(andreas): Assumes all overrides are recursive which is not true,
                    //                This should access `recursive_component_overrides` instead.
                    property_overrides
                        .resolved_component_overrides
                        .get(&component_name)
                        != Some(active_override)
                })
            {
                break;
            }

            override_source = parent_path;
        }

        Some(override_source)
    }

    /// Returns true if the current component's value was inherited from a parent entity.
    pub fn is_inherited(
        &self,
        result_tree: &DataResultTree,
        component_name: ComponentName,
    ) -> bool {
        let override_source = self.component_override_source(result_tree, component_name);
        override_source.is_some() && override_source.as_ref() != Some(&self.entity_path)
    }

    /// Shorthand for checking for visibility on data overrides.
    ///
    /// Note that this won't check if the chunk store has visibility logged.
    // TODO(#6541): Check the datastore.
    #[inline]
    pub fn is_visible(&self, ctx: &ViewerContext<'_>) -> bool {
        self.lookup_override::<re_types::blueprint::components::Visible>(ctx)
            .unwrap_or_default()
            .0
    }

    /// Shorthand for checking for interactivity on data overrides.
    ///
    /// Note that this won't check if the chunk store has interactivity logged.
    // TODO(#6541): Check the datastore.
    #[inline]
    pub fn is_interactive(&self, ctx: &ViewerContext<'_>) -> bool {
        *self
            .lookup_override::<re_types::blueprint::components::Interactive>(ctx)
            .unwrap_or_default()
            .0
    }

    /// Returns the query range for this data result.
    pub fn query_range(&self) -> &QueryRange {
        const DEFAULT_RANGE: QueryRange = QueryRange::LatestAt;
        self.property_overrides
            .as_ref()
            .map_or(&DEFAULT_RANGE, |p| &p.query_range)
    }
}

pub type PerSystemDataResults<'a> = BTreeMap<ViewSystemIdentifier, Vec<&'a DataResult>>;

pub struct ViewQuery<'s> {
    /// The id of the space in which context the query happens.
    pub space_view_id: SpaceViewId,

    /// The root of the space in which context the query happens.
    pub space_origin: &'s EntityPath,

    /// All [`DataResult`]s that are queried by active visualizers.
    ///
    /// Contains also invisible objects, use `iter_visible_data_results` to iterate over visible ones.
    pub per_visualizer_data_results: PerSystemDataResults<'s>,

    /// The timeline we're on.
    pub timeline: Timeline,

    /// The time on the timeline we're currently at.
    pub latest_at: TimeInt,

    /// Hover/select highlighting information for this space view.
    ///
    /// TODO(andreas): This should be the result of a [`crate::ViewContextSystem`] instead?
    pub highlights: SpaceViewHighlights,
}

impl<'s> ViewQuery<'s> {
    /// Iter over all of the currently visible [`DataResult`]s for a given `ViewSystem`
    pub fn iter_visible_data_results<'a>(
        &'a self,
        ctx: &'a ViewContext<'a>,
        visualizer: ViewSystemIdentifier,
    ) -> impl Iterator<Item = &DataResult>
    where
        's: 'a,
    {
        self.per_visualizer_data_results.get(&visualizer).map_or(
            itertools::Either::Left(std::iter::empty()),
            |results| {
                itertools::Either::Right(
                    results
                        .iter()
                        .filter(|result| result.is_visible(ctx.viewer_ctx))
                        .copied(),
                )
            },
        )
    }

    /// Iterates over all [`DataResult`]s of the [`ViewQuery`].
    #[inline]
    pub fn iter_all_data_results(&self) -> impl Iterator<Item = &DataResult> + '_ {
        self.per_visualizer_data_results
            .values()
            .flat_map(|data_results| data_results.iter().copied())
    }

    /// Iterates over all entities of the [`ViewQuery`].
    #[inline]
    pub fn iter_all_entities(&self) -> impl Iterator<Item = &EntityPath> + '_ {
        self.iter_all_data_results()
            .map(|data_result| &data_result.entity_path)
            .unique()
    }

    #[inline]
    pub fn latest_at_query(&self) -> LatestAtQuery {
        LatestAtQuery::new(self.timeline, self.latest_at)
    }
}
