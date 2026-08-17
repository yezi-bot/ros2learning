# 导入库文件
import rclpy
from rclpy.node import Node    #模块的子模块导入类

def main(args=None):
    rclpy.init(args=args)  #初始化rclpy
    node = Node("node_02")  #新节点
    node.get_logger().info("node_02")
    rclpy.spin(node)
    rclpy.shutdown()
