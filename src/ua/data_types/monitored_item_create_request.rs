use open62541_sys::UA_MonitoredItemCreateRequest_default;

use crate::ua;

crate::data_type!(
    MonitoredItemCreateRequest,
    UA_MonitoredItemCreateRequest,
    UA_TYPES_MONITOREDITEMCREATEREQUEST
);

impl MonitoredItemCreateRequest {
    #[must_use]
    // TODO: Find better name for this method.
    pub fn init_node_id(node_id: ua::NodeId) -> Self {
        let inner = unsafe { UA_MonitoredItemCreateRequest_default(node_id.into_inner()) };
        Self(inner)
    }
}
