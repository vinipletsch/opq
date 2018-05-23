import { Meteor } from 'meteor/meteor';
import { Mongo } from 'meteor/mongo';
import { check } from 'meteor/check';
import SimpleSchema from 'simpl-schema';
import BaseCollection from '../base/BaseCollection.js';

/**
 * The measurements collection provides short-term, low-fidelity OPQBox snapshot data for a specific moment in time.
 * Measurements are automatically deleted by the system after 24 hours using a TTL attribute.
 * @see {@link https://openpowerquality.org/docs/cloud-datamodel.html#measurements}
 */
class MeasurementsCollection extends BaseCollection {

  constructor() {
    super('measurements', new SimpleSchema({
      _id: { type: Mongo.ObjectID },
      box_id: { type: String },
      timestamp_ms: { type: Number },
      voltage: { type: Number },
      frequency: { type: Number },
      thd: { type: Number, optional: true },
      expireAt: { type: Date },
    }));

    this.publicationNames = {
      RECENT_MEASUREMENTS: 'recent_measurements',
    };
    if (Meteor.server) {
      this._collection.rawCollection().createIndex({ timestamp_ms: 1, box_id: 1 }, { background: true });
    }
  }

  /**
   * Defines a new Measurement document.
   * @param {String} box_id - The OPQBox's id value (not Mongo ID)
   * @param {Number} timestamp_ms - The unix timestamp (millis) of the measurement.
   * @param {Number} voltage - The voltage measurement.
   * @param {Number} frequency - The frequency measurement.
   * @param {Number} thd - The thd measurement.
   * @returns The newly created document ID.
   */
  define({ box_id, timestamp_ms, voltage, frequency, thd }) {
    const docID = this._collection.insert({ box_id, timestamp_ms, voltage, frequency, thd });
    return docID;
  }

  /**
   * Loads all publications related to the Measurements collection.
   */
  publish() { // eslint-disable-line class-methods-use-this
    if (Meteor.isServer) {
      const self = this;
      Meteor.publish(this.publicationNames.RECENT_MEASUREMENTS, function (startTimeSecondsAgo, boxID) {
        check(startTimeSecondsAgo, Number);
        check(boxID, String);

        const instance = this;

        const startTimeMs = Date.now() - (startTimeSecondsAgo * 1000);
        // eslint-disable-next-line max-len
        const publishedMeasurementsMap = new Map(); // {timestamp: id} - To keep track of currently published measurements.

        const selector = (boxID) ? {
          box_id: boxID,
          timestamp_ms: { $gte: startTimeMs },
        } : { timestamp_ms: { $gte: startTimeMs } };

        let init = true;
        const measurementsHandle = self.find(selector, {
          fields: { _id: 1, timestamp_ms: 1, voltage: 1, frequency: 1, box_id: 1 },
          pollingIntervalMs: 1000,
        }).observeChanges({
          added: function (id, fields) {
            publishedMeasurementsMap.set(fields.timestamp_ms, id);
            instance.added('measurements', id, fields);

            if (!init) {
              const startTime = Date.now() - (startTimeSecondsAgo * 1000);
              // Note: (_id, timestamp) corresponds to (value, key); for some reason Map's foreach is called this way.
              publishedMeasurementsMap.forEach((_id, timestamp) => {
                if (timestamp < startTime) {
                  instance.removed('measurements', _id);
                  publishedMeasurementsMap.delete(timestamp);
                }
              });
            }
          },
        });
        init = false;
        instance.ready();
        instance.onStop(function () {
          measurementsHandle.stop();
        });
      });
    }
  }
}

/**
 * Provides the singleton instance of this class.
 * @type {MeasurementsCollection}
 */
export const Measurements = new MeasurementsCollection();
