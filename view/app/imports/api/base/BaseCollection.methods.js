import { CallPromiseMixin } from 'meteor/didericis:callpromise-mixin';
import { ValidatedMethod } from 'meteor/mdg:validated-method';
import { OPQ } from '../opq/Opq';

/**
 * Meteor method used to define new instances of the given collection name.
 * @param collectionName the name of the collection.
 * @param definitionDate the object used in the collection.define method.
 * @memberOf api/base
 */
export const defineMethod = new ValidatedMethod({
  name: 'BaseCollection.define',
  mixins: [CallPromiseMixin],
  validate: null,
  run({ collectionName, definitionData }) {
    const collection = OPQ.getCollection(collectionName);
    collection.assertValidRoleForMethod(this.userId);
    return collection.define(definitionData);
  },
});

export const updateMethod = new ValidatedMethod({
  name: 'BaseCollection.update',
  mixins: [CallPromiseMixin],
  validate: null,
  run({ collectionName, updateData }) {
    const collection = OPQ.getCollection(collectionName);
    collection.assertValidRoleForMethod(this.userId);
    return collection.update(updateData.id, updateData);
  },
});

export const removeMethod = new ValidatedMethod({
  name: 'BaseCollection.remove',
  mixins: [CallPromiseMixin],
  validate: null,
  run({ collectionName, docID }) {
    const collection = OPQ.getCollection(collectionName);
    collection.assertValidRoleForMethod(this.userId);
    return collection.remove(docID);
  },
});
