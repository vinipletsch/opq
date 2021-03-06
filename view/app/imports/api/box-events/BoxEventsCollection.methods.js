import { Meteor } from 'meteor/meteor';
import { ValidatedMethod } from 'meteor/mdg:validated-method';
import SimpleSchema from 'simpl-schema';
import { BoxEvents } from './BoxEventsCollection.js';

export const totalBoxEventsCount = new ValidatedMethod({
  name: 'BoxEvents.totalBoxEventsCount',
  validate: new SimpleSchema().validator({ clean: true }),
  run() {
    return BoxEvents.find({}).count();
  },
});

export const getBoxEvent = new ValidatedMethod({
  name: 'BoxEvents.getBoxEvent',
  validate: new SimpleSchema({
    event_id: { type: Number },
    box_id: { type: String },
  }).validator({ clean: true }),
  run({ event_id, box_id }) {
    if (Meteor.isServer) {
      const boxEvent = BoxEvents.findOne({ event_id, box_id }, {});
      if (!boxEvent) {
        throw new Meteor.Error(`BoxEvents document not found for event_number: ${event_id}, box_id: ${box_id}`);
      }
      return boxEvent;
    }
    return null;
  },
});
