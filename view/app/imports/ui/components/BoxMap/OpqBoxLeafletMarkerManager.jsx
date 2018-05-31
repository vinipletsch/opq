import { Meteor } from 'meteor/meteor';
import React from 'react';
import { withTracker } from 'meteor/react-meteor-data';
import PropTypes from 'prop-types';
import { Marker, Popup } from 'react-leaflet';
import { divIcon } from 'leaflet';
import MarkerClusterGroup from 'react-leaflet-markercluster';
import 'react-leaflet-markercluster/dist/styles.min.css';
import { Loader, List } from 'semantic-ui-react';
import { Measurements } from '../../../api/measurements/MeasurementsCollection';
import './boxMapStyle.css';

class OpqBoxLeafletMarkerManager extends React.Component {
  constructor(props) {
    super(props);

    // OpqBoxAndMarkersDict is a mapping of OpqBox (Mongo) id => {opqBox, marker, markerLeafletElement}
    // Note: We need this kind of dictionary because we require a way to retrieve an OpqBox's markerLeafletElement. Due
    // to the way the react-leaflet package implemented its Marker component, there is no way to access a Marker
    // component's internal leaflet object from outside the component. We can only get a reference to it during the
    // Marker's instantiation (see the createMarker() method).
    // The value object of this dictionary consists of the following properties:
    // opqBox: The OpqBox Mongo document
    // marker: The Marker (React) component representing the OpqBox on the Leaflet map.
    // markerLeafletElement: The Marker component's internal leaflet element object (required for updating the Marker)
    this.state = {
      opqBoxAndMarkersDict: {},
    };
  }

  componentDidMount() {
    const { opqBoxes = [] } = this.props;
    // Create initial Markers for each passed in OpqBox.
    this.createMarkers(opqBoxes);
  }

  componentDidUpdate(prevProps) {
    const { opqBoxes, currentMapLocationGranularity } = this.props;
    const { currentMapLocationGranularity: prevMapLocationGranularity } = prevProps;

    // We destroy all markers and recreate new ones whenever we need to change their positions. Currently, the only
    // event that requires us to change Marker positions is when the mapLocationGranularity option changes.
    // The main reason we do this is because React-Leaflet's Marker component doesn't expose its setLatLng() method.
    // While we could get its internal leaflet element and call setLatLng() on it, and the marker position would
    // successfully change, it turns out that this would not actually update the Marker component's 'position' props
    // value. Subsequently, whenever a re-render occurs, we end up passing 'old' Markers with incorrect position props
    // to the MCG component.
    let mustRecreateMarkers = false;
    if (currentMapLocationGranularity !== prevMapLocationGranularity) {
      mustRecreateMarkers = true;
    }

    // Delete all old Markers before recreating new ones.
    if (mustRecreateMarkers) {
      this.setState({ opqBoxAndMarkersDict: {} });
    }

    // Whenever new box measurements are received via props, we must update Markers.
    opqBoxes.forEach(box => {
      // Create new Marker if does not yet exist, or update existing marker with new data.
      if (!this.opqBoxExists(box) || mustRecreateMarkers) {
        this.createMarker(box);
      } else {
        this.updateMarker(box);
      }
    });
  }

  updateMarker(opqBox) {
    const { currentMapDataDisplay, currentMapLocationGranularity, mapLocationGranularityTypes } = this.props;
    // Retrieve box's corresponding Marker leafletElement, and update it.
    const marker = this.getMarkerLeafletElement(opqBox);
    if (marker) {
      const newestMeasurement = this.findNewestMeasurement(opqBox.box_id);
      const rawMeasurementValue = this.filterCurrentMapDataDisplay(newestMeasurement, currentMapDataDisplay, false);
      const formattedMeasurement = this.filterCurrentMapDataDisplay(newestMeasurement, currentMapDataDisplay, true);
      const newOpts = marker.options; // Don't clone this; must modify the original options obj.
      newOpts.rawValue = rawMeasurementValue;
      newOpts.formattedValue = formattedMeasurement;
      // Create appropriate html for marker icon.
      let markerHtml = '';
      if (currentMapLocationGranularity === mapLocationGranularityTypes.BOX_REGION) {
        const regionDoc = this.getOpqBoxRegionDoc(opqBox);
        const region = (regionDoc) ? regionDoc.regionSlug : null;
        markerHtml = `<div><b>${opqBox.name}<br />Region: ${region}<br />${formattedMeasurement}</b></div>`;
      } else {
        markerHtml = `<div><b>${opqBox.name}<br />${formattedMeasurement}</b></div>`;
      }
      newOpts.icon = this.opqBoxIcon({ markerHtml, opqBox });
      marker.refreshIconOptions(newOpts, true);
    }
  }

  calcMarkerPosition(opqBox) {
    const { currentMapLocationGranularity, mapLocationGranularityTypes } = this.props;

    let newLatLng = null;
    switch (currentMapLocationGranularity) {
      case mapLocationGranularityTypes.BOX_LOCATION:
        // For some reason, we are storing coordinates as [lng, lat] rather than [lat, lng]
        newLatLng = this.getOpqBoxLocationDoc(opqBox).coordinates.slice().reverse();
        break;
      case mapLocationGranularityTypes.BOX_REGION:
        newLatLng = this.getOpqBoxRegionCoords(opqBox);
        // Use Location coords if no set region.
        if (!newLatLng) newLatLng = this.getOpqBoxLocationDoc(opqBox).coordinates.slice().reverse();
        break;
      default:
        break;
    }
    return newLatLng;
  }

  getOpqBoxRegionDoc(opqBox) {
    const { regions } = this.props;
    return regions.find(region => region.locationSlug === opqBox.location);
  }

  getOpqBoxRegionCoords(opqBox) {
    const { zipcodeLatLngDict } = this.props;
    const regionDoc = this.getOpqBoxRegionDoc(opqBox);
    // Ensure that regionSlug is a zipcode. Currently, regionSlug is only storing zipcodes (string), but this might
    // change in the future, so let's ensure we are only dealing with a zipcode here by checking that the string
    // has 5 characters and is numeric.
    if (regionDoc && regionDoc.regionSlug && regionDoc.regionSlug.length === 5 && !Number.isNaN(regionDoc.regionSlug)) {
      const zipcode = regionDoc.regionSlug; // We now know that regionSlug is a zipcode string.
      // Retrieve zipcode coords from dict
      const coords = zipcodeLatLngDict[zipcode];
      return coords;
    }
    return null;
  }

  filterCurrentMapDataDisplay(measurement, measurementType, format = false) {
    const { mapDataDisplayTypes } = this.props;
    if (!measurement && format) return 'No Data';
    if (!measurement) return null;
    switch (measurementType) {
      case mapDataDisplayTypes.VOLTAGE_DATA:
        return (format) ? this.formatMeasurement(measurement.voltage, measurementType) : measurement.voltage;
      case mapDataDisplayTypes.FREQUENCY_DATA:
        return (format) ? this.formatMeasurement(measurement.frequency, measurementType) : measurement.frequency;
      case mapDataDisplayTypes.THD_DATA:
        return (format) ? this.formatMeasurement(measurement.thd, measurementType) : measurement.thd;
      default:
        return (format) ? this.formatMeasurement(measurement.voltage, measurementType) : measurement.voltage;
    }
  }

  formatMeasurement(value, measurementType) {
    const { mapDataDisplayTypes } = this.props;
    switch (measurementType) {
      case mapDataDisplayTypes.VOLTAGE_DATA:
        return `${value.toFixed(2)} V`;
      case mapDataDisplayTypes.FREQUENCY_DATA:
        return `${value.toFixed(3)} Hz`;
      case mapDataDisplayTypes.THD_DATA:
        return `${value.toFixed(4)}`;
      default:
        return `${value.toFixed(2)} V`;
    }
  }

  findNewestMeasurement(boxId) {
    const { measurements } = this.props;
    if (measurements.length) {
      // Measurements are already sorted before being passed to the BoxMap component, so Array.find() is appropriate.
      return measurements.find(measurement => measurement.box_id === boxId);
    }
    return null;
  }

  opqBoxExists(opqBox) {
    const boxes = this.getOpqBoxes();
    let exists = false;
    boxes.forEach(box => {
      if (box.box_id === opqBox.box_id) {
        exists = true;
      }
    });
    return exists;
  }

  getOpqBoxes() {
    const { opqBoxAndMarkersDict } = this.state;
    return Object.values(opqBoxAndMarkersDict).map(boxAndMarkers => boxAndMarkers.opqBox);
  }

  getMarkers() {
    const { opqBoxAndMarkersDict } = this.state;
    return Object.values(opqBoxAndMarkersDict).map(boxAndMarkers => boxAndMarkers.marker);
  }

  getMarker(opqBox) {
    const { opqBoxAndMarkersDict } = this.state;
    const boxEntry = opqBoxAndMarkersDict[opqBox._id.toHexString()];
    return (boxEntry) ? boxEntry.marker : null;
  }

  getMarkerLeafletElements() {
    const { opqBoxAndMarkersDict } = this.state;
    return Object.values(opqBoxAndMarkersDict).map(boxAndMarkers => boxAndMarkers.markerLeafletElement);
  }

  getMarkerLeafletElement(opqBox) {
    const { opqBoxAndMarkersDict } = this.state;
    const boxEntry = opqBoxAndMarkersDict[opqBox._id.toHexString()];
    return (boxEntry) ? boxEntry.markerLeafletElement : null;
  }

  markersToRender() {
    const { opqBoxes } = this.props;

    // Only display the current subset of opqBoxes that were passed in props (even though more may exist in
    // opqBoxMarkersDict)
    const boxMarkers = [];
    opqBoxes.forEach(box => {
      const marker = this.getMarker(box);
      if (marker) boxMarkers.push(marker);
    });
    return boxMarkers;
  }

  addMarkerLeafletElementToDict(opqBox) {
    return (elem) => {
      // Elem can sometimes be null due to React's mounting and unmounting behavior.
      if (elem) {
        this.createOrUpdateOpqBoxAndMarkersDictEntry(opqBox._id.toHexString(), {
          markerLeafletElement: elem.leafletElement,
        });
      }
    };
  }

  opqBoxIcon({ markerHtml, opqBox }) {
    const { currentMapDataDisplay } = this.props;
    const newestMeasurement = this.findNewestMeasurement(opqBox.box_id);
    const filteredMeasurement = this.filterCurrentMapDataDisplay(newestMeasurement, currentMapDataDisplay);
    const quality = this.calculateDataQuality(filteredMeasurement);
    let className = 'opqBoxMarker '; // Note the trailing space.
    if (quality === 'good') {
      className += 'blue';
    } else if (quality === 'mediocre') {
      className += 'yellow';
    } else if (quality === 'poor') {
      className += 'red';
    }

    return divIcon({
      html: markerHtml,
      className: className,
      iconSize: [40, 40],
      iconAnchor: [32, 40],
    });
  }

  clusterIcon(cluster) {
    const { opqBoxAndMarkersDict } = this.state;
    const { currentMapDataDisplay = '', currentMapLocationGranularity, mapLocationGranularityTypes } = this.props;
    const children = cluster.getAllChildMarkers();
    let total = 0;
    let clusterActiveBoxesCount = children.length;
    children.forEach(child => {
      const value = Number(child.options.rawValue);
      if (value) { // Reminder: When rawValue = null, Number(null) == 0 == falsy.
        total += value;
      } else {
        clusterActiveBoxesCount--;
      }
    });

    const avg = (clusterActiveBoxesCount) ? total / clusterActiveBoxesCount : 0;
    const formattedAvg = this.formatMeasurement(avg, currentMapDataDisplay);
    const className = this.clusterIconCssClass(avg);

    // Determine regions of all child markers
    const regions = [];
    const boxIds = children.map(boxMarker => boxMarker.options.boxId); // Mongo ids
    boxIds.forEach(id => {
      const opqBoxEntry = opqBoxAndMarkersDict[id];
      if (opqBoxEntry) {
        const region = this.getOpqBoxRegionDoc(opqBoxEntry.opqBox);
        if (region && region.regionSlug) {
          regions.push(region.regionSlug);
        }
      }
    });

    const uniqRegions = regions.filter((region, idx, arr) => arr.indexOf(region) === idx);

    let markerHtml = `
      <div class="marker-cluster container-fix ${className}">
        <div><span><b>${formattedAvg}</b></span></div>
      </div>`;

    if (currentMapLocationGranularity === mapLocationGranularityTypes.BOX_REGION) {
      const regionStr = (uniqRegions.length > 1) ? 'Regions:' : 'Region:';
      markerHtml += `<div class="marker-cluster-sideLabel"><b>${regionStr} ${uniqRegions.toString()}</b></div>`;
    }

    return divIcon({
      html: markerHtml,
      className: 'marker-cluster-container',
      iconSize: [70, 70], // Should be equal to marker-cluster div width (or height) + (margin-left x 2)
    });
  }

  clusterIconCssClass(value) {
    const quality = this.calculateDataQuality(value);
    let className = 'marker-cluster-';
    if (quality === 'poor') {
      className += 'red';
    } else if (quality === 'mediocre') {
      className += 'yellow';
    } else if (quality === 'good') {
      className += 'blue';
    }
    return className;
  }

  calculateDataQuality(value) {
    const { currentMapDataDisplay, mapDataDisplayTypes } = this.props;
    let quality = null; // Can be 'poor', 'mediocre', or 'good'.
    switch (currentMapDataDisplay) {
      case mapDataDisplayTypes.VOLTAGE_DATA: {
        const NOMINAL = 120;
        const MULTIPLIER = 0.05;
        const HALF_MULTIPLIER = MULTIPLIER / 2.0;
        if (value < NOMINAL * (1.0 - MULTIPLIER) || value > NOMINAL * (1.0 + MULTIPLIER)) {
          quality = 'poor';
        } else if (value < NOMINAL * (1.0 - HALF_MULTIPLIER) || value > NOMINAL * (1.0 + HALF_MULTIPLIER)) {
          quality = 'mediocre';
        } else {
          quality = 'good';
        }
        break;
      }
      case mapDataDisplayTypes.FREQUENCY_DATA: {
        const NOMINAL = 60;
        const MULTIPLIER = 0.05;
        const HALF_MULTIPLIER = MULTIPLIER / 2.0;
        if (value < NOMINAL * (1.0 - MULTIPLIER) || value > NOMINAL * (1.0 + MULTIPLIER)) {
          quality = 'poor';
        } else if (value < NOMINAL * (1.0 - HALF_MULTIPLIER) || value > NOMINAL * (1.0 + HALF_MULTIPLIER)) {
          quality = 'mediocre';
        } else {
          quality = 'good';
        }
        break;
      }
      case mapDataDisplayTypes.THD_DATA: {
        // Found these values by just looking at trends data. Need to find out official values.
        if (value < 0.005 || value > 0.05) {
          quality = 'poor';
        } else if (value < 0.01 || value > 0.04) {
          quality = 'mediocre';
        } else {
          quality = 'good';
        }
        break;
      }
      default: {
        break;
      }
    }
    return quality;
  }

  createMarkers(opqBoxes) {
    opqBoxes.forEach(opqBox => this.createMarker(opqBox));
  }

  createMarker(opqBox) {
    // Create new object for the dictionary for the given OpqBox and populate it with the following properties:
    // {opqBox, marker, markerLeafletElement}. See constructor for more details on these properties.

    // We have to update the dict entry in three stages for each new Box/Marker due how to React-Leaflet works.
    // 1. Create new entry in the Dict for the new box id, store the OpqBox document as 'opqBox' property.
    // 2. Create a new Marker for the Box, then update entry with a 'marker' property.
    // 3. From the Ref callback, add a 'markerLeafletElement' property to the entry for the newly created Marker.
    this.createOrUpdateOpqBoxAndMarkersDictEntry(opqBox._id.toHexString(), { opqBox });
    const initialMarkerHtml = `<div><b>${opqBox.name}</div>`;
    // For some reason, we are storing coordinates as [lng, lat] rather than [lat, lng]
    const markerPosition = this.calcMarkerPosition(opqBox);

    // Note: The props passed to Marker here are just the initial values for the Marker component. RawValue and
    // formattedValue are given up-to-date measurement values during updateMarker().
    const newMarker = <Marker
                        ref={this.addMarkerLeafletElementToDict.bind(this)(opqBox)}
                        icon={this.opqBoxIcon({ markerHtml: initialMarkerHtml, opqBox })}
                        key={opqBox._id}
                        rawValue=''
                        formattedValue=''
                        boxId={opqBox._id.toHexString()}
                        position={markerPosition}>
                        <Popup offset={[-10, -30]} maxWidth={300}>
                          {this.opqBoxDetailsList(opqBox)}
                        </Popup>
                      </Marker>;

    this.createOrUpdateOpqBoxAndMarkersDictEntry(opqBox._id.toHexString(), { marker: newMarker });
  }

  getOpqBoxLocationDoc(opqBox) {
    const { locations } = this.props;
    return locations.find(location => opqBox.location === location.slug);
  }

  opqBoxDetailsList(opqBox) {
    const boxLocationDoc = this.getOpqBoxLocationDoc(opqBox);
    const boxRegionDoc = this.getOpqBoxRegionDoc(opqBox);
    return (
        <List divided style={{ width: '250px' }}>
          <List.Item>
            <List.Icon name='hdd outline' color='blue' size='large' verticalAlign='middle' />
            <List.Content style={{ paddingLeft: '2px' }}>
              <List.Header>Box Name</List.Header>
              <List.Description><i>{opqBox.name}</i></List.Description>
            </List.Content>
          </List.Item>
          <List.Item>
            <List.Icon name='hashtag' color='blue' size='large' verticalAlign='middle' />
            <List.Content>
              <List.Header>Box ID</List.Header>
              <List.Description><i>{opqBox.box_id}</i></List.Description>
            </List.Content>
          </List.Item>
          <List.Item>
            <List.Icon name='marker' color='blue' size='large' verticalAlign='middle' />
            <List.Content>
              <List.Header>Location</List.Header>
              <List.Description>
                <i>{boxLocationDoc ? (boxLocationDoc.description) : ('None')}</i>
              </List.Description>
              <List.Header>Coordinates</List.Header>
              <List.Description>
                <i>{boxLocationDoc
                    ? (`Lat: ${boxLocationDoc.coordinates[1]}, Lng: ${boxLocationDoc.coordinates[0]}`)
                    : ('None')}
                </i>
              </List.Description>
              <List.Header><i>Region</i></List.Header>
              <List.Description>
                {boxRegionDoc ? (<i>{boxRegionDoc.regionSlug}</i>) : ('None')}
              </List.Description>
            </List.Content>
          </List.Item>
          <List.Item>
            <List.Icon name='tag' color='blue' size='large' verticalAlign='middle' />
            <List.Content style={{ paddingLeft: '4px' }}>
              <List.Header><i>Description</i></List.Header>
              <List.Description><i>{opqBox.description}</i></List.Description>
            </List.Content>
          </List.Item>
          <List.Item>
            <List.Icon name='plug' color='blue' size='large' verticalAlign='middle' />
            <List.Content>
              <List.Header><i>Unplugged Status</i></List.Header>
              <List.Description><i>{opqBox.unplugged.toString()}</i></List.Description>
            </List.Content>
          </List.Item>
          <List.Item>
            <List.Icon name='creative commons' color='blue' size='large' verticalAlign='middle' />
            <List.Content style={{ paddingLeft: '4px' }}>
              <List.Header>Calibration Constant</List.Header>
              <List.Description><i>{opqBox.calibration_constant}</i></List.Description>
            </List.Content>
          </List.Item>
        </List>
    );
  }

  createOrUpdateOpqBoxAndMarkersDictEntry(boxIdString, opqBoxAndMarkersObj) {
    this.setState(prevState => {
      // Always treat state (and prevState) as immutable. (Actually, this might not be enough for nested objects,
      // see: https://stackoverflow.com/questions/43040721/how-to-update-a-nested-state-in-react)
      const currentDict = { ...prevState.opqBoxAndMarkersDict };
      // FYI: It seems like || {} is not necessary - if currentVal is undefined and we try to spread it, it will simply
      // be ignored. But will keep it like this because it's a bit more clear.
      const currentVal = currentDict[boxIdString] || {};
      const updatedVal = { ...currentVal, ...opqBoxAndMarkersObj };
      currentDict[boxIdString] = updatedVal;
      return {
        opqBoxAndMarkersDict: currentDict,
      };
    });
  }

  zoomToMarker(box_id) {
    const { opqBoxAndMarkersDict } = this.state;
    // Retrieve Marker for the given OpqBox
    const marker = opqBoxAndMarkersDict[box_id].markerLeafletElement;
    // Zoom to marker, open its Popup
    this.markerClusterGroupRefElem.zoomToShowLayer(marker, () => {
      marker.openPopup(); // Trigger marker's Popup after zoom is completed.
    });
  }

  render() {
    // Render when props are ready and Markers have been created.
    return (this.props.ready && this.getMarkers().length)
        ? this.renderPage()
        : <Loader active content='Retrieving data...'/>;
  }

  markerClusterGroupRef(elem) {
    // We need to store the MarkerClusterGroup component's leaflet element because the React component does not expose
    // the zoomToShowLayer() method to us.
    // Only need to store ref on initial call, or when we have a new MCG instance - which occurs whenever we pass in a
    // new set of OpqBox markers to the MCG component. If we do not do this, the zoomToMarker() method will not work
    // because it is pointing to an outdated MCG instance.
    // Note that we always have to check for elem because React seems to always call this ref function twice, with the
    // initial call passing in a null elem.
    if (elem && !this.markerClusterGroupRefElem) {
      this.markerClusterGroupRefElem = elem.leafletElement; // Just store the leaflet element itself.
    } else {
      const isNewInstance = (elem) ? elem.leafletElement._leaflet_id !== this.markerClusterGroupRefElem._leaflet_id
          : false;
      if (elem && isNewInstance) {
        this.markerClusterGroupRefElem = elem.leafletElement; // Just store the leaflet element itself.
      }
    }
  }

  renderPage() {
    return (
        <MarkerClusterGroup
            ref={this.markerClusterGroupRef.bind(this)}
            animate={true}
            maxClusterRadius={100}
            spiderfyDistanceMultiplier={6}
            iconCreateFunction={this.clusterIcon.bind(this)}>
          {this.markersToRender()}
        </MarkerClusterGroup>
    );
  }

}

OpqBoxLeafletMarkerManager.propTypes = {
  ready: PropTypes.bool.isRequired,
  opqBoxes: PropTypes.array.isRequired,
  locations: PropTypes.array.isRequired,
  regions: PropTypes.array.isRequired,
  zipcodeLatLngDict: PropTypes.object.isRequired,
  measurements: PropTypes.array.isRequired,
  currentMapDataDisplay: PropTypes.string.isRequired,
  currentMapLocationGranularity: PropTypes.string.isRequired,
  mapDataDisplayTypes: PropTypes.object.isRequired,
  mapLocationGranularityTypes: PropTypes.object.isRequired,
};

export default withTracker(props => {
  const { opqBoxes = [], zipcodeLatLngDict, childRef } = props;
  const measurementsSub = Meteor.subscribe(
      Measurements.publicationNames.BOX_MAP_MEASUREMENTS,
      opqBoxes.map(box => box.box_id),
  );
  return {
    ready: measurementsSub.ready(),
    opqBoxes,
    zipcodeLatLngDict,
    measurements: Measurements.find({}, { sort: { timestamp_ms: -1 } }).fetch(),
    ref: childRef,
  };
})(OpqBoxLeafletMarkerManager);