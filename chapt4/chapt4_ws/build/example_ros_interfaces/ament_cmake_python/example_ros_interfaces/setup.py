from setuptools import find_packages
from setuptools import setup

setup(
    name='example_ros_interfaces',
    version='0.0.0',
    packages=find_packages(
        include=('example_ros_interfaces', 'example_ros_interfaces.*')),
)
