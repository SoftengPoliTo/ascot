use std::collections::HashMap;

use ascot_library::hazards::Hazards;

// TODO: Eventually rewrite policy IDs as &'static str.

/// A privacy policy manager.
///
/// A privacy policy can be applied to all requests sent to devices or just to
/// specific ones.
#[derive(Debug, PartialEq)]
pub struct Policy {
    block_on_hazards: Hazards,
    block_device_on_hazards: HashMap<usize, Hazards>,
}

impl Policy {
    /// Creates a [`Policy`] to block the sending of all requests with
    /// the given [`Hazards`].
    #[must_use]
    #[inline]
    pub fn new(block_on_hazards: Hazards) -> Self {
        Self {
            block_on_hazards,
            block_device_on_hazards: HashMap::new(),
        }
    }

    /// Creates a [`Policy`] to block the sending of requests for the
    /// [`crate::device::Device`] with the given identifier and [`Hazards`].
    #[must_use]
    #[inline]
    pub fn no_global_policy(id: usize, hazards: Hazards) -> Self {
        let policy = Self::init();
        policy.block_device_on_hazards(id, hazards)
    }

    /// Adds a [`Policy`] rule to block the sending of requests for the
    /// [`crate::device::Device`] with the given identifier and [`Hazards`].
    #[must_use]
    #[inline]
    pub fn block_device_on_hazards(mut self, id: usize, hazards: Hazards) -> Self {
        self.block_device_on_hazards.insert(id, hazards);
        self
    }

    pub(crate) fn init() -> Self {
        Self {
            block_on_hazards: Hazards::new(),
            block_device_on_hazards: HashMap::new(),
        }
    }

    pub(crate) fn global_blocked_hazards(&self, hazards: &Hazards) -> Hazards {
        let mut blocked_hazards = Hazards::new();
        for hazard in hazards {
            if self.block_on_hazards.contains(hazard) {
                blocked_hazards.add(*hazard);
            }
        }
        blocked_hazards
    }

    pub(crate) fn local_blocked_hazards(&self, id: usize, hazards: &Hazards) -> Hazards {
        if let Some(local_hazards) = self.block_device_on_hazards.get(&id) {
            let mut blocked_hazards = Hazards::new();
            for hazard in hazards {
                if local_hazards.contains(hazard) {
                    blocked_hazards.add(*hazard);
                }
            }
            blocked_hazards
        } else {
            Hazards::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use ascot_library::hazards::Hazard;

    use super::{HashMap, Hazards, Policy};

    fn create_policy() -> (Hazards, Policy) {
        let hazards = Hazards::new().insert(Hazard::ElectricEnergyConsumption);

        let policy = Policy::new(hazards.clone());

        (hazards, policy)
    }

    fn check_device_policies(policy: &Policy, block_on_hazards: Hazards, local_hazards: &Hazards) {
        let mut devices_hazards = HashMap::new();
        devices_hazards.insert(1, local_hazards.clone());
        devices_hazards.insert(2, local_hazards.clone());

        assert_eq!(
            policy,
            &Policy {
                block_on_hazards,
                block_device_on_hazards: devices_hazards,
            }
        );
    }

    #[test]
    fn global_policy() {
        let (hazards, policy) = create_policy();

        assert_eq!(
            policy,
            Policy {
                block_on_hazards: hazards,
                block_device_on_hazards: HashMap::new()
            }
        );
    }

    #[test]
    fn no_global_policy() {
        let local_hazards = Hazards::new().insert(Hazard::Explosion);

        let policy = Policy::no_global_policy(1, local_hazards.clone())
            .block_device_on_hazards(2, local_hazards.clone());

        check_device_policies(&policy, Hazards::new(), &local_hazards);
    }

    #[test]
    fn devices_policy() {
        let (global_hazards, policy) = create_policy();

        let local_hazards = Hazards::new().insert(Hazard::Explosion);

        let policy = policy
            .block_device_on_hazards(1, local_hazards.clone())
            .block_device_on_hazards(2, local_hazards.clone());

        check_device_policies(&policy, global_hazards, &local_hazards);
    }
}
