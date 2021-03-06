import React from 'react';
import { Meteor } from 'meteor/meteor';
import { withRouter, Link } from 'react-router-dom';
import { Loader, Table, Button, Container } from 'semantic-ui-react';
import { Locations } from '/imports/api/locations/LocationsCollection';
import { BoxOwners } from '/imports/api/users/BoxOwnersCollection';
import { OpqBoxes } from '/imports/api/opq-boxes/OpqBoxesCollection';
import { withTracker } from 'meteor/react-meteor-data';
import PropTypes from 'prop-types';
import WidgetPanel from '/imports/ui/layouts/WidgetPanel';

/** Renders a table containing all of the OPQBox documents. */
class ManageBoxPage extends React.Component {

  helpText = `
  <p>Lists all currently defined OPQ Boxes</p>
  `;

  /** If the subscription(s) have been received, render the page, otherwise show a loading icon. */
  render() {
    return (this.props.ready) ? this.renderPage() : <Loader active>Getting data</Loader>;
  }

  /** Render the page once subscriptions have been received. */
  renderPage() { // eslint-disable-line class-methods-use-this
    const boxIds = OpqBoxes.findBoxIds(true);
    const boxes = boxIds.map(id => OpqBoxes.findBox(id));
    return (
        <Container>
          <WidgetPanel title="Manage OPQ Boxes" helpText={this.helpText} noPadding>
            <Table>
              <Table.Header>
                <Table.Row>
                  <Table.HeaderCell>ID</Table.HeaderCell>
                  <Table.HeaderCell>Name</Table.HeaderCell>
                  <Table.HeaderCell>Location</Table.HeaderCell>
                  <Table.HeaderCell>Info</Table.HeaderCell>
                  <Table.HeaderCell></Table.HeaderCell>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {boxes.map((box, index) => <Table.Row key={index}>
                  <Table.Cell>{box.box_id}</Table.Cell>
                  <Table.Cell>{box.name}</Table.Cell>
                  <Table.Cell>{Locations.getDoc(box.location).description}</Table.Cell>
                  <Table.Cell>{this.getBoxInfoString(box)}</Table.Cell>
                  <Table.Cell>
                    <Button size='tiny' as={Link} to={`/admin/manage/opqbox/edit/${box.box_id}`}>Edit</Button>
                  </Table.Cell>
                </Table.Row>)}
              </Table.Body>
              <Table.Footer fullWidth>
                <Table.Row>
                  <Table.HeaderCell colSpan='5'>
                    <Button size='tiny' as={Link} to={'/admin/manage/opqbox/new'}>Add OPQ Box</Button>
                  </Table.HeaderCell>
                </Table.Row>
              </Table.Footer>
            </Table>
          </WidgetPanel>
        </Container>
    );
  }

  getBoxInfoString(box) {
    return `Description: ${box.description}, Calibration: ${box.calibration_constant}, Unplugged: ${box.unplugged}`;
  }

}

/** Require the ready flag. */
ManageBoxPage.propTypes = {
  ready: PropTypes.bool.isRequired,
};

/** withTracker connects Meteor data to React components. https://guide.meteor.com/react.html#using-withTracker */
export default withTracker(() => {
  // Get access to Stuff documents.
  const sub1 = Meteor.subscribe(Locations.getPublicationName());
  const sub2 = Meteor.subscribe(BoxOwners.getPublicationName());
  const sub3 = Meteor.subscribe(OpqBoxes.getPublicationName());
  return {
    ready: sub1.ready() && sub2.ready() && sub3.ready(),
  };
})(withRouter(ManageBoxPage));
