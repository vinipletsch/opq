//
// Created by tusk on 8/23/17.
//

#include "MongoDriver.h"
#include "util.h"
#include <mongocxx/instance.hpp>
#include <mongocxx/exception/write_exception.hpp>
#include <mongocxx/exception/query_exception.hpp>
#include <bsoncxx/builder/stream/document.hpp>
#include <bsoncxx/document/element.hpp>
#include <bsoncxx/json.hpp>

using bsoncxx::builder::stream::close_array;
using bsoncxx::builder::stream::close_document;
using bsoncxx::builder::stream::document;
using bsoncxx::builder::stream::finalize;
using bsoncxx::builder::stream::open_array;
using bsoncxx::builder::stream::open_document;


static const std::string OPQ_DB = "opq";
static const std::string OPQ_EVENT_COLLECTION = "events";
static const std::string OPQ_DATA_COLLECTION = "box_events";
static const std::string OPQ_BOX_COLLECTION = "opq_boxes";

static const std::string BOX_ID_FIELD = "box_id";
static const std::string EVENT_NUMBER_FIELD = "event_id";
static const std::string EVENT_TYPE_FIELD = "type";
static const std::string DESCRIPTION_FIELD = "description";
static const std::string BOXES_TRIGGERED_FIELD = "boxes_triggered";
static const std::string BOXES_RECEIVED_FIELD = "boxes_received";
static const std::string EVENT_START_FIELD = "target_event_start_timestamp_ms";
static const std::string EVENT_END_FIELD = "target_event_end_timestamp_ms";

static const std::string BOX_EVENT_START_FIELD = "event_start_timestamp_ms";
static const std::string BOX_EVENT_END_FIELD = "event_end_timestamp_ms";

static const std::string TIME_STAMP_FIELD = "latencies_ms";
static const std::string WINDOW_TIME_STAMP_FIELD = "window_timestamp_ms";
static const std::string TIME_DATA_FIELD = "data_fs_filename";

static const std::string LOCATION_OPQ_BOXES_FIELD = "locations";
static const std::string LOCATION_BOX_EVENT_FIELD = "location";

MongoDriver::MongoDriver(std::string uri) : _client(mongocxx::uri(uri)){
    _db = _client[OPQ_DB];
    _event_collection = _db[OPQ_EVENT_COLLECTION];
    _data_collection = _db[OPQ_DATA_COLLECTION];
    _box_collection = _db[OPQ_BOX_COLLECTION];
    _bucket = _db.gridfs_bucket();
}

bool MongoDriver::init_mongo_client() {
    static mongocxx::instance instance{};
}

uint32_t MongoDriver::get_next_event_number() {
    mongocxx::options::find opts;
    opts.limit( 1 );
    opts.sort(document{} << EVENT_NUMBER_FIELD << -1 << finalize);
    auto result = _event_collection.find_one(document{}.view(), opts);
    if (!result) return 1;
    bsoncxx::document::element element= result->view()[EVENT_NUMBER_FIELD];
    if (!element) return 1;
    int32_t next_event_number = element.get_int32() + 1;
    if(next_event_number < 0) return 1;
    return next_event_number;

}

bool MongoDriver::create_event(opq::proto::RequestEventMessage &m, uint64_t ts, uint32_t event_num) {
    auto builder = document{};
    builder << EVENT_TYPE_FIELD << opq::proto::RequestEventMessage_TriggerType_Name(m.trigger_type())
            << EVENT_NUMBER_FIELD << (int32_t)event_num
            << DESCRIPTION_FIELD << m.description();

    auto box_array = builder << BOXES_TRIGGERED_FIELD << open_array;
    std::for_each(m.box_ids().begin(), m.box_ids().end(),
                  [&box_array](int box){box_array << std::to_string(box);});

    box_array << close_array;
    builder << BOXES_RECEIVED_FIELD << open_array << close_array;
    builder << TIME_STAMP_FIELD << open_array << close_array;
    builder << EVENT_START_FIELD << (int64_t)m.start_timestamp_ms_utc()
            << EVENT_END_FIELD << (int64_t)m.end_timestamp_ms_utc();
    bsoncxx::document::value doc_value = builder << finalize;
    try {
        auto result = _event_collection.insert_one(doc_value.view());
    }
    catch(const mongocxx::write_exception &e){
        std::cout << e.what() << std::endl;
    }
}

bool MongoDriver::append_data_to_event(std::vector<opq::proto::DataMessage> &messages, uint32_t event_num) {
    if (messages.size() == 0)
        return false;
    int32_t id = messages.front().id();

    try {
        _event_collection.update_one(document{}
                                             << EVENT_NUMBER_FIELD << (int32_t) event_num
                                             << finalize,
                                     document{}
                                             << "$push"
                                             << open_document
                                             << BOXES_RECEIVED_FIELD << id
                                             << TIME_STAMP_FIELD << (int64_t) chrono_to_mili_now()
                                             << close_document
                                             << finalize);
    }
    catch (const mongocxx::write_exception &e) {
        std::cout << e.what() << std::endl;
    }

    string data_file = "event_" + std::to_string(event_num) + "_" + std::to_string(id);

    auto builder = bsoncxx::builder::stream::document{};
    auto start_time = messages.front().cycles().Get(0).time();
    auto cycle_size = (size_t) messages.front().cycles().size();
    auto end_time = messages.front().cycles().Get(cycle_size - 1).time();
    builder << BOX_ID_FIELD << std::to_string(id)
            << BOX_EVENT_START_FIELD << (int64_t) start_time
            << BOX_EVENT_END_FIELD << (int64_t) end_time
            << EVENT_NUMBER_FIELD << (int32_t) event_num;
    auto time_array_context = builder << WINDOW_TIME_STAMP_FIELD << bsoncxx::builder::stream::open_array;
    for (auto &message : messages) {
        for (auto &cycle : message.cycles()) {
            time_array_context << (int64_t) cycle.time();
        }
    }
    time_array_context << close_array;
    builder << TIME_DATA_FIELD << data_file;

    //Fill in location.
    try {
        mongocxx::options::find opts{};
        opts.projection(document{} << LOCATION_OPQ_BOXES_FIELD << open_document << "$slice" << -1 << close_document << finalize);

        auto locations = _box_collection.find_one(document{} << BOX_ID_FIELD << std::to_string(id) << finalize,
                                                 opts
        );
        using namespace std;
        if(locations){
            auto element = (*locations).view()[LOCATION_OPQ_BOXES_FIELD][0];
            if(element){
                builder << LOCATION_BOX_EVENT_FIELD << element.get_document();
            }
        }
    }
    catch (const mongocxx::query_exception &e) {
        std::cout << e.what() << std::endl;
    }

    bsoncxx::document::value doc_value = builder << finalize;

    try {
        auto result = _data_collection.insert_one(doc_value.view());
    }
    catch (const mongocxx::write_exception &e) {
        std::cout << e.what() << std::endl;
    }

    auto uploader = _bucket.open_upload_stream(data_file);

    std::vector<uint8_t> file_data;
    for (auto &message : messages) {
        for (auto &cycle : message.cycles()) {
            std::for_each(cycle.data().begin(), cycle.data().end(),
                          [&file_data](auto sample) {
                              file_data.push_back(sample | 0xFF);
                              file_data.push_back(sample >> 8);
                          }
            );
            //file_data.insert(file_data.end(),cycle.data().begin(), cycle.data().end());
        }
    }
    uploader.write(file_data.data(), file_data.size());
    auto result = uploader.close();
}