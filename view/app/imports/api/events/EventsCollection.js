import { Meteor } from 'meteor/meteor';
import { check, Match } from 'meteor/check';
import { SimpleSchema } from 'meteor/aldeed:simple-schema';
import BaseCollection from '../base/BaseCollection.js';
import { progressBarSetup } from '../../modules/utils';

/**
 * Collection class for the events collection.
 * Docs: https://open-power-quality.gitbooks.io/open-power-quality-manual/content/datamodel/description.html#events
 */
class EventsCollection extends BaseCollection {

  /**
   * Creates the collection.
   */
  constructor() {
    super('events', new SimpleSchema({
      event_id: { type: Number },
      type: { type: String },
      description: { type: String },
      boxes_triggered: { type: [Number] }, // List of box_id's
      latencies_ms: { type: [Number] },
    }));

    this.publicationNames = {
      GET_EVENT_META_DATA: 'get_event_meta_data',
    };
  }

  /**
   * Defines a new Event document.
   * @param {Number} event_id - The event's id value (not Mongo ID)
   * @param {String} type - The unix timestamp (millis) of the measurement.
   * @param {String} description - The description of the event.
   * @param {Number} boxes_triggered - The OPQBoxes from which data was requested for this event.
   * @param {Number} latencies - Array of unix timestamps for the event. See docs for details.
   * @returns The newly created document ID.
   */
  define({ event_id, type, description, boxes_triggered, latencies }) {
    const docID = this._collection.insert({ event_id, type, description, boxes_triggered, latencies });
    return docID;
  }

  /**
   * Returns an object representing a single Event.
   * @param {Object} docID - The Mongo.ObjectID of the Event.
   * @returns {Object} - An object representing a single Event.
   */
  dumpOne(docID) {
    /* eslint-disable camelcase */
    const doc = this.findDoc(docID);
    const event_id = doc.event_id;
    const type = doc.type;
    const description = doc.description;
    const boxes_triggered = doc.boxes_triggered;
    const latencies = doc.latencies;

    return { event_id, type, description, boxes_triggered, latencies };
    /* eslint-enable camelcase */
  }

  checkIntegrity() {
    const problems = [];
    const totalCount = this.count();
    const validationContext = this.getSchema().namedContext('eventsIntegrity');
    const pb = progressBarSetup(totalCount, 2000, `Checking ${this._collectionName} collection: `);

    this.find().forEach((doc, index) => {
      pb.updateBar(index); // Update progress bar.

      // Validate each document against the collection schema.
      validationContext.validate(doc);
      if (!validationContext.isValid) {
        // eslint-disable-next-line max-len
        problems.push(`Events document failed schema validation: ${doc._id} (Invalid keys: ${validationContext.invalidKeys()})`);
      }
      validationContext.resetValidation();
    });

    pb.clearInterval();
    return problems;
  }


  /**
   * Loads all publications related to this collection.
   * Note: We conditionally import the publications file only on the server as a way to hide publication code from
   * being sent to the client.
   */
  publish() {
    if (Meteor.isServer) {
      Meteor.publish(this.publicationNames.GET_EVENT_META_DATA, function ({ startTime, endTime }) {
        check(startTime, Match.Maybe(Number));
        check(endTime, Match.Maybe(Number));

        const selector = this.queryConstructors().getEventMetaData({ startTime, endTime });
        return this.find(selector, {});
      });
    }
  }

  queryConstructors() { // eslint-disable-line class-methods-use-this
    return {
      getEventMetaData({ startTime, endTime }) {
        check(startTime, Match.Maybe(Number));
        check(endTime, Match.Maybe(Number));

        const selector = {};
        if (startTime) selector.event_start = { $gte: startTime };
        if (endTime) selector.event_end = { $lte: endTime };

        return selector;
      } };
  }
}

/**
 * Provides the singleton instance of this class.
 * @type {EventsCollection}
 */
export const Events = new EventsCollection();