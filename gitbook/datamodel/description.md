# OPQ Data Model

## Overview {#overview}

At the core of the OPQ system lies a centralized MongoDB database. The majority of this data is produced by the OPQMauka and OPQMakai systems.
The following section will provide a high-level overview of OPQ's data model.

## Data Model

The OPQ system utilizes the following collections:
* **[measurements](#measurements):** Provides low-fidelity OPQBox data
* **[events](#events):** Provides detected events
* **[box_events](#box_events):** Provides high-fidelity event data
* **[fs.files](#fs.files) and [fs.chunks](#fs.chunks)** Internal to GridFS. Stores box_event binary waveform data.
* **[opq_boxes](#opq_boxes):** Provides individual OPQBox information
* **[users](#users):** Provides user information

### Naming Conventions
The OPQ system is comprised of a multitude of different tools, libraries, and frameworks. 
In order to minimize confusion, we follow a basic set of naming conventions for our collections and documents that we feel will keep things as simple as possible:
* All collection names and documents fields are in lower-case
* Collection names should always be plural
* Use underscores over camel-case to separate words

### Measurements {#measurements}

The **measurements** collection provides low-fidelity OPQBox snapshot data at a given moment in time.
Due to the high default sampling rate of the corresponding [OPQMauka measurements plugin](../mauka/plugins.md#measurement), documents in this collection are produced at very rapid rate.
As such, each measurement document can essentially be thought of as an OPQBox "heartbeat", providing a timestamp and some additional low-fidelity data.

![Measurements Collection](images/measurements-collection.png)

Each measurement document always corresponds to a single OPQBox, as indicated by the **box_id** field.

The **voltage** and **frequency** fields are RMS calculations of voltage and frequency at a given moment in time, indicated by the **timestamp_ms** field.

The **location.zipcode** field stores the OPQBox zipcode location. Note how the **location** field is an object, with a **zipcode** sub-property.
This **location** object (and its sub-properties) should always match the location object stored in the **opq_boxes** collection.

### Events and Box Events
#### Events {#events}
The **events** collection provides events detected by the OPQ System. These documents are generated by OPQMakai after being requested by OPQMauka upon event detection.

![Events and Box Events Collection](images/events-boxevents-collection.png)

The **event_id** field is a unique integer value generated for each event. 

The **type** field indicates the classification of the event, as determined by OPQMauka.
Valid event types currently are:
* "FREQUENCY_SAG"
* "FREQUENCY_SWELL"
* "VOLTAGE_SAG"
* "VOLTAGE_SWELL"
* "THD"
* "OTHER"


The **description** field indicates additional information about the event (*Serge: Clarify on this?*)

The **boxes_triggered** array is a list of all OPQBoxes associated with the given event - however it is important to note that this does not always correspond to all of the OPQBoxes for which we have received actual data from for the event.

The **latencies** field provides (*Serge: Clarify on this array? Note that this was previously named "timestamps"*)

#### Box Events {#box_events}
The **box_events** collection provides the event meta-data for a given OPQBox.

The **event_id** field corresponds to the event_id generated by the aforementioned **events** document.

The **box_id** field indicates the OPQBox from which this data was produced.

As an event can be associated with multiple OPQBoxes, it is therefore important to understand that there can be (and often are) multiple box_event documents with the same event_id.
Together, the **event_id** and **box_id** fields are what one would query on in order to find data for a given OPQBox for a specific event.

The **event_start** and **event_end** fields are unix timestamps indicating the start and end time of the event.

The **latencies** array is (*Serge: Again, could you clarify on this? Was previously named timestamps*)

The **thd** and **itic** fields correspond to the total harmonic distortion and ITIC values, as calculated by the OPQMauka [THD](../mauka/plugins.md#thd) and [ITIC](../mauka/plugins.md#itic) plugins.

The **location.zipcode** field stores the OPQBox zipcode location. Note how the **location** field is an object, with a **zipcode** sub-property.
This **location** object (and its sub-properties) should always match the location object stored in the **opq_boxes** collection.

The **data_fs_filename** field indicates the GridFS filename that holds the box_event's actual raw waveform data.

### GridFS
[GridFS](https://docs.mongodb.com/manual/core/gridfs/) is a MongoDB specification for storing large documents. As an OPQBox can collect a very large amount of data for each given event (often exceeding the 16 MB MongoDB document size limit), we've opted to utilize GridFS to store our high-fidelity data.
At its core, GridFS is a very simple system consisting of two collections, **fs.files** and **fs.chunks**. 

![GridFS Collections](images/gridfs-collections.png)

#### FS.Files {#fs.files}
The **fs.files** collection is internal to GridFS and stores file metadata.
All fields except **metadata.event_id** and **metadata.box_id** are generated by GridFS upon document creation.
Of all these fields internal to GridFS, the most noteworthy is the **filename** field, which corresponds to the box_event's **data_fs_filename** field.

The **metadata.event_id** and **metadata.box_id** fields are used to find the corresponding **box_event** document for which this file holds data for.

Note: The GridFS specification requires the **metadata** field be used to store any external information for the given file document. See [GridFS files.metadata](https://docs.mongodb.com/manual/core/gridfs/#files.metadata)  for more information.

#### FS.Chunks {#fs.chunks}
This collection is internal to GridFS and is used for storing file chunks.
The **files_id** field is a Mongo ObjectID reference to the chunk's corresponding **fs.files** document.
It might be interesting to note that this is the only occurrence in the data model where a Mongo ObjectID is being referenced.

### OPQBoxes {#opq_boxes}
The **opq_boxes** collection provides the information of each individual OPQBox in the system.

![OPQBoxes Collection](images/opqboxes-collection.png)

The **box_id** field is a unique string identifier for the OPQBox. This value is always referenced throughout the data model when we need to store a box_id value within a document.

The **name** field is a unique user-friendly string identifier for the OPQBox. Unlike the **box_id** value, which is often used internally throughout the data model, the **name** value should be thought of as the external representation of the OPQBox.

The **description** field is optional and can be used to further describe an OPQBox.

The **calibration_constant** field is the box specific value that is used to calculate the actual waveform data values. (*Serge: Is this correct?*)

As there are a variety of ways to represent a location, the **location** field is an *object* with its sub-properties representing different types of location definitions.
For now, we are only keeping track of zipcode, but this approach makes it trivial to add additional location types (such at lat and lng) in the future. 

The **location.zipcode** field indicates the zipcode location where the OPQBox currently resides. 

### Users {#users}
The **users** collection provides information on a given user of the OPQ system.

![Users Collection](images/users-collection.png)

The **email** field is the user's email address. This field also serves as the username to log into OPQView.

The **password** field is the user's password, encrypted with bcrypt.

The **first_name** and **last_name** fields are the user's first and last name.

The **boxes** array indicates the **opq_boxes** owned by the user. 

The **role** field indicates the role of the user in the OPQ system. Currently, there are only two roles: "user" and "admin".