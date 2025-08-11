use ascot::device::DeviceKind;

use alloc::vec::Vec;

pub struct DeviceAction {

}

/// A general smart home device.
pub struct Device {
    // Kind.
    kind: DeviceKind,
    // Main device route.
    main_route: &'static str,
    // All device routes with their hazards and handlers.
    routes_data: Vec<DeviceAction>,
}
