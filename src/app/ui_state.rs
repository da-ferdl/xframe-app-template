use nohash_hasher::{IntMap, IntSet};
use std::any::Any;
use xframe::egui::ahash::{HashMapExt, HashSetExt};

pub struct UiStateMap {
    /// Map which holds the state item instances.
    state_item_map: IntMap<u64, Box<dyn Any>>,
    /// Set which is filled on every ui render pass to detect at the end
    /// of the pass which state items are in use and which need to be dropped.
    state_item_ui_pass_ids: IntSet<u64>,

    /// Incrementing integer id for ui-state item provider.
    state_item_auto_id: u64,

    /// Incrementing integer for unique-int provider.
    unique_int_value: u64,
}
impl UiStateMap {
    pub fn new() -> Self {
        Self {
            state_item_map: IntMap::with_capacity(256),
            state_item_ui_pass_ids: IntSet::with_capacity(256),
            state_item_auto_id: 0,
            unique_int_value: 0,
        }
    }

    /// Returns a mutable reference to the state type `T`.
    ///
    /// If the type is not available yet it is created with the given `builder`.
    ///
    /// The type `T` is dropped as soon not used on the ui anymore (after a ui frame
    /// pass it is detected that the type was not used).
    pub fn get_or_init_state_item<T: Any>(
        &mut self,
        id: u64,
        builder: impl FnOnce() -> T,
    ) -> &mut T {
        self.state_item_ui_pass_ids.insert(id);

        self.state_item_map
            .entry(id)
            .or_insert_with(|| Box::new(builder()))
            .downcast_mut::<T>()
            .expect("downcast to expected type should be impossible to fail here")
    }

    /// On every call this increments the inner `state_item_auto_id` and returns the incremented value.
    ///
    /// Used to give each `ui_state!()` state item a unique id.
    pub fn get_next_state_item_auto_id(&mut self) -> u64 {
        self.state_item_auto_id += 1;

        if self.state_item_auto_id == u64::MAX {
            self.state_item_auto_id = 0;
        }

        self.state_item_auto_id
    }

    /// Must be called after every ui render pass - this detects which state
    /// items must be deallocated.
    pub fn on_egui_end_pass(&mut self) {
        let used_ids: Vec<u64> = self.state_item_ui_pass_ids.drain().collect();
        self.state_item_map.retain(|id, _| used_ids.contains(id));
    }

    /// On every call this increments the inner `unique_int_value` and returns the incremented value.
    ///
    /// Used for the `unique_int!()` macro.
    pub fn get_next_unique_int_value(&mut self) -> u64 {
        self.unique_int_value += 1;

        if self.unique_int_value == u64::MAX {
            self.unique_int_value = 0;
        }

        self.unique_int_value
    }
}

/// This can be used to set a `UiStateMap` mock implementation for tests.
///
/// Must be set before `ui_state` is used the first time, otherwise the default
/// `UiStateMap` implementation is set.
///
/// Can only be called once, further calls are noop.
pub fn set_ui_state_map(state_map: UiStateMap) {
    #[expect(static_mut_refs)]
    if unsafe { UI_STATE_HOLDER.is_some() } {
        return;
    }

    unsafe { UI_STATE_HOLDER = Some(state_map) };
}

/// Must be called on every ui pass end, eg. At the end on the
/// `XFrameApp::ui` implementation.
pub fn on_ui_pass_end() {
    state_holder().on_egui_end_pass();
}

/// Static `UiStateMap` instance.
pub fn state_holder<'s>() -> &'s mut UiStateMap {
    #[expect(static_mut_refs)]
    unsafe {
        UI_STATE_HOLDER.get_or_insert_with(|| UiStateMap::new())
    }
}

static mut UI_STATE_HOLDER: Option<UiStateMap> = None;

/// UI state type provider.
///
/// Creates the state type on first access with the provided builder and
/// de-allocates the state instance when not used anymore inside a egui UI.
///
/// Returns always a mutable reference to the state type - either newly created
/// if not currently not available, or the reference to the currently active one.
#[macro_export]
macro_rules! ui_state {
    ($builder:expr) => {{
        use crate::app::ui_state::state_holder;

        static mut LOCAL_ID: Option<u64> = None;
        #[expect(static_mut_refs)]
        let id =
            unsafe { LOCAL_ID.get_or_insert_with(|| state_holder().get_next_state_item_auto_id()) };

        state_holder().get_or_init_state_item(*id, $builder)
    }};
}

/// Unique `u64` integer value provider.
///
/// Provides a unique integer that is created on first call and
/// further calls return the same created value.
#[macro_export]
macro_rules! unique_int {
    () => {{
        use crate::app::ui_state::state_holder;

        static mut LOCAL_ID: Option<u64> = None;
        #[expect(static_mut_refs)]
        let value =
            unsafe { LOCAL_ID.get_or_insert_with(|| state_holder().get_next_unique_int_value()) };

        *value
    }};
}
