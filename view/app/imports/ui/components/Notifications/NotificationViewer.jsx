import React from 'react';
import { Meteor } from 'meteor/meteor';
import { Feed, Icon, Popup, Label, Loader } from 'semantic-ui-react';
import PropTypes from 'prop-types';
import { withTracker } from 'meteor/react-meteor-data';
import { HashLink as Link } from 'react-router-hash-link';
import { updateMethod } from '/imports/api/base/BaseCollection.methods';
import { Notifications } from '../../../api/notifications/NotificationsCollection';
import { UserProfiles } from '../../../api/users/UserProfilesCollection';
import './notificationStyle.css';

class NotificationViewer extends React.Component {

  /** If the subscription(s) have been received, render the page, otherwise show a loading icon. */
  render() {
    return (this.props.ready) ? this.renderPage() : <Loader active>Getting data</Loader>;
  }

  /**
   * When notification icon is clicked set the user's unseen_notification field to false
   * newNotification state also set to false so that red label disappears immediately
   */
  notificationSeen = () => {
    const collectionName = UserProfiles.getCollectionName();
    const id = this.props.user._id;
    const updateData = { id, unseen_notifications: 'false' };
    updateMethod.call({ collectionName, updateData });
  };

  renderBellIcon() {
    let newNotifications = false;
    if (this.props.user !== undefined) {
      newNotifications = this.props.user.unseen_notifications;
    }
    return (
        <Icon.Group>
          <Icon name='bell' style={this.props.divStyle} onClick={this.notificationSeen}/>
          {newNotifications ? (
              <Icon corner name='circle' color='red'/>
          ) : ''}
        </Icon.Group>
    );
  }

  /** Renders a popup that displays all of the user's notifications */
  renderPage() {
    return (
        <Popup trigger={this.renderBellIcon()} on='click' basic position='bottom left' id='popupStyle'>
          <Popup.Header id='popupHeader'>
            Notifications
          </Popup.Header>
          <Feed>
            {(this.props.userNotifications).length > 0 ? (this.props.userNotifications.map((notification) =>
                    <Feed.Event key={notification._id}
                                icon='exclamation circle'
                                meta={notification.timestamp.toLocaleString()}
                                summary={notification.data.summary}
                    />)) :
                // if there are no notifications display this message
                <Feed.Extra>
                  <Icon name='bell slash outline' size='big'/>
                  <p><br/>No new notifications yet</p>
                </Feed.Extra>
            }
          </Feed>
          <Label attached='bottom' size='small'>
            <Link smooth to="/profile#ntf-settings"><Icon name='setting'/>Manage Notifications</Link>
          </Label>
        </Popup>
    );
  }
}

NotificationViewer.propTypes = {
  ready: PropTypes.bool.isRequired,
  userNotifications: PropTypes.array,
  divStyle: PropTypes.object,
  user: PropTypes.object,
};

export default withTracker(() => {
  const notificationsSub = Meteor.subscribe(Notifications.getPublicationName());
  const userProfilesSub = Meteor.subscribe(UserProfiles.getPublicationName());
  const username = Meteor.user().username;
  const user = UserProfiles.findOne({ username });
  return {
    ready: notificationsSub.ready() && userProfilesSub.ready(),
    userNotifications: Notifications.findNotificationsByUser(username),
    user: user,
  };
})(NotificationViewer);
