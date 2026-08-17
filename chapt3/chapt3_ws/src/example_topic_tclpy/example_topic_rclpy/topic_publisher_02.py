import rclpy
from rclpy.node import Node
from std_msgs.msg import String

class NodePublisher02(Node):
    def __init__(self, name):
        super().__init__(name)
        self.get_logger().info("%s" %name)
        self.command_publisher_=self.create_publisher(String,"command",10)  #消息接口，话题名称，服务质量
        self.timer = self.create_timer(0.5,self.timer_callback)#h回调周期，回调函数

    def timer_callback(self):
        msg=String()
        msg.data='backup'
        self.command_publisher_.publish(msg)
        self.get_logger().info(f'发布了{msg.data}')

def main(args=None):
    rclpy.init(args=args)
    node = NodePublisher02("topic_publisher_02")
    rclpy.spin(node)
    rclpy.shutdown()