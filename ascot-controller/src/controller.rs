/// A requests controller.
///
/// It manages how requests are sent through some decisions affected by:
///
/// - Privacy and security policies
/// - Scheduling programs
///
/// When a request is sent to a device, the returned response will be forwarded
/// to the caller.
pub struct Controller {
    // discovery: Discovery,
    // policy: Policy,
    // scheduler: Scheduler,
}

/*
impl Controller {
   pub fn new(devices: Devices) -> Self {
      Self {
        devices,
        policy: Policy::empty(),
        scheduler: Scheduler::empty(),
      }
   }
}

// Configure a controller
let controller = Controller::new(Discovery)
.block_actions_with_hazards(&[Hazard])
// TODO: Define an ID on devices to tell which kind of devices switch on and
// off!!!!
// Run a parallel thread to perform that.
// Device Id, Action id, [start_time, end_time]
.schedule_task(Device.id("DiningRoomLight"), "/on", [19:30, 22]));
.configure()?; --> Checks if the passed device id, action and everything else is
correct

// Show discovered devices.
controller.show_devices();


// Change blocking rules
controller.change_blocked_hazards(&[Hazard]);

controller.change_scheduled_tasks(...)?;


let device_runtime = controller.get_device(id); Result<Device, RuntimeError>
let action_runtime = device_runtime.get_action("/on"); Result<ActionRuntime, RuntimeError>

// Run with the input parameters.
let response = action_runtime.run_with_params(Inputs::empty().insert_f64("value", 5.0)).await?; Result<Response; RuntimeError>

or

// Run without any params (using default input values)
let response = action_runtime.run_without_params().await?; Result<ReponseRuntimeError>



// Run again discover.
controller.discover()?;


*/
