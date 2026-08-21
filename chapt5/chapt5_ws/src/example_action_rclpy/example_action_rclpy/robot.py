
import math
from robot_control_interfaces.action import MoveRobot

class Robot():
    def __init__(self):
        self.current_pose = 0.0
        self.target_pose = 0.0
        self.move_distance=0.0
        self.status=MoveRobot.Feedback
    
    def get_status(self):
        """获取状态"""
        return self.status

    def get_current_pose(self):
        """获取当前位置"""
        return self.current_pose

    def close_goal(self):
        """接近目标"""
        return math.fabs(self.target_pose-self.current_pose)<0.01

    def stop_move(self):
        """停止移动"""
        self.status=MoveRobot.Feedback.STATUS_STOP

    def move_step(self):
        """移动一小步"""
        diret = self.move_distance/math.fabs(self.move_distance)
        step = diret*math.fabs(self.target_pose-self.current_pose)*0.1
        self.current_pose+=step
        return self.current_pose

    def set_goal(self, distance):
        """设置目标"""
        self.move_distance=distance
        self.target_pose+=distance
        if self.close_goal():
            self.stop_move()
            return False
        self.status = MoveRobot.FeedBack.STATUS_MOVING
        return True