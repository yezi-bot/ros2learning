import rclpy
from rclpy.node import Node

class ParameterBasicNode(Node):
    def __init__(self,name):
        super().__init__(name)
        self.get_logger().info("start node")
        #declare parameters
        self.declare_parameter('rcl_log_level',0)
        log_level = self.get_parameter("rcl_log_level").value
        self.get_logger().set_level(log_level)
        self.timer=self.create_timer(0.5,self.timer_callback)

  
    def timer_callback(self):
        log_level =self.get_parameter("rcl_log_level").value
        self.get_logger().set_level(log_level)
        print("put")

def main(args=None):
    rclpy.init(args=args)
    node = ParameterBasicNode("parameters_basic")
    rclpy.spin(node)
    rclpy.shutdown()        
