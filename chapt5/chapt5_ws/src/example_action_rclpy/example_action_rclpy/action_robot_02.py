import time
#rclpy
import rclpy
from rclpy.node import Node
from rclpy.action import ActionServer
from rclpy.action.server import ServerGoalHandle
#interface
from robot_control_interfaces.action import MoveRobot
#robot
from example_action_rclpy.robot import Robot

class ActionRobot02(Node):
    def __init__(self,name):
        super().__init__(name)
        self.get_logger().info("start")
        self.robot =Robot()
        self.action_server=ActionServer(self,MoveRobot,'move_robot',self.execute_callback)

    def execute_callback(self,goal_handle:ServerGoalHandle):
        self.get_logger().info("execute moving robot")
        feedback_msg = MoveRobot.Feedback()
        self.robot.set_goal(goal_handle.request.distance)
        while rclpy.ok()and not self.robot.close_goal():
            self.robot.move_step()
            feedback_msg.pose=self.robot.get_current_pose()
            feedback_msg.status=self.robot.get_status()
            goal_handle.publish_feedback(feedback_msg)
            if goal_handle.is_cancel_requested:
                result = MoveRobot.Result()
                result.pose=self.robot.get_current_pose()
                return result
            time.sleep(0.5)
        goal_handle.succeed()
        result=MoveRobot.Result()
        result.pose=self.robot.get_current_pose()
        return result
    
def main(args=None):
    rclpy.init(args=args)
    action_robot_02 = ActionRobot02("action_robot_02")
    rclpy.spin(action_robot_02)
    rclpy.shutdown()


