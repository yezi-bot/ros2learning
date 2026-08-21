#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to example_ros_interfaces__srv__MoveRobot_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveRobot_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: f32,

}



impl Default for MoveRobot_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MoveRobot_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MoveRobot_Request {
  type RmwMsg = super::srv::rmw::MoveRobot_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        distance: msg.distance,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      distance: msg.distance,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      distance: msg.distance,
    }
  }
}


// Corresponds to example_ros_interfaces__srv__MoveRobot_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveRobot_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: f32,

}



impl Default for MoveRobot_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MoveRobot_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MoveRobot_Response {
  type RmwMsg = super::srv::rmw::MoveRobot_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        pose: msg.pose,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      pose: msg.pose,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      pose: msg.pose,
    }
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


