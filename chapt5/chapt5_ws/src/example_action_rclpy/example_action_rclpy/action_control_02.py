import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from robot_control_interfaces.action import MoveRobot

class ActionControl02(Node):
    def __init__(self,name):
        super().__init__(name)
        self.get_logger().info("start")
        self.action_client = ActionClient(self,MoveRobot,'move_robot')
        self.send_goal_timer=self.create_timer(1,self.send_goal)

    def send_goal(self):
        self.send_goal_timer.cancel()
        goal_msg=MoveRobot.Goal()
        goal_msg.distance=5.0
        self.action_client.wait_for_server()
        #async异步发送目标，过程反馈函数
        self._send_goal_future=self.action_client.send_goal_async(goal_msg,feedback_callback=self.feedback_callback)
        self._send_goal_future.add_done_callback(self.goal_response_callback)

    def goal_response_callback(self,future):
        goal_handle=future.result()  #取出future的结果
        if not goal_handle.accepted:
            self.get_logger().info('Goal rejected')    
            return
        self.get_logger().info('Goal accepted')
        self._get_result_future = goal_handle.get_result_async()  #发起新的异步等待
        self._get_result_future.add_done_callback(self.get_result_callback)

    def get_result_callback(self,future):
        result = future.reslut().result
        self.get_logger().info(f'result:{result.pose}')

    def feedback_callback(self,feedback_msg):
        feedback=feedback_msg.feedback
        self.get_logger().info(f'Receive feedback{feedback}')    
            

    