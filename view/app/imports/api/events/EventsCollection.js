import { Meteor } from 'meteor/meteor';
import { check, Match } from 'meteor/check';
import SimpleSchema from 'simpl-schema';
import moment from 'moment/moment';
import BaseCollection from '../base/BaseCollection.js';
import { OpqBoxes } from '../opq-boxes/OpqBoxesCollection';
import { BoxEvents } from '../box-events/BoxEventsCollection';
import { FSFiles } from '../fs-files/FSFilesCollection';
import { FSChunks } from '../fs-chunks/FSChunksCollection';

/**
 * The Events collection stores abnormal PQ data detected by the system.
 * @see {@link https://openpowerquality.org/docs/cloud-datamodel.html#events}
 */
class EventsCollection extends BaseCollection {

  constructor() {
    super('events', new SimpleSchema({
      event_id: Number,
      type: String,
      description: String,
      boxes_triggered: [String],
      boxes_received: [String],
      target_event_start_timestamp_ms: Number,
      target_event_end_timestamp_ms: Number,
      latencies_ms: { type: Array, optional: true },
      'latencies_ms.$': Number,
    }));

    // Event Type classifications will eventually move to the Incident level.
    this.FREQUENCY_SAG_TYPE = 'FREQUENCY_SAG';
    this.FREQUENCY_SWELL_TYPE = 'FREQUENCY_SWELL';
    this.VOLTAGE_SAG_TYPE = 'VOLTAGE_SAG';
    this.VOLTAGE_SWELL_TYPE = 'VOLTAGE_SWELL';
    this.THD_TYPE = 'THD';
    this.OTHER_TYPE = 'OTHER';
    this.eventTypes = [this.FREQUENCY_SAG_TYPE, this.FREQUENCY_SWELL_TYPE, this.VOLTAGE_SAG_TYPE,
      this.VOLTAGE_SWELL_TYPE, this.THD_TYPE, this.OTHER_TYPE];

    this.publicationNames = {
      GET_EVENTS: 'get_events',
      GET_RECENT_EVENTS: 'get_recent_events',
    };

    // if (Meteor.server) {
    //   this._collection.rawCollection().createIndex({ target_event_start_timestamp_ms: 1 }, { background: true });
    //   this._collection.rawCollection().createIndex({ event_id: 1 }, { background: true, unique: true });
    // }
  }

  /**
   * Defines a new Event document.
   * @param {Number} event_id - The event's id value (not Mongo ID)
   * @param {String} type - The unix timestamp (millis) of the measurement. (Deprecated, now optional).
   * @param {String} description - The description of the event.
   * @param {String} boxes_triggered - The OPQBoxes from which data was requested for this event.
   * @param {String} boxes_received - The OPQBoxes from which data was received for this event.
   * @param {String} target_event_start_timestamp_ms - The time this event started. Anything acceptable to moment().
   * @param {String} target_event_end_timestamp_ms - The time this event ended. Anything acceptable to moment().
   * @param {Number} latencies_ms - Array of unix timestamps for the event. See docs for details.
   * @returns The newly created document ID.
   */
  define({ event_id, type, description, boxes_triggered, boxes_received, target_event_start_timestamp_ms,
    target_event_end_timestamp_ms, latencies_ms }) {
    // Allow start and end times to be anything acceptable to the moment() parser.
    target_event_start_timestamp_ms = moment(target_event_start_timestamp_ms).valueOf(); // eslint-disable-line
    target_event_end_timestamp_ms = moment(target_event_end_timestamp_ms).valueOf(); // eslint-disable-line
    // Make sure boxes_triggered contains valid box_ids.
    OpqBoxes.assertValidBoxIds(boxes_triggered);
    const docID = this._collection.insert({ event_id, type, description, boxes_triggered, boxes_received,
      target_event_start_timestamp_ms, target_event_end_timestamp_ms, latencies_ms });
    return docID;
  }

  /**
   * Returns truthy (i.e. the document) if event_id is a defined event_id, falsey otherwise.
   * @param event_id An event_id (a number)
   * @returns The document, or null.
   */
  isEventId(event_id) {
    return this._collection.findOne({ event_id });
  }

  /**
   * Defines all publications related to this collection.
   */
  publish() {
    if (Meteor.isServer) {
      const self = this;

      Meteor.publish(this.publicationNames.GET_EVENTS, function ({ startTime, endTime }) {
        check(startTime, Match.Maybe(Number));
        check(endTime, Match.Maybe(Number));
        const selector = self.queryConstructors().getEvents({ startTime, endTime });
        return self.find(selector);
      });

      Meteor.publish(this.publicationNames.GET_RECENT_EVENTS, function ({ numEvents, excludeOther }) {
        check(numEvents, Number);
        const query = excludeOther ? { type: { $ne: 'OTHER' } } : {};
        const events = self.find(query, { sort: { target_event_start_timestamp_ms: -1 }, limit: numEvents });
        return events;
      });
    }
  }

  /**
   * Supports the GET_EVENTS publication method.
   * @returns { Object } An object formatted for the publication optional object.
   */
  queryConstructors() { // eslint-disable-line class-methods-use-this
    return {
      getEvents({ startTime, endTime }) {
        check(startTime, Match.Maybe(Number));
        check(endTime, Match.Maybe(Number));

        const selector = {};
        if (startTime) selector.target_event_start_timestamp_ms = { $gte: startTime };
        if (endTime) selector.target_event_end_timestamp_ms = { $lte: endTime };

        return selector;
      },
    };
  }

  /**
   * Checks the integrity of the passed Event document. Integrity checking means:
   *   * Is the event_id a number?
   *   * Do boxes_triggered and boxes_received contain valid boxIDs?
   *   * Are the timestamps reasonable UTC millisecond values?
   * @param doc The event document.
   * @param repair If repair is true, and an integrity problem is discovered, then the Event and its corresponding
   * Box_Events, FS.Files, and FS.Chunks are all deleted.
   * @returns {Array} An array of strings describing any problems that were found.
   */
  checkIntegrity(doc, repair) {
    const problems = [];
    if (!_.isNumber(doc.event_id)) {
      problems.push(`event_id ${doc.event_id} (non-number)`);
    }
    if (!OpqBoxes.areBoxIds(doc.boxes_triggered)) {
      problems.push(`boxes_triggered ${doc.boxes_triggered} (non-box)`);
    }
    if (!OpqBoxes.areBoxIds(doc.boxes_received)) {
      problems.push(`boxes_received ${doc.boxes_received} (non-box)`);
    }
    if (!this.isValidTimestamp(doc.target_event_start_timestamp_ms)) {
      problems.push(`target_event_start_timestamp_ms ${doc.target_event_start_timestamp_ms} (invalid)`);
    }
    if (!this.isValidTimestamp(doc.target_event_end_timestamp_ms)) {
      problems.push(`target_event_end_timestamp_ms ${doc.target_event_end_timestamp_ms} (invalid)`);
    }
    if (doc.description === 'Health check') {
      problems.push(`description: ${doc.description} (false event)`);
    }
    const result = { docName: `Event ${doc.event_id}`, problems };
    if ((problems.length > 0) && repair) {
        result.repair = this.repair(doc);
      }
    return result;
  }

  /**
   * Repair (i.e. delete) an Event. This means also deleting the associated box_events, fs.files, and fs.chunks.
   * @param doc The event document.
   * @return A string indicating how many box_events, fs.files, and fs.chunks were deleted.
   */
  repair(doc) {
    const event_id = doc.event_id;
    const boxEventDocs = BoxEvents.find({ event_id }).fetch();
    const boxEventIDs = _.pluck(boxEventDocs, '_id');
    const fsFilenames = _.pluck(boxEventDocs, 'data_fs_filename');
    const fsFileDocs = FSFiles.find({ filename: { $in: fsFilenames } }).fetch();
    const fsFileIDs = _.pluck(fsFileDocs, '_id');
    const fsFileChunkDocs = FSChunks.find({ files_id: { $in: fsFileIDs } }).fetch();
    const fsChunkIDs = _.pluck(fsFileChunkDocs, '_id');
    const returnString = `Deleting event_id: ${event_id} as well as ${boxEventDocs.length} BoxEvents, ${fsFileDocs.length} FSFiles, and ${fsFileChunkDocs.length} FSChunks`;  //eslint-disable-line

    BoxEvents._collection.remove({ _id: { $in: boxEventIDs } });
    FSFiles._collection.remove({ _id: { $in: fsFileIDs } });
    FSChunks._collection.remove({ _id: { $in: fsChunkIDs } });
    this._collection.remove({ event_id });
    return returnString;
  }
}

/**
 * Provides the singleton instance of this class.
 * @type {EventsCollection}
 */
export const Events = new EventsCollection();
