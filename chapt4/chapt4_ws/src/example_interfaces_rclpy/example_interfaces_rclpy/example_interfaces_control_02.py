import rclpy
from rclpy.node import Node
from example_ros_interfaces.msg import RobotStatus
from example_ros_interfaces.srv import MoveRobot

class ExampleInterfacesControl02(Node):
    def __init__(self, name):
        super().__init__(name)
        self.get_logger().info("start")
        self.client_=self.create_client(MoveRobot,"move_robot")
        self.robot_status_subscribe_=self.create_subscription(RobotStatus,"robot_status",self.robot_status_callback,10)

    def robot_status_callback(self,msg):
        self.get_logger().info(f"receive status {msg.pose} {msg.status}")

    def move_result_callback(self,result_future):
        response = result_future.result()
        self.get_logger().info(f"receive result {response.pose}")   

    def move_robot(self,distance):
       while rclpy.ok() and self.client_.wait_for_service(1)==False:
           self.get_logger().log("waiting for service")
           request=MoveRobot.Request()
           request.distance = distance
           self.client_.call_async(request).add_done_callback(self.move_result_callback)

def main(args=None):
    rclpy.init(args=args)
    node =  ExampleInterfacesControl02("example_interfaces_control_02")
    node.move_robot(5.0)
    rclpy.spin(node)
    rclpy.shutdown()