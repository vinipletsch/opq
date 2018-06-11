"""
This plugin detects, classifies, and stores frequency variation incidents.
Frequency variations are classified as +/-0.10hz as specified by IEEE standards
"""
import typing
import multiprocessing
import numpy
import constants
import plugins.base
import protobuf.mauka_pb2
import protobuf.util

class FrequencyVariationPlugin(plugins.base.MaukaPlugin):
    """
    Mauka plugin that classifies and stores frequency variation incidents for any event that includes a raw waveform
    """
    NAME = "FrequencyVariationPlugin"

    def __init__(self, config: typing.Dict, exit_event: multiprocessing.Event):
        """
        Initializes this plugin
        :param config: Mauka configuration
        :param exit_event: Exit event that can disable this plugin from parent process
        """
        super().__init__(config, ["WindowedFrequency"], FrequencyVariationPlugin.NAME, exit_event)
        self.freq_ref = float(self.config_get("plugins.FrequencyVariationPlugin.frequency.ref"))
        self.freq_var_low = float(self.config_get("plugins.FrequencyVariationPlugin.frequency.variation.threshold.low"))
        self.freq_var_high = float(self.config_get("plugins.FrequencyVariationPlugin.frequency.variation.threshold.high"))

    def _frequency_incident_classifier(self, event_id: int, box_id: str, windowed_frequencies: numpy.ndarray,
                                       window_size: float = constants.SAMPLES_PER_CYCLE):
        """
        Classifies a frequency incident as a Sag, Swell, or Interruption. Creates a Mongo Anomaly document
        :param event_id: Makai Event ID
        :param box_id: Box reporting event
        :param windowed_frequencies: High fidelity frequency measurements of windows
        :param window_size: The number of samples per window
        """

        window_duration = window_size / constants.SAMPLE_RATE_HZ
        

    def on_message(self, topic, mauka_message):
        """
        Called async when a topic this plugin subscribes to produces a message
        :param topic: The topic that is producing the message
        :param message: The message that was produced
        """
        self.debug("on_message")
        if protobuf.util.is_payload(mauka_message, protobuf.mauka_pb2.FREQUENCY_WINDOWED):
            self.debug("on_message {}:{} len:{}".format(mauka_message.payload.event_id,
                                                        mauka_message.payload.box_id,
                                                        len(mauka_message.payload.data)))
            self._frequency_incident_classifier(mauka_message.payload.event_id,
                                                mauka_message.payload.box_id,
                                                protobuf.util.repeated_as_ndarray(
                                                mauka_message.payload.data
                                                ))

        else:
            self.logger.error("Received incorrect mauka message [{}] at FrequencyVariationPlugin".format(
                protobuf.util.which_message_oneof(mauka_message)
            ))
