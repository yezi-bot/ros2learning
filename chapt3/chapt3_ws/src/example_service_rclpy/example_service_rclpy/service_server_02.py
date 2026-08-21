import rclpy
from rclpy.node import Node
from example_interfaces.srv import AddTwoInts

class ServiceServer02(Node):
    def __init__(self,name):
       super().__init__(name)
       self.get_logger().info("节点启动")
       self.add_ints_server_ = self.create_service(AddTwoInts,"add_two_ints_srv",self.handle_add_two_ints) #服务类型，服务名称，回调函数

    def handle_add_two_ints(self,request,response):
        self.get_logger().info(f"receive request a={request.a},b={request.b}")
        response.sum = request.a+request.b
        return response
    
def main(args=None):
    rclpy.init(args=args)
    node = ServiceServer02("service_server_02")
    rclpy.spin(node)
    rclpy.shutdown()



