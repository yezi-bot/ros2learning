from launch import LaunchDescription  #describe launch
from launch_ros.actions import Node

def generate_launch_description():
    

    parameters_basic2=Node(
        package="example_parameters_rclpy",
        namespace="rclpy",
        executable="parameters_basic",
        parameters=[{'rcl_log_level':50}]
    )  #list

    #describe the launch file
    launch_description=LaunchDescription([parameters_basic2])
    return launch_description

