"""
This plugin calculates the ITIC region of a voltage, duration pair.
"""
import enum
import typing
import multiprocessing

import plugins.base

import numpy
import matplotlib.path


class IticRegion(enum.Enum):
    """
    Enumerations of ITIC regions.
    """
    NO_INTERRUPTION = "NO_INTERRUPTION"
    PROHIBITED = "PROHIBITED"
    NO_DAMAGE = "NO_DAMAGE"
    OTHER = "OTHER"


def c_to_ms(c: float) -> float:
    """
    Convert cycles to milliseconds
    :param c: cycles
    :return: milliseconds
    """
    return (c * (1 / 60)) * 1000.0


HUNDREDTH_OF_A_CYCLE = c_to_ms(0.01)

prohibited_region_polygon = [
    [HUNDREDTH_OF_A_CYCLE, 500],
    [1, 200],
    [3, 140],
    [3, 120],
    [20, 120],
    [5000, 120],
    [5000, 110],
    [10000, 110],
    [10000, 500],
    [HUNDREDTH_OF_A_CYCLE, 500]
]

no_damage_region_polygon = [
    [20, 0],
    [20, 40],
    [20, 70],
    [5000, 70],
    [5000, 80],
    [10000, 80],
    [10000, 90],
    [10000, 0],
    [20, 0]
]

no_interruption_region_polygon = [
    [0, 0],
    [0, 500],
    [HUNDREDTH_OF_A_CYCLE, 500],
    [1, 200],
    [3, 140],
    [3, 120],
    [20, 120],
    [5000, 120],
    [5000, 110],
    [10000, 110],
    [10000, 90],
    [10000, 80],
    [5000, 80],
    [5000, 70],
    [20, 70],
    [20, 40],
    [20, 0],
    [0, 0]
]


def point_in_polygon(x: float, y: float, polygon: typing.List[typing.List[float]]) -> bool:
    """
    Checks if a point is in a given polygon.
    :param x: x
    :param y: y
    :param polygon: The polygon to check for inclusion
    :return: Whether or not the given point is in the provided polygon
    """
    path = matplotlib.path.Path(vertices=numpy.array(polygon), closed=True)
    return path.contains_point([x, y])


def itic_region(rms_voltage: float, duration_ms: float) -> enum.Enum:
    """
    Returns the ITIC region of a given RMS voltage and duration.
    The reference curve is at http://www.keysight.com/upload/cmc_upload/All/1.pdf
    :param rms_voltage: The RMS voltage value
    :param duration_ms: The duration of the voltage event in milliseconds
    :return: The appropriate ITIC region enum
    """

    percent_nominal = (rms_voltage / 120.0) * 100.0

    # First, let's check the extreme edge cases
    if duration_ms < c_to_ms(0.01):
        return IticRegion.NO_INTERRUPTION

    if rms_voltage <= 0:
        if duration_ms <= 20:
            return IticRegion.NO_INTERRUPTION
        else:
            return IticRegion.NO_DAMAGE

    # In the x and y directions
    if duration_ms >= 10000 and percent_nominal >= 500:
        return IticRegion.PROHIBITED

    # In the x-direction
    if duration_ms >= 10000:
        if percent_nominal >= 110:
            return IticRegion.PROHIBITED
        elif percent_nominal <= 90:
            return IticRegion.NO_DAMAGE
        else:
            return IticRegion.NO_INTERRUPTION

    # In the y-direction
    if percent_nominal >= 500:
        if duration_ms <= HUNDREDTH_OF_A_CYCLE:
            return IticRegion.NO_INTERRUPTION
        else:
            return IticRegion.PROHIBITED

    # If the voltage is not an extreme case, we run point in polygon calculations to determine which region its in
    if point_in_polygon(duration_ms, percent_nominal, no_interruption_region_polygon):
        return IticRegion.NO_INTERRUPTION

    if point_in_polygon(duration_ms, percent_nominal, prohibited_region_polygon):
        return IticRegion.PROHIBITED

    if point_in_polygon(duration_ms, percent_nominal, no_damage_region_polygon):
        return IticRegion.NO_DAMAGE

    # If it's directly on the line of one of the polygons, its easiest to just say no_interruption
    return IticRegion.NO_INTERRUPTION


class IticPlugin(plugins.base.MaukaPlugin):
    NAME = "IticPlugin"

    def __init__(self, config: typing.Dict, exit_event: multiprocessing.Event):
        super().__init__(config, ["VoltageEvent"], IticPlugin.NAME, exit_event)

    def on_message(self, topic, message):
        pass


if __name__ == "__main__":
    from matplotlib.patches import PathPatch
    import matplotlib.pyplot as plt

    poly = prohibited_region_polygon
    path = matplotlib.path.Path(vertices=numpy.array(poly), closed=True)
    pathpatch = PathPatch(path)
    fig, ax = plt.subplots()
    ax.add_patch(pathpatch)

    ax.dataLim.update_from_data_xy(poly)
    ax.autoscale_view()

    plt.show()