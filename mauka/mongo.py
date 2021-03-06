"""
This module contains classes and functions for querying and manipulating data within a mongo database.
"""

import enum
import typing

import bson
import bson.objectid

import gridfs
import numpy
import pymongo

import analysis
import constants


def to_s16bit(data: bytes) -> numpy.ndarray:
    """
    Converts raw bytes into an array of 16 bit integers.
    :param data:
    :return:
    """
    return numpy.frombuffer(data, numpy.int16)


class IEEEDuration(enum.Enum):
    """String enumerations and constants for durations"""
    INSTANTANEOUS = "INSTANTANEOUS"  # A duration between 0.5 and 30 cycles
    MOMENTARY = "MOMENTARY"  # A duration between 30 cycles and 3 seconds
    TEMPORARY = "TEMPORARY"  # A duration between 3 seconds and 1 minute
    SUSTAINED = "SUSTAINED"  # A duration greater than 1 minute


class BoxEventType(enum.Enum):
    """String enumerations and constants for event types"""
    FREQUENCY_DIP = "FREQUENCY_SAG"
    FREQUENCY_SWELL = "FREQUENCY_SWELL"
    FREQUENCY_INTERRUPTION = "FREQUENCY_INTERRUPTION"
    VOLTAGE_DIP = "VOLTAGE_SAG"
    VOLTAGE_SWELL = "VOLTAGE_SWELL"
    THD = "THD"
    OTHER = "OTHER"


class Collection(enum.Enum):
    """String enumerations and constants for collection names"""
    MEASUREMENTS = "measurements"
    EVENTS = "events"
    BOX_EVENTS = "box_events"
    OPQ_BOXES = "opq_boxes"
    INCIDENTS = "incidents"


class OpqMongoClient:
    """Convenience mongo client for easily operating on OPQ data"""

    def __init__(self, host: str = "127.0.0.1", port: int = 27017, database_name: str = "opq"):
        """
        Initializes an OpqMongoClient.
        :param host: Host of mongo database
        :param port: Port of mongo database
        :param database_name: The name of the database on mongo
        """

        self.client = pymongo.MongoClient(host, port)
        """MongoDB client"""

        self.database = self.client[database_name]
        """MongoDB"""

        self.gridfs = gridfs.GridFS(self.database)
        """Access to MongoDB gridfs"""

        self.events_collection = self.get_collection(Collection.EVENTS.value)
        """Events collections"""

        self.measurements_collection = self.get_collection(Collection.MEASUREMENTS.value)
        """Measurements collection"""

        self.box_events_collection = self.get_collection(Collection.BOX_EVENTS.value)
        """Box events collection"""

        self.opq_boxes_collection = self.get_collection(Collection.OPQ_BOXES.value)
        """Opq boxes collection"""

        self.incidents_collection = self.get_collection(Collection.INCIDENTS.value)
        "Incidents collection"

    def get_collection(self, collection: str):
        """ Returns a mongo collection by name

        Parameters
        ----------
        collection : str
            Name of collection

        Returns
        -------
        A mongo collection

        """
        return self.database[collection]

    def drop_collection(self, collection: str):
        """Drops a collection by name

        Parameters
        ----------
        collection : str
            Name of the collection

        """
        self.database[collection].drop()

    def drop_indexes(self, collection: str):
        """Drop all indexes of a particular named collection

        Parameters
        ----------
        collection : str
            Name of the collection

        Returns
        -------

        """
        self.database[collection].drop_indexes()

    def read_file(self, fid: str) -> bytes:
        """
        Loads a file from gridfs as an array of bytes
        :param fid: The file name to open stored in gridfs
        :return: A list of bytes from reading the file
        """
        return self.gridfs.find_one({"filename": fid}).read()

    def write_incident_waveform(self, incident_id: int, gridfs_filename: str, payload: bytes):
        """
        Write incident waveform to database.
        :param incident_id: The id of the incident we are storing a waveform for.
        :param gridfs_filename: The filename to store the waveform to.
        :param payload: The bytes of the waveform.
        """
        self.gridfs.put(payload, **{"filename": gridfs_filename,
                                    "metadata": {"incident_id": incident_id}})


def get_waveform(mongo_client: OpqMongoClient, data_fs_filename: str) -> numpy.ndarray:
    """
    Returns the waveform at data_fs_filename.
    :param mongo_client:
    :param data_fs_filename:
    :return: The waveform stored at data_fs_filename as a numpy array.
    """
    data = mongo_client.read_file(data_fs_filename)
    waveform = to_s16bit(data).astype(numpy.int64)
    return waveform


def get_box_calibration_constants(mongo_client: OpqMongoClient = None, defaults=None) -> \
        typing.Dict[int, float]:
    """
    Loads calibration constants from the mongo database a a dictionary from box id to calibration constant.
    Default values can be passed in if needed.
    :param mongo_client: Optional mongo db opq client
    :param defaults: Default values supplied as a mapping from box id to calibration constant.
    :return: A dictionary mapping of box_id to calibration constant
    """
    if defaults is None:
        defaults = {}
    client = mongo_client if mongo_client is not None else OpqMongoClient()
    opq_boxes = client.opq_boxes_collection.find(projection={'_id': False,
                                                             "calibration_constant": True,
                                                             "box_id": True})
    calibration_constants = {}
    for box_id, calibration_constant in defaults.items():
        calibration_constants[box_id] = calibration_constant
    for opq_box in opq_boxes:
        calibration_constants[opq_box["box_id"]] = opq_box["calibration_constant"]
    return calibration_constants


def get_default_client(mongo_client: OpqMongoClient = None) -> OpqMongoClient:
    """
    Creates a new mongo client or reuses a passed in mongo client
    :param mongo_client: Mongo client
    :return: Mongo client
    """
    return mongo_client if mongo_client is not None else OpqMongoClient()


class IncidentMeasurementType(enum.Enum):
    """Incident Feature Extracted Measurement Types"""
    VOLTAGE = "VOLTAGE"
    FREQUENCY = "FREQUENCY"
    THD = "THD"
    TRANSIENT = "TRANSIENT"

class IncidentClassification(enum.Enum):
    """Incident Classification Types"""
    EXCESSIVE_THD = "EXCESSIVE_THD"
    ITIC_PROHIBITED = "ITIC_PROHIBITED"
    ITIC_NO_DAMAGE = "ITIC_NO_DAMAGE"
    VOLTAGE_SWELL = "VOLTAGE_SWELL"
    VOLTAGE_SAG = "VOLTAGE_SAG"
    VOLTAGE_INTERRUPTION = "VOLTAGE_INTERRUPTION"
    FREQUENCY_SWELL = "FREQUENCY_SWELL"
    FREQUENCY_SAG = "FREQUENCY_SAG"
    FREQUENCY_INTERRUPTION = "FREQUENCY_INTERRUPTION"
    SEMI_F47_VIOLATION = "SEMI_F47_VIOLATION"
    UNDEFINED = "UNDEFINED"

class IncidentIeeeDuration(enum.Enum):
    """IEEE Duration Classifications"""
    INSTANTANEOUS = "INSTANTANEOUS"
    MOMENTARY = "MOMENTARY"
    TEMPORARY = "TEMPORARY"
    SUSTAINED = "SUSTAINED"
    UNDEFINED = "UNDEFINED"


def get_ieee_duration(duration_ms: float) -> IncidentIeeeDuration:
    """
    Given a duration in milliseconds, return the corresponding IEEE duration classification.
    :param duration_ms: Duration in milliseconds.
    :return: IEEE duration classification.
    """
    ms_half_c = analysis.c_to_ms(0.5)
    ms_30_c = analysis.c_to_ms(30)
    ms_3_s = 3_000
    ms_1_m = 60_000

    if ms_half_c < duration_ms <= ms_30_c:
        return IncidentIeeeDuration.INSTANTANEOUS
    elif ms_30_c < duration_ms <= ms_3_s:
        return IncidentIeeeDuration.MOMENTARY
    elif ms_3_s < duration_ms <= ms_1_m:
        return IncidentIeeeDuration.TEMPORARY
    elif duration_ms > ms_1_m:
        return IncidentIeeeDuration.SUSTAINED

    return IncidentIeeeDuration.UNDEFINED


def next_available_incident_id(opq_mongo_client: OpqMongoClient) -> int:
    """
    Returns the next available incident id using the database as a source of truth
    :param opq_mongo_client: An optional mongo client to use for DB access (will be created if not-provided)
    :return: Next available incident id
    """
    mongo_client = get_default_client(opq_mongo_client)
    if mongo_client.incidents_collection.count() > 0:
        last_incident = mongo_client.incidents_collection.find().sort("incident_id", pymongo.DESCENDING).limit(1)[0]
        last_incident_id = last_incident["incident_id"]
        return last_incident_id + 1

    return 1


def store_incident(event_id: int,
                   box_id: str,
                   start_timestamp_ms: int,
                   end_timestamp_ms: int,
                   measurement_type: IncidentMeasurementType,
                   deviation_from_nominal: float,
                   classifications: typing.List[IncidentClassification],
                   annotations: typing.List = None,
                   metadata: typing.Dict = None,
                   opq_mongo_client: OpqMongoClient = None):
    """
    Creates and stores an incident in the database.
    :param event_id: The event_id that this incident is associated with.
    :param box_id: The box_id that this incident is associated with.
    :param start_timestamp_ms: The start timestamp of this incident.
    :param end_timestamp_ms: The end timestamp of this incident.
    :param measurement_type:  The feature measurement type this incident was derrived from.
    :param deviation_from_nominal: The maximum deviation from nominal during this incident.
    :param classifications: A list of classifications assigned to this incident
    :param annotations: A list of annotations assigned to this incident
    :param metadata: An incident specific dictionary of key-pair values providing extra context to this incident
    :param opq_mongo_client: An optional mongo client to use for DB access (will be created if not-provided)
    """

    mongo_client = get_default_client(opq_mongo_client)

    box_event = mongo_client.box_events_collection.find_one({"event_id": event_id,
                                                             "box_id": box_id})
    measurements = list(mongo_client.measurements_collection.find({"box_id": box_id,
                                                                   "timestamp_ms": {"$gte": start_timestamp_ms,
                                                                                    "$lte": end_timestamp_ms}},
                                                                  {"_id": False,
                                                                   "expireAt": False}))

    event_adc_samples = get_waveform(mongo_client, box_event["data_fs_filename"])
    event_start_timestamp_ms = box_event["event_start_timestamp_ms"]
    delta_start_ms = start_timestamp_ms - event_start_timestamp_ms
    delta_end_ms = end_timestamp_ms - event_start_timestamp_ms
    # Provide a ms worth of buffer to account for floating point error?
    incident_start_idx = int(max(0, round(analysis.ms_to_samples(delta_start_ms)) - constants.SAMPLES_PER_MILLISECOND))
    incident_end_idx = int(min(len(event_adc_samples),
                               round(analysis.ms_to_samples(delta_end_ms)) + constants.SAMPLES_PER_MILLISECOND))
    incident_adc_samples_bytes = event_adc_samples[incident_start_idx:incident_end_idx].astype(numpy.int16).tobytes()

    ieee_duration = get_ieee_duration(end_timestamp_ms - start_timestamp_ms).value
    location = get_location(box_id, mongo_client)

    incident_id = next_available_incident_id(mongo_client)
    gridfs_filename = "incident_{}".format(incident_id)
    mongo_client.write_incident_waveform(incident_id, gridfs_filename, incident_adc_samples_bytes)

    incident = {
        "incident_id": incident_id,
        "box_id": box_id,
        "start_timestamp_ms": start_timestamp_ms,
        "end_timestamp_ms": end_timestamp_ms,
        "location": location,
        "measurement_type": measurement_type.value,
        "deviation_from_nominal": deviation_from_nominal,
        "measurements": measurements,
        "gridfs_filename": gridfs_filename,
        "classifications": list(map(lambda e: e.value, classifications)),
        "ieee_duration": ieee_duration,
        "annotations": annotations,
        "metadata": metadata
    }

    mongo_client.incidents_collection.insert_one(incident)


def get_box_event(event_id: int, box_id: str, opq_mongo_client: OpqMongoClient = None) -> typing.Dict:
    """
    Returns the box_event associated with the provided event_id and box_id.
    :param event_id: The event_id associated with this box_event.
    :param box_id: The box_id associated with this box_event.
    :param opq_mongo_client: An optional mongo client to use for DB access (will be created if not-provided)
    :return: The box event associated with the provided event and box ids.
    """
    mongo_client = get_default_client(opq_mongo_client)
    return mongo_client.box_events_collection.find_one({"event_id": event_id,
                                                        "box_id": box_id})


def get_location(box_id: str, opq_mongo_client: OpqMongoClient) -> str:
    """
    Returns the current location of a box, or "UNKNOWN" if the location DNE.
    :param box_id: The box to lookup the location for.
    :param opq_mongo_client: An optional mongo client to use for DB access (will be created if not-provided)
    :return: The location slug for the provided box
    """
    unknown = "UNKNOWN"
    mongo_client = get_default_client(opq_mongo_client)
    box = mongo_client.opq_boxes_collection.find_one({"box_id": box_id})

    if box is None:
        return unknown

    if "location" not in box:
        return unknown

    return box["location"]


def get_calibration_constant(box_id: int) -> float:
    """
    Return the calibration constant for a specified box id.
    :param box_id: The box id to query
    :return: The calibration constant or 1.0 if the constant can't be found
    """
    calibration_constants = get_box_calibration_constants()
    if box_id in calibration_constants:
        return calibration_constants[box_id]

    return 1.0


def memoize(single_param_fn: typing.Callable) -> typing.Callable:
    """
    Memoizes function returns.
    :param single_param_fn: The function to memoize.
    :return: A memoized version of callable
    """
    cache = {}

    def helper(key):
        """
        Closure that closes over cache. If value is not in cache, callable is ran to produce the value.
        :param key: Value to check if in cache
        :return: Value from fn call.
        """
        if key in cache:
            return cache[key]

        cache[key] = single_param_fn(key)
        return cache[key]

    return helper


# pylint: disable=C0103
cached_calibration_constant = memoize(get_calibration_constant)


def object_id(oid: str) -> bson.objectid.ObjectId:
    """Given the string representation of an object an id, return an instance of an ObjectID

    :param oid: The oid to encode
    :return: ObjectId from string
    """
    return bson.objectid.ObjectId(oid)
