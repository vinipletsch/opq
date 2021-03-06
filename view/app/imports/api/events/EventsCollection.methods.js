import { Meteor } from 'meteor/meteor';
import { ValidatedMethod } from 'meteor/mdg:validated-method';
import { CallPromiseMixin } from 'meteor/didericis:callpromise-mixin';
import SimpleSchema from 'simpl-schema';
import { Events } from './EventsCollection.js';

/** Returns an array of events that were detected by specified boxes, in a specified range.
 * @param {String[]} boxIDs: List of box IDs to get data for
 * @param {Number} startDate_ms: Start of range in Unix epoch time
 * @param {Number} endDate_ms: End of range in Unix epoch time
 */
export const getEventsInRange = new ValidatedMethod({
  name: 'Events.getEventsInRange',
  mixins: [CallPromiseMixin],
  validate: new SimpleSchema({
    boxIDs: { type: Array },
    'boxIDs.$': { type: String },
    startTime_ms: { type: Number },
    endTime_ms: { type: Number },
  }).validator({ clean: true }),
  run({ boxIDs, startTime_ms, endTime_ms }) {
    if (Meteor.isServer) {
      return Events.find({
        boxes_triggered: { $in: boxIDs },
        target_event_start_timestamp_ms: { $gte: startTime_ms },
        target_event_end_timestamp_ms: { $lte: endTime_ms },
      }).fetch();
    }
    return null;
  },
});

// jun12018: 1527890400000
// jan12017: 1483308000000
