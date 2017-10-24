import numpy

import mongo.mongo


def to_s16bit(data):
    return numpy.frombuffer(data, numpy.int16)


def get_data(mongo_client, event_data):
    if "data" in event_data:
        data = mongo_client.read_file(event_data["data"])
        event_data["data"] = to_s16bit(data)
    else:
        print("No data pointer in event metadata")


def load_event(event_id: int, mongo_client=None):
    _mongo_client = mongo_client if mongo_client is not None else mongo.mongo.OpqMongoClient()

    events_collection = _mongo_client.get_collection(mongo.mongo.Collection.EVENTS)
    event = events_collection.find_one({"event_number": event_id})

    data_collection = _mongo_client.get_collection(mongo.mongo.Collection.DATA)
    event_data = list(data_collection.find({"event_number": event_id}))

    for d in event_data:
        get_data(_mongo_client, d)

    return {"event": event,
            "event_data": event_data}
