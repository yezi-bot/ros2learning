#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "example_ros_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__example_ros_interfaces__msg__RobotPose() -> *const std::ffi::c_void;
}

#[link(name = "example_ros_interfaces__rosidl_generator_c")]
extern "C" {
    fn example_ros_interfaces__msg__RobotPose__init(msg: *mut RobotPose) -> bool;
    fn example_ros_interfaces__msg__RobotPose__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotPose>, size: usize) -> bool;
    fn example_ros_interfaces__msg__RobotPose__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotPose>);
    fn example_ros_interfaces__msg__RobotPose__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotPose>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotPose>) -> bool;
}

// Corresponds to example_ros_interfaces__msg__RobotPose
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotPose {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::Pose,

}

impl RobotPose {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_MOVING: u32 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_STOP: u32 = 2;

}


impl Default for RobotPose {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !example_ros_interfaces__msg__RobotPose__init(&mut msg as *mut _) {
        panic!("Call to example_ros_interfaces__msg__RobotPose__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotPose {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { example_ros_interfaces__msg__RobotPose__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { example_ros_interfaces__msg__RobotPose__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { example_ros_interfaces__msg__RobotPose__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotPose {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotPose where Self: Sized {
  const TYPE_NAME: &'static str = "example_ros_interfaces/msg/RobotPose";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__example_ros_interfaces__msg__RobotPose() }
  }
}


#[link(name = "example_ros_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__example_ros_interfaces__msg__RobotStatus() -> *const std::ffi::c_void;
}

#[link(name = "example_ros_interfaces__rosidl_generator_c")]
extern "C" {
    fn example_ros_interfaces__msg__RobotStatus__init(msg: *mut RobotStatus) -> bool;
    fn example_ros_interfaces__msg__RobotStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotStatus>, size: usize) -> bool;
    fn example_ros_interfaces__msg__RobotStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotStatus>);
    fn example_ros_interfaces__msg__RobotStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotStatus>) -> bool;
}

// Corresponds to example_ros_interfaces__msg__RobotStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: f32,

}

impl RobotStatus {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_MOVING: u32 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STATUS_STOP: u32 = 1;

}


impl Default for RobotStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !example_ros_interfaces__msg__RobotStatus__init(&mut msg as *mut _) {
        panic!("Call to example_ros_interfaces__msg__RobotStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { example_ros_interfaces__msg__RobotStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { example_ros_interfaces__msg__RobotStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { example_ros_interfaces__msg__RobotStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotStatus where Self: Sized {
  const TYPE_NAME: &'static str = "example_ros_interfaces/msg/RobotStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__example_ros_interfaces__msg__RobotStatus() }
  }
}


