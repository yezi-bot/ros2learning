import rclpy
from rclpy.node import Node
from time import sleep
import math 
from example_ros_interfaces.msg import RobotStatus
from example_ros_interfaces.srv import MoveRobot

class Robot():
    def __init__(self) ->Node:  
         self.current_pose_=0.0
         self.current_pose_=0.0
         self.status_=RobotStatus.STATUS_STOP

    def get_status(self):
        return self.status_
    
    def get_current_pose(self):
        return self.current_pose_     

    def move_distance(self,distance):
        self.status_=RobotStatus.STATUS_MOVING
        self.target_pose+=distance     #更新位置
        while math.fabs(self.target_pose - self.current_pose_)>0.01:
            step=distance/math.fabs(distance) * math.fabs(self.target_pose-self.current_pose_)*0.1
        self.status_=RobotStatus.STATUS_STOP
        return self.current_pose_
    
class ExampleInterfacesRobot02(Node):
    def __init__(self, name):
        super().__init__(name)
        self.get_logger().info("start")
        self.robot = Robot()
        self.move_robot_server = self.create_service(MoveRobot,"move_robot",self.handle_move_robot)
        self.robot_status_publsiher = self.create_publisher(RobotStatus,"robot_status",10)
        self.publisher_timer = self.create_timer(0.5,self.publisher_timer_callback)

    def publisher_timer_callback(self):
        msg=RobotStatus()#构造消息
        msg.status=self.robot.get_status()
        msg.pose=self.robot.get_current_pose()
        self.robot_status_publsiher.publish(msg)
        self.get_logger().info(f"publish current {msg.status} {msg.pose}") 

    def handle_move_robot(self,request,response):
        self.robot.move_distance(request.distance)   
        response.pose = self.robot.get_current_pose()
        return response
       
def main(args=None):
    rclpy.init(args=args)        
    node = ExampleInterfacesRobot02("example_interfaces_robot_02")
    rclpy.spin(node)
    rclpy.shutdown()