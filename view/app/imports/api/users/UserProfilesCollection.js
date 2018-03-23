import { Meteor } from 'meteor/meteor';
import { Accounts } from 'meteor/accounts-base';
import { Roles } from 'meteor/alanning:roles';
import { SimpleSchema } from 'meteor/aldeed:simple-schema';
import BaseCollection from '../base/BaseCollection';

/**
 * User Profiles (first and last name, role, and username (email).
 * To create a new User, call UserProfiles.define(), which both defines the profile and creates the Meteor.user.
 * Docs: https://open-power-quality.gitbooks.io/open-power-quality-manual/content/datamodel/description.html#users
 */
class UserProfilesCollection extends BaseCollection {

  /**
   * Creates the User Profiles collection.
   */
  constructor() {
    super('UserProfiles', new SimpleSchema({
      username: String,
      firstName: String,
      lastName: String,
      role: String,
    }));
  }

  /**
   * Defines a new UserProfile if username is not defined in Meteor.users, and adds the user to Meteor.users.
   * Updates an existing User Profile if username is already defined in Meteor.users.
   * @param {String} username - Must be the user's email address.
   * @param {String} password - The user's password. (Only used when adding a user to Meteor.users.)
   * @param {String} firstName - The user's first name.
   * @param {String} lastName - The user's last name.
   * @param {String} role - The role of the user: either 'admin' or 'user'.
   * @param {[Number]} boxIds - An array of integer IDs of the boxes that can be managed by this user.
   */
  // eslint-disable-next-line class-methods-use-this
  define({ username, password, firstName, lastName, boxIds = [], role = 'user' }) {
    if (Meteor.isServer) {
      // boxIds array must contain integers.
      boxIds.forEach(boxId => {
        if (!_.isInteger(boxId)) {
          throw new Meteor.Error(`All boxIds must be a number: ${boxId}`);
        }
      });

      // Role must be either 'user' or 'admin'.
      if (role !== 'user' && role !== 'admin') {
        throw new Meteor.Error('Invalid user role - must either be "user" or "admin"');
      }

      // Figure out if the user is already defined.
      let user = Accounts.findUserByUsername(username);
      let userId = user && user._id;
      // Define the new user if they don't already exist.
      if (!userId) {
        userId = Accounts.createUser({ username, email: username, password });
        user = Accounts.findUserByUsername(username);
      }

      // Create or modify the UserProfiles document associated with this username.
      const profileId = this._collection.upsert({ username }, { $set: { firstName, lastName, role } });

      // Set the role using the Roles package. This makes it easier to do Role-based decisions on client.
      if (userId) {
        Roles.addUsersToRoles(userId, role);
      }
      // Return the profileID if executed on the server.
      return profileId;
    }
    // Return undefined if executed on the client (which shouldn't ever happen.)
    return undefined;
  }

  /**
   * Returns an object representing a single UserProfile.
   * @param {Object} docID - The Mongo.ObjectID of the User.
   * @returns {Object} - An object representing a single UserProfile.
   */
  dumpOne(docID) {
    const doc = this.findDoc(docID);
    const username = doc.username;
    const firstName = doc.firstName;
    const lastName = doc.lastName;
    const role = doc.roles;
    return { username, firstName, lastName, role };
  }
}

/**
 * Provides the singleton instance of this class.
 * @type {UserProfilesCollection}
 */
export const UserProfiles = new UserProfilesCollection();
