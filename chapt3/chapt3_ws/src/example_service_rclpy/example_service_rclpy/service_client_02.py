import rclpy
from rclpy.node import Node
from example_interfaces.srv import AddTwoInts

class ServiceClient02(Node):
    def __init__(self,name):
       super().__init__(name)
       self.get_logger().info("节点启动")
       self.client_=self.create_client(AddTwoInts,"add_two_ints_srv")

    def result_callback_(self,result_future):
        response = result_future.result() 
        self.get_logger().info(f"receive return result {response.sum}")  

    def send_request(self,a,b):
        while rclpy.ok() and self.client_.wait_for_service(1)==False:
            self.get_logger().info(f"等待服务端")

        request = AddTwoInts.Request() #请求结构体
        request.a=a
        request.b=b
        self.client_.call_async(request).add_done_callback(self.result_callback_)   #回调函数 
        
def main(args=None):
    rclpy.init(args=args)
    node = ServiceClient02("service_client_02")       
    node.send_request(3,4)
    rclpy.spin(node)
    rclpy.shutdown()