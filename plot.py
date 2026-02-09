#!/usr/bin/env python3


import argparse
from copy import deepcopy
from enum import Enum
import json
import os
from typing import Any
import plotly.graph_objects as go
import plotly.offline as po

BASE_DIR = "./results"
PEERING_DATA = os.path.join(BASE_DIR, "peering_data.json")
MIRRORED_DATA = os.path.join(BASE_DIR, "peering_data_mirrored.json")
PEERING_OUTPUT = os.path.join(BASE_DIR, "peerings.html")
T1_ASNS = {
    174: "Cogent",
    701: "Verizon",
    1273: "Vodafone",
    1299: "Arelion",
    2914: "NTT",
    3257: "GTT",
    3320: "DTAG",
    3356: "Lumen",
    3491: "PCCW",
    5511: "Orange",
    6453: "TATA",
    6461: "Zayo",
    6762: "TI Sparkle",
    6830: "Liberty Global",
    6939: "Hurricane Electric",
    7018: "AT&T",
    12956: "Telxius",
}


class PeerTypes(Enum):
    Customer = "Customer"
    Peer = "Peer"
    PaidPeer = "Paid Peer"
    Upstream = "Upstream"
    NoneFound = ""


class PeerLocations(Enum):
    Africa = "Africa"
    AsiaPac = "Asia Pac"
    Europe = "Europe"
    MiddleEast = "Middle East"
    NorthAmerica = "North America"
    SouthAmerica = "South America"
    NoneFound = "None Found"


cli_args: argparse.Namespace


def load_json(filename: str) -> dict[str, Any]:
    data: dict[str, dict[str, Any]] = json.load(open(filename))
    return data


def mirror_data(peering_data: dict[str, Any]) -> dict[str, Any]:
    """
    Add corresponding entries to peering data, for peerings discoverd on only
    one side of the peering i.e., if we found 123 is peering for 456, but we
    don't have a peering under 456 to 123; add the reverse direction because
    we know it exists from the direction we did find.
    """
    mirrored_data = deepcopy(peering_data)

    for local_asn, peers in peering_data["peering_data"].items():
        for peer_asn, location_peerings in peers["peers"].items():

            local_ptr = mirrored_data["peering_data"]

            if peer_asn not in local_ptr:
                local_ptr[peer_asn] = {"peers": {}}

            peer_prt = local_ptr[peer_asn]["peers"]

            if local_asn not in peer_prt:
                peer_prt[local_asn] = {"location_peerings": {}}

            for location, peerings_in_loc in location_peerings[
                "location_peerings"
            ].items():

                location_ptr = peer_prt[local_asn]["location_peerings"]

                if location not in location_ptr:
                    location_ptr[location] = {"peerings_in_loc": {}}

                for peer_type, peerings_by_ver in peerings_in_loc[
                    "peerings_in_loc"
                ].items():

                    peerings_ptr = location_ptr[location]["peerings_in_loc"]

                    # Customer and upstream roles need to be reversed
                    if peer_type == PeerTypes.Customer.name:
                        peer_type = PeerTypes.Upstream.name
                    elif peer_type == PeerTypes.Upstream.name:
                        peer_type = PeerTypes.Customer.name

                    if peer_type not in peerings_ptr:
                        peerings_ptr[peer_type] = {"peerings_by_ver": {}}

                    version_prt = peerings_ptr[peer_type]["peerings_by_ver"]

                    for addr_family, route in peerings_by_ver[
                        "peerings_by_ver"
                    ].items():

                        if addr_family not in version_prt:
                            version_prt[addr_family] = route

    return mirrored_data["peering_data"]


def generate_plot_data(data: dict[str, Any]) -> dict[str, Any]:
    """
    Take the data as parsed from JSON, and transpose it into column-wise data
    for plotting. Also create to data structures for storing the text colour
    and background colour of each table cell, based on it's value.
    """

    text_colour_map = {
        "no_data": "black",
        "peer": "darkgreen",
        "non_peer": "firebrick",
    }
    fill_colour_map = {
        "no_data": "white",
        "skip": "#f0f0f0",
        "peer": "#eaf8e0",
        "non_peer": "lightgoldenrodyellow",
    }

    fill_colours: dict[int, list[list[str]]] = {}
    for t1_asn in T1_ASNS.keys():
        fill_colours[t1_asn] = [
            [fill_colour_map["no_data"] for _ in T1_ASNS.keys()]
        ] + [
            [fill_colour_map["no_data"] for _ in T1_ASNS.keys()]
            for _ in PeerLocations
        ]

    text_colours: dict[int, list[list[str]]] = {}
    for t1_asn in T1_ASNS.keys():
        text_colours[t1_asn] = [
            [text_colour_map["no_data"] for _ in T1_ASNS.keys()]
        ] + [
            [text_colour_map["no_data"] for _ in T1_ASNS.keys()]
            for _ in PeerLocations
        ]

    # Values are by column
    values: dict[int, list[list[str]]] = {
        t1_asn: [[]] + [[] for _ in PeerLocations] for t1_asn in T1_ASNS.keys()
    }

    for t1_asn in T1_ASNS.keys():

        for asn_index, local_asn in enumerate(T1_ASNS.keys()):
            values[t1_asn][0].append(
                f"<b>{local_asn}</b> ({T1_ASNS[local_asn]})"
            )

            if t1_asn == local_asn:
                for i in range(1, len(values[t1_asn])):
                    values[t1_asn][i].append("-")
                    fill_colours[t1_asn][i][asn_index] = fill_colour_map[
                        "skip"
                    ]

                continue

            if (
                str(t1_asn) not in data
                or str(local_asn) not in data[str(t1_asn)]["peers"]
            ):
                for i in range(1, len(values[t1_asn])):
                    values[t1_asn][i].append("")
                continue

            locations: dict[str, dict[str, dict[str, Any]]] = data[
                str(t1_asn)
            ]["peers"][str(local_asn)]["location_peerings"]
            for loc_index, location in enumerate(PeerLocations):
                if not location.name in locations:
                    values[t1_asn][loc_index + 1].append("")
                    continue

                peer_types = set(
                    sorted(
                        list(
                            locations[location.name]["peerings_in_loc"].keys()
                        )
                    )
                )
                values[t1_asn][loc_index + 1].append(
                    f"{', '.join(peer_types)}"
                )

                if peer_types == set([PeerTypes.Peer.name]):
                    mapping = "peer"
                elif peer_types == set([PeerTypes.NoneFound.name]):
                    mapping = "no_data"
                else:
                    mapping = "non_peer"

                fill_colours[t1_asn][loc_index + 1][asn_index] = (
                    fill_colour_map[mapping]
                )
                text_colours[t1_asn][loc_index + 1][asn_index] = (
                    text_colour_map[mapping]
                )

    return {
        "values": values,
        "fill_colours": fill_colours,
        "text_colours": text_colours,
    }


def plot_peerings(data: dict[str, Any]) -> None:
    col_headings = ["<b>ASN</b>"] + [
        f"<b>{location.value}</b>" for location in PeerLocations
    ]

    data = generate_plot_data(data)
    values, fill_colours, text_colours = (
        data["values"],
        data["fill_colours"],
        data["text_colours"],
    )

    fig = go.Figure(
        data=[
            go.Table(  # type: ignore
                header=dict(
                    values=col_headings,
                    line_color="gainsboro",
                    fill_color="white",
                    align="center",
                    font=dict(color="black", size=12),
                ),
                cells=dict(
                    values=values[list(T1_ASNS.keys())[0]],
                    line_color="gainsboro",
                    fill_color=fill_colours[list(T1_ASNS.keys())[0]],
                    align="center",
                    font=dict(
                        color=text_colours[list(T1_ASNS.keys())[0]], size=12
                    ),
                ),
            )
        ]
    )

    fig_title = dict(
        text="Peering Locations and Relationships Inferred from Communities",
        x=0.5,
        font=dict(size=22),
    )

    fig.update_layout(  # type: ignore
        updatemenus=[
            dict(
                active=0,
                buttons=list(
                    [
                        dict(
                            label=t1_asn,
                            method="update",
                            args=[
                                dict(
                                    cells=dict(
                                        values=values[t1_asn],
                                        line=dict(color="gainsboro"),
                                        fill=dict(color=fill_colours[t1_asn]),
                                        font=dict(
                                            color=text_colours[t1_asn],
                                            size=12,
                                        ),
                                    ),
                                ),
                                dict(
                                    title=fig_title
                                    | dict(
                                        subtitle=dict(
                                            text=f"Peerings found for AS{t1_asn} ({T1_ASNS[t1_asn]})",
                                            font=dict(size=18),
                                        ),
                                    )
                                ),
                            ],
                        )
                        for t1_asn in T1_ASNS.keys()
                    ]
                ),
            )
        ]
    )

    fig.update_layout(
        title=fig_title
        | dict(
            subtitle=dict(
                text=f"Peerings found for AS{list(T1_ASNS.keys())[0]} ({list(T1_ASNS.values())[0]})",
                font=dict(size=18),
            ),
        ),
        margin=dict(l=0, r=0, b=0, t=100, pad=0),
    )
    po.plot(
        fig,
        filename=PEERING_OUTPUT,
        auto_open=False,
    )
    print(f"Wrote to {PEERING_OUTPUT}")


def write_json(data: dict[str, Any], filename: str):
    json.dump(data, open(filename, "w"), indent=2)
    print(f"Wrote to {filename}")


def parse_cli_args() -> None:
    parser = argparse.ArgumentParser(
        description="Script to plot discovered peering relationships and locations",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument(
        "--mirrored-output",
        help="Peering data with mirror values added in",
        type=str,
        default=MIRRORED_DATA,
    )
    parser.add_argument(
        "--peering-data",
        help="Peering data extracted from MRT files",
        type=str,
        default=PEERING_DATA,
    )
    parser.add_argument(
        "--peering-output",
        help="Generated table of discovered peerings",
        type=str,
        default=PEERING_OUTPUT,
    )

    global cli_args
    cli_args = parser.parse_args()


def main() -> None:
    parse_cli_args()
    peering_data = load_json(cli_args.peering_data)
    mirrored_data = mirror_data(peering_data)
    write_json(mirrored_data, cli_args.mirrored_output)
    plot_peerings(mirrored_data)


if __name__ == "__main__":
    main()
