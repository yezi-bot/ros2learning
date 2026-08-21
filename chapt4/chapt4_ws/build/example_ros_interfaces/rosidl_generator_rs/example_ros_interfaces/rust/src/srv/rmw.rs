#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "example_ros_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__example_ros_interfaces__srv__MoveRobot_Request() -> *const std::ffi::c_void;
}

#[link(name = "example_ros_interfaces__rosidl_generator_c")]
extern "C" {
    fn example_ros_interfaces__srv__MoveRobot_Request__init(msg: *mut MoveRobot_Request) -> bool;
    fn example_ros_interfaces__srv__MoveRobot_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Request>, size: usize) -> bool;
    fn example_ros_interfaces__srv__MoveRobot_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Request>);
    fn example_ros_interfaces__srv__MoveRobot_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveRobot_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Request>) -> bool;
}

// Corresponds to example_ros_interfaces__srv__MoveRobot_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveRobot_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: f32,

}



impl Default for MoveRobot_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !example_ros_interfaces__srv__MoveRobot_Request__init(&mut msg as *mut _) {
        panic!("Call to example_ros_interfaces__srv__MoveRobot_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveRobot_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { example_ros_interfaces__srv__MoveRobot_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { example_ros_interfaces__srv__MoveRobot_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { example_ros_interfaces__srv__MoveRobot_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveRobot_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveRobot_Request where Self: Sized {
  const TYPE_NAME: &'static str = "example_ros_interfaces/srv/MoveRobot_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__example_ros_interfaces__srv__MoveRobot_Request() }
  }
}


#[link(name = "example_ros_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__example_ros_interfaces__srv__MoveRobot_Response() -> *const std::ffi::c_void;
}

#[link(name = "example_ros_interfaces__rosidl_generator_c")]
extern "C" {
    fn example_ros_interfaces__srv__MoveRobot_Response__init(msg: *mut MoveRobot_Response) -> bool;
    fn example_ros_interfaces__srv__MoveRobot_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Response>, size: usize) -> bool;
    fn example_ros_interfaces__srv__MoveRobot_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Response>);
    fn example_ros_interfaces__srv__MoveRobot_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveRobot_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveRobot_Response>) -> bool;
}

// Corresponds to example_ros_interfaces__srv__MoveRobot_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveRobot_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: f32,

}



impl Default for MoveRobot_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !example_ros_interfaces__srv__MoveRobot_Response__init(&mut msg as *mut _) {
        panic!("Call to example_ros_interfaces__srv__MoveRobot_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveRobot_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { example_ros_interfaces__srv__MoveRobot_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { example_ros_interfaces__srv__MoveRobot_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { example_ros_interfaces__srv__MoveRobot_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveRobot_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveRobot_Response where Self: Sized {
  const TYPE_NAME: &'static str = "example_ros_interfaces/srv/MoveRobot_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__example_ros_interfaces__srv__MoveRobot_Response() }
  }
}






#[link(name = "example_ros_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__example_ros_interfaces__srv__MoveRobot() -> *const std::ffi::c_void;
}

// Corresponds to example_ros_interfaces__srv__MoveRobot
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveRobot;

impl rosidl_runtime_rs::Service for MoveRobot {
    type Request = MoveRobot_Request;
    type Response = MoveRobot_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__example_ros_interfaces__srv__MoveRobot() }
    }
}


