"""
This module provides classes and base functionality for building OPQMauka plugins.
"""

import json
import multiprocessing
import signal
import threading
import typing

# noinspection PyPackageRequirements
import bson
# noinspection PyPackageRequirements
import bson.objectid
import zmq

import log
import mongo
import protobuf.mauka_pb2
import protobuf.util

# pylint: disable=C0103
logger = log.get_logger(__name__)


def run_plugin(plugin_class, config: typing.Dict):
    """Runs the given plugin using the given configuration dictionary

    :param plugin_class: Name of the class of the plugin to be ran
    :param config: Configuration dictionary
    """

    def _run_plugin():
        """Inner function that acts as target to multiprocess constructor"""
        plugin_instance = plugin_class(config)
        plugin_instance.run_plugin()

    process = multiprocessing.Process(target=_run_plugin)
    process.start()
    return process


class JSONEncoder(json.JSONEncoder):
    """
    This class allows us to serialize items with ObjectIds to JSON
    """

    # https://github.com/PyCQA/pylint/issues/414
    # pylint: disable=E0202
    def default(self, o):
        """If o is an object id, return the string of it, otherwise use the default encoder for this object

        :param o: Object to serialize
        :return: Serialized version of this object
        """
        if isinstance(o, bson.ObjectId):
            return str(o)
        return json.JSONEncoder.default(self, o)


def from_json(json_str: str) -> typing.Dict:
    """Deserialize json into dictionary

    :param json_str: JSON string to deserialize
    :return: Dictionary from json
    """
    return json.loads(json_str)


def to_json(obj: object) -> str:
    """Serializes the given object to json

    :param obj: The object to serialize
    :return: JSON representation of object
    """
    return json.dumps(obj, cls=JSONEncoder)


class MaukaPlugin:
    """
    This is the base MaukaPlugin class that provides easy access to the database and also provides publish/subscribe
    semantics and distributed processing primitives.
    """

    NAME = "MaukaPlugin"

    def __init__(self, config: typing.Dict, subscriptions: typing.List[str], name: str,
                 exit_event: multiprocessing.Event):
        """ Initializes the base plugin

        :param config: Configuration dictionary
        :param subscriptions: List of subscriptions this plugin should subscribe to
        :param name: The name of this plugin
        """

        self.config = config
        """Configuration dictionary"""

        self.subscriptions = subscriptions
        """Plugin subscriptions"""

        self.name = name
        """Plugin name"""

        self.exit_event = exit_event
        """Multiprocessing primitive that when set allows us to easily exit a process or thread"""

        self.mongo_client = self.get_mongo_client()
        """MongoDB OPQ client"""

        self.zmq_context = zmq.Context()
        """ZeroMQ context"""

        # noinspection PyUnresolvedReferences
        # pylint: disable=E1101
        self.zmq_consumer = self.zmq_context.socket(zmq.SUB)
        """ZeroMQ consumer"""

        # noinspection PyUnresolvedReferences
        # pylint: disable=E1101
        self.zmq_producer = self.zmq_context.socket(zmq.PUB)
        """ZeroMQ producer"""

        self.heartbeat_interval_s = float(self.config_get("plugins.base.heartbeatIntervalS"))
        """How often in seconds this plugin should produce a heartbeat"""

        self.on_message_cnt = 0
        """Number of times this plugin has received a message since starting"""

        self.last_received = 0
        """Timestamp since this plugin has last received a message"""

        self.logger = logger
        """Provides access to a single configured logger all plugins can use"""

        self.producer_lock = multiprocessing.Lock()
        """Lock that ensures the base plugin is thread safe while producing messages"""

        self.zmq_consumer.connect(self.config_get("zmq.mauka.plugin.sub.interface"))
        self.zmq_producer.connect(self.config_get("zmq.mauka.plugin.pub.interface"))

        # Every plugin subscribes to itself to allow for plugin control
        self.subscriptions.append(name)

        self.mauka_debug = self.config["mauka.debug"]

    # pylint: disable=R0201
    def get_status(self) -> str:
        """ Return the status of this plugin
        :return: The status of this plugin
        """
        return "N/A"

    def get_mongo_client(self):
        """ Returns an OPQ mongo client

        :return: An OPQ mongo client
        """
        mongo_host = self.config_get("mongo.host")
        mongo_port = self.config_get("mongo.port")
        mongo_db = self.config_get("mongo.db")
        return mongo.OpqMongoClient(mongo_host, mongo_port, mongo_db)

    def start_heartbeat(self):
        """
        This is a recursive function that acts as a heartbeat.

        This function calls itself over-and-over on a timer to produce heartbeat messages. The interval can be
        configured is the configuration file.
        """

        start_after_seconds = 5.0

        def heartbeat():
            """
            Recursively produces a heartbeat message on a timer.
            """
            heartbeat_message = protobuf.util.build_heartbeat(self.name,
                                                              self.last_received,
                                                              self.on_message_cnt,
                                                              self.get_status())
            self.produce("heartbeat", heartbeat_message)
            timer = threading.Timer(self.heartbeat_interval_s, heartbeat)
            timer.start()

        threading.Timer(start_after_seconds, heartbeat).start()

    def config_get(self, key: str) -> str:
        """Retrieves a value from the configuration dictionary

        :param key: The key associated with the value we're looking to retrieve
        :return: The value associated with the provided key
        :raises KeyError: When key is not in the configuration
        """
        if key not in self.config:
            raise KeyError("Key {} not in config".format(key))
        else:
            return self.config[key]

    # pylint: disable=W0613
    # pylint: disable=R0201
    def on_message(self, topic: str, mauka_message: protobuf.mauka_pb2.MaukaMessage):
        """This gets called when a subscriber receives a message from a topic they are subscribed too.

        This should be implemented in all subclasses.

        :param topic: The topic this message is associated with
        :param mauka_message: The message contents
        """
        logger.info("on_message not implemented")

    def produce(self, topic: str, mauka_message: protobuf.mauka_pb2.MaukaMessage):
        """Produces a message with a given topic to the system

        :param topic: The topic to produce this message to
        :param mauka_message: The message to produce
        """
        serialized_mauka_message = protobuf.util.serialize_mauka_message(mauka_message)
        with self.producer_lock:
            self.zmq_producer.send_multipart((topic.encode(), serialized_mauka_message))

    def is_self_message(self, topic: str) -> bool:
        """Determines if this is a message directed at this plugin. I.e. the topic is the name of the plugin.

        :param topic: Topic of the message
        :return: If this is a self message or not
        """
        return topic == self.name

    def handle_self_message(self, message: str):
        """Handles a self-message

        :param message: The message to handle
        """
        if message == "EXIT":
            self.exit_event.set()

    def debug(self, msg: str):
        """
        Prints a debug message using this classes logger and formatted the plugin name.
        :param msg: Message to print to debug.
        """
        if self.mauka_debug:
            self.logger.debug("%s\n%s", self.name, msg)

    def run_plugin(self):
        """This is the run loop for this plugin process"""
        logger.info("Starting Mauka plugin: %s", self.name)
        signal.signal(signal.SIGTERM, signal.SIG_IGN)
        signal.signal(signal.SIGINT, signal.SIG_IGN)

        for subscription in self.subscriptions:
            # noinspection PyUnresolvedReferences
            # pylint: disable=E1101
            self.zmq_consumer.setsockopt_string(zmq.SUBSCRIBE, subscription)

        self.start_heartbeat()

        while not self.exit_event.is_set():
            data = self.zmq_consumer.recv_multipart()

            if len(data) != 2:
                logger.error("Malformed data from ZMQ. Data size should = 2, but instead is %s", str(len(data)))
                for data_item in data:
                    logger.error("%s", str(data_item.decode()))
                break

            topic = data[0].decode()
            message = data[1]

            if self.is_self_message(topic):
                logger.info("Receive self message")
                self.handle_self_message(message.decode())
            else:
                # Update statistics
                self.on_message_cnt += 1
                self.last_received = protobuf.util.get_timestamp_ms()
                mauka_message = protobuf.util.deserialize_mauka_message(message)
                self.on_message(topic, mauka_message)

        logger.info("Exiting Mauka plugin: %s", self.name)
