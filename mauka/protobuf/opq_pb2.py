# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: opq.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
from google.protobuf import descriptor_pb2
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='opq.proto',
  package='opq.proto',
  syntax='proto2',
  serialized_pb=_b('\n\topq.proto\x12\topq.proto\"[\n\x05\x43ycle\x12\x0c\n\x04time\x18\x01 \x02(\x04\x12\x0c\n\x04\x64\x61ta\x18\x02 \x03(\x05\x12\x10\n\x08last_gps\x18\x03 \x01(\x05\x12\x15\n\rcurrent_count\x18\x04 \x01(\x05\x12\r\n\x05\x66lags\x18\x05 \x01(\x05\";\n\x0b\x44\x61taMessage\x12\n\n\x02id\x18\x01 \x02(\x05\x12 \n\x06\x63ycles\x18\x03 \x03(\x0b\x32\x10.opq.proto.Cycle\"\x8f\x01\n\x0eTriggerMessage\x12\n\n\x02id\x18\x01 \x02(\x05\x12\x0c\n\x04time\x18\x02 \x02(\x04\x12\x11\n\tfrequency\x18\x03 \x02(\x02\x12\x0b\n\x03rms\x18\x04 \x02(\x02\x12\x0b\n\x03thd\x18\x05 \x01(\x02\x12\x10\n\x08last_gps\x18\x06 \x01(\x05\x12\x15\n\rcurrent_count\x18\x07 \x01(\x05\x12\r\n\x05\x66lags\x18\x08 \x01(\x05\"\xf8\x01\n\x12RequestDataMessage\x12\x37\n\x04type\x18\x01 \x02(\x0e\x32).opq.proto.RequestDataMessage.RequestType\x12\x17\n\x0fsequence_number\x18\x02 \x02(\r\x12\r\n\x05\x62oxId\x18\x03 \x01(\r\x12\x0c\n\x04time\x18\x04 \x01(\x04\x12\x0c\n\x04\x62\x61\x63k\x18\x05 \x01(\x04\x12\x0f\n\x07\x66orward\x18\x06 \x01(\x04\x12\x12\n\nnum_cycles\x18\x07 \x01(\r\"@\n\x0bRequestType\x12\x08\n\x04PING\x10\x01\x12\x08\n\x04PONG\x10\x02\x12\x08\n\x04READ\x10\x03\x12\x08\n\x04RESP\x10\x04\x12\t\n\x05\x45RROR\x10\x05\"\xe5\x02\n\x13RequestEventMessage\x12\x1e\n\x16start_timestamp_ms_utc\x18\x01 \x02(\x04\x12\x1c\n\x14\x65nd_timestamp_ms_utc\x18\x02 \x02(\x04\x12@\n\x0ctrigger_type\x18\x03 \x02(\x0e\x32*.opq.proto.RequestEventMessage.TriggerType\x12\x19\n\x11percent_magnitude\x18\x04 \x02(\x01\x12\x0f\n\x07\x62ox_ids\x18\x05 \x03(\x05\x12\x11\n\trequestee\x18\x06 \x02(\t\x12\x13\n\x0b\x64\x65scription\x18\x07 \x02(\t\x12\x14\n\x0crequest_data\x18\x08 \x02(\x08\"d\n\x0bTriggerType\x12\x11\n\rFREQUENCY_SAG\x10\x01\x12\x13\n\x0f\x46REQUENCY_SWELL\x10\x02\x12\x0f\n\x0bVOLTAGE_SAG\x10\x03\x12\x11\n\rVOLTAGE_SWELL\x10\x04\x12\t\n\x05OTHER\x10\x05')
)



_REQUESTDATAMESSAGE_REQUESTTYPE = _descriptor.EnumDescriptor(
  name='RequestType',
  full_name='opq.proto.RequestDataMessage.RequestType',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='PING', index=0, number=1,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='PONG', index=1, number=2,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='READ', index=2, number=3,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='RESP', index=3, number=4,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='ERROR', index=4, number=5,
      options=None,
      type=None),
  ],
  containing_type=None,
  options=None,
  serialized_start=509,
  serialized_end=573,
)
_sym_db.RegisterEnumDescriptor(_REQUESTDATAMESSAGE_REQUESTTYPE)

_REQUESTEVENTMESSAGE_TRIGGERTYPE = _descriptor.EnumDescriptor(
  name='TriggerType',
  full_name='opq.proto.RequestEventMessage.TriggerType',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='FREQUENCY_SAG', index=0, number=1,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='FREQUENCY_SWELL', index=1, number=2,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='VOLTAGE_SAG', index=2, number=3,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='VOLTAGE_SWELL', index=3, number=4,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='OTHER', index=4, number=5,
      options=None,
      type=None),
  ],
  containing_type=None,
  options=None,
  serialized_start=833,
  serialized_end=933,
)
_sym_db.RegisterEnumDescriptor(_REQUESTEVENTMESSAGE_TRIGGERTYPE)


_CYCLE = _descriptor.Descriptor(
  name='Cycle',
  full_name='opq.proto.Cycle',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='time', full_name='opq.proto.Cycle.time', index=0,
      number=1, type=4, cpp_type=4, label=2,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='data', full_name='opq.proto.Cycle.data', index=1,
      number=2, type=5, cpp_type=1, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='last_gps', full_name='opq.proto.Cycle.last_gps', index=2,
      number=3, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='current_count', full_name='opq.proto.Cycle.current_count', index=3,
      number=4, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='flags', full_name='opq.proto.Cycle.flags', index=4,
      number=5, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=24,
  serialized_end=115,
)


_DATAMESSAGE = _descriptor.Descriptor(
  name='DataMessage',
  full_name='opq.proto.DataMessage',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='id', full_name='opq.proto.DataMessage.id', index=0,
      number=1, type=5, cpp_type=1, label=2,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='cycles', full_name='opq.proto.DataMessage.cycles', index=1,
      number=3, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=117,
  serialized_end=176,
)


_TRIGGERMESSAGE = _descriptor.Descriptor(
  name='TriggerMessage',
  full_name='opq.proto.TriggerMessage',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='id', full_name='opq.proto.TriggerMessage.id', index=0,
      number=1, type=5, cpp_type=1, label=2,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='time', full_name='opq.proto.TriggerMessage.time', index=1,
      number=2, type=4, cpp_type=4, label=2,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='frequency', full_name='opq.proto.TriggerMessage.frequency', index=2,
      number=3, type=2, cpp_type=6, label=2,
      has_default_value=False, default_value=float(0),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='rms', full_name='opq.proto.TriggerMessage.rms', index=3,
      number=4, type=2, cpp_type=6, label=2,
      has_default_value=False, default_value=float(0),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='thd', full_name='opq.proto.TriggerMessage.thd', index=4,
      number=5, type=2, cpp_type=6, label=1,
      has_default_value=False, default_value=float(0),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='last_gps', full_name='opq.proto.TriggerMessage.last_gps', index=5,
      number=6, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='current_count', full_name='opq.proto.TriggerMessage.current_count', index=6,
      number=7, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='flags', full_name='opq.proto.TriggerMessage.flags', index=7,
      number=8, type=5, cpp_type=1, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=179,
  serialized_end=322,
)


_REQUESTDATAMESSAGE = _descriptor.Descriptor(
  name='RequestDataMessage',
  full_name='opq.proto.RequestDataMessage',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='type', full_name='opq.proto.RequestDataMessage.type', index=0,
      number=1, type=14, cpp_type=8, label=2,
      has_default_value=False, default_value=1,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='sequence_number', full_name='opq.proto.RequestDataMessage.sequence_number', index=1,
      number=2, type=13, cpp_type=3, label=2,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='boxId', full_name='opq.proto.RequestDataMessage.boxId', index=2,
      number=3, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='time', full_name='opq.proto.RequestDataMessage.time', index=3,
      number=4, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='back', full_name='opq.proto.RequestDataMessage.back', index=4,
      number=5, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='forward', full_name='opq.proto.RequestDataMessage.forward', index=5,
      number=6, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='num_cycles', full_name='opq.proto.RequestDataMessage.num_cycles', index=6,
      number=7, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
    _REQUESTDATAMESSAGE_REQUESTTYPE,
  ],
  options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=325,
  serialized_end=573,
)


_REQUESTEVENTMESSAGE = _descriptor.Descriptor(
  name='RequestEventMessage',
  full_name='opq.proto.RequestEventMessage',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='start_timestamp_ms_utc', full_name='opq.proto.RequestEventMessage.start_timestamp_ms_utc', index=0,
      number=1, type=4, cpp_type=4, label=2,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='end_timestamp_ms_utc', full_name='opq.proto.RequestEventMessage.end_timestamp_ms_utc', index=1,
      number=2, type=4, cpp_type=4, label=2,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='trigger_type', full_name='opq.proto.RequestEventMessage.trigger_type', index=2,
      number=3, type=14, cpp_type=8, label=2,
      has_default_value=False, default_value=1,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='percent_magnitude', full_name='opq.proto.RequestEventMessage.percent_magnitude', index=3,
      number=4, type=1, cpp_type=5, label=2,
      has_default_value=False, default_value=float(0),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='box_ids', full_name='opq.proto.RequestEventMessage.box_ids', index=4,
      number=5, type=5, cpp_type=1, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='requestee', full_name='opq.proto.RequestEventMessage.requestee', index=5,
      number=6, type=9, cpp_type=9, label=2,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='description', full_name='opq.proto.RequestEventMessage.description', index=6,
      number=7, type=9, cpp_type=9, label=2,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='request_data', full_name='opq.proto.RequestEventMessage.request_data', index=7,
      number=8, type=8, cpp_type=7, label=2,
      has_default_value=False, default_value=False,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
    _REQUESTEVENTMESSAGE_TRIGGERTYPE,
  ],
  options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=576,
  serialized_end=933,
)

_DATAMESSAGE.fields_by_name['cycles'].message_type = _CYCLE
_REQUESTDATAMESSAGE.fields_by_name['type'].enum_type = _REQUESTDATAMESSAGE_REQUESTTYPE
_REQUESTDATAMESSAGE_REQUESTTYPE.containing_type = _REQUESTDATAMESSAGE
_REQUESTEVENTMESSAGE.fields_by_name['trigger_type'].enum_type = _REQUESTEVENTMESSAGE_TRIGGERTYPE
_REQUESTEVENTMESSAGE_TRIGGERTYPE.containing_type = _REQUESTEVENTMESSAGE
DESCRIPTOR.message_types_by_name['Cycle'] = _CYCLE
DESCRIPTOR.message_types_by_name['DataMessage'] = _DATAMESSAGE
DESCRIPTOR.message_types_by_name['TriggerMessage'] = _TRIGGERMESSAGE
DESCRIPTOR.message_types_by_name['RequestDataMessage'] = _REQUESTDATAMESSAGE
DESCRIPTOR.message_types_by_name['RequestEventMessage'] = _REQUESTEVENTMESSAGE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

Cycle = _reflection.GeneratedProtocolMessageType('Cycle', (_message.Message,), dict(
  DESCRIPTOR = _CYCLE,
  __module__ = 'opq_pb2'
  # @@protoc_insertion_point(class_scope:opq.proto.Cycle)
  ))
_sym_db.RegisterMessage(Cycle)

DataMessage = _reflection.GeneratedProtocolMessageType('DataMessage', (_message.Message,), dict(
  DESCRIPTOR = _DATAMESSAGE,
  __module__ = 'opq_pb2'
  # @@protoc_insertion_point(class_scope:opq.proto.DataMessage)
  ))
_sym_db.RegisterMessage(DataMessage)

TriggerMessage = _reflection.GeneratedProtocolMessageType('TriggerMessage', (_message.Message,), dict(
  DESCRIPTOR = _TRIGGERMESSAGE,
  __module__ = 'opq_pb2'
  # @@protoc_insertion_point(class_scope:opq.proto.TriggerMessage)
  ))
_sym_db.RegisterMessage(TriggerMessage)

RequestDataMessage = _reflection.GeneratedProtocolMessageType('RequestDataMessage', (_message.Message,), dict(
  DESCRIPTOR = _REQUESTDATAMESSAGE,
  __module__ = 'opq_pb2'
  # @@protoc_insertion_point(class_scope:opq.proto.RequestDataMessage)
  ))
_sym_db.RegisterMessage(RequestDataMessage)

RequestEventMessage = _reflection.GeneratedProtocolMessageType('RequestEventMessage', (_message.Message,), dict(
  DESCRIPTOR = _REQUESTEVENTMESSAGE,
  __module__ = 'opq_pb2'
  # @@protoc_insertion_point(class_scope:opq.proto.RequestEventMessage)
  ))
_sym_db.RegisterMessage(RequestEventMessage)


# @@protoc_insertion_point(module_scope)
