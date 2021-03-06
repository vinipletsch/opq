import React from 'react';
import PropTypes from 'prop-types';
import { Meteor } from 'meteor/meteor';
import { withTracker } from 'meteor/react-meteor-data';
import { withRouter, NavLink } from 'react-router-dom';
import { Menu, Image, Dropdown } from 'semantic-ui-react';
import { Roles } from 'meteor/alanning:roles';
import NotificationViewer from './Notifications/NotificationViewer';
import { ROLE } from '../../api/opq/Role';

/* eslint max-len: 0 */
class NavBar extends React.Component {
  render() {
    const divStyle = { color: '#2185D0', paddingLeft: '2px', fontWeight: 'bold' };

    return (
        <Menu stackable borderless>
          <Menu.Item as={NavLink} exact to="/">
            <Image width="20px" src="/images/opqlogo.png"/>
            <div style={divStyle}>OPQView</div>
          </Menu.Item>

          {this.props.currentUser ? (
              <Menu.Item as={NavLink} activeClassName="active" exact to="/profile" key='profile'>
                <div style={divStyle}>Profile</div>
              </Menu.Item>
          ) : ''}

          {this.props.currentUser ? (
              <Menu.Item as={NavLink} activeClassName="active" exact to="/livedata" key='livedata'>
                <div style={divStyle}>Live Data</div>
              </Menu.Item>
          ) : ''}

          {this.props.currentUser ? (
              <Menu.Item as={NavLink} activeClassName="active" exact to="/inspector" key='inspector'>
                <div style={divStyle}>Inspector</div>
              </Menu.Item>
          ) : ''}

          {this.props.currentUser ? (
              <Menu.Item as={NavLink} activeClassName="active" exact to="/boxmap" key='boxmap'>
                <div style={divStyle}>Box Map</div>
              </Menu.Item>
          ) : ''}

        {Roles.userIsInRole(Meteor.userId(), ROLE.ADMIN) ? (
            <Menu.Item>
            <Dropdown text="Admin" pointing="top left" style={divStyle}>
              <Dropdown.Menu>
                <Dropdown.Item key="ManageBoxPage" text="Manage OPQBoxes" as={NavLink} exact to="/admin/manage/opqbox"/>
                <Dropdown.Item key="ManageLocationPage" text="Manage Locations" as={NavLink} exact to="/admin/manage/location"/>
                <Dropdown.Item key="ManageRegionPage" text="Manage Regions" as={NavLink} exact to="/admin/manage/region"/>
                <Dropdown.Item key="ManageUserPage" text="Manage Users" as={NavLink} exact to="/admin/manage/user"/>
              </Dropdown.Menu>
            </Dropdown>
            </Menu.Item>
        ) : ''}

          {this.props.currentUser ? (
              <Menu.Item>
                <NotificationViewer divStyle={divStyle}/>
              </Menu.Item>
          ) : ''}

          <Menu.Item position="right" as={NavLink} exact to="/about">
            <div style={divStyle}>About OPQ</div>
          </Menu.Item>

          <Menu.Item>
            {this.props.currentUser === '' ? (
                <Dropdown text="Login" pointing="top right" style={divStyle}>
                  <Dropdown.Menu>
                    <Dropdown.Item key="signin" icon="user" text="Sign In" as={NavLink} exact to="/signin"/>
                  </Dropdown.Menu>
                </Dropdown>
            ) : (
                <Dropdown text={this.props.currentUser} pointing="top right" style={divStyle}>
                  <Dropdown.Menu>
                    <Dropdown.Item icon="sign out" text="Sign Out" as={NavLink} exact to="/signout"/>
                  </Dropdown.Menu>
                </Dropdown>
            )}
          </Menu.Item>

        </Menu>
    );
  }
}

NavBar.propTypes = {
  currentUser: PropTypes.string,
};

const NavBarContainer = withTracker(() => ({
  currentUser: Meteor.user() ? Meteor.user().username : '',
}))(NavBar);
export default withRouter(NavBarContainer);
