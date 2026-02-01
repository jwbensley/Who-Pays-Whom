#!/usr/bin/env python3


import argparse
from enum import Enum
import json
import os
from typing import Any
import plotly.graph_objects as go
import plotly.offline as po

BASE_DIR = "./results"
PEERING_DATA = os.path.join(BASE_DIR, "peering_data.json")
PEERING_OUTPUT = os.path.join(BASE_DIR, "peerings.html")
T1_ASNS = [
    174,
    701,
    1273,
    1299,
    2914,
    3257,
    3320,
    3356,
    3491,
    5511,
    6453,
    6461,
    6762,
    6830,
    6939,
    7018,
    12956,
]


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


def load_peerings(filename: str) -> dict[str, Any]:
    data: dict[str, dict[str, Any]] = json.load(open(filename))
    return data["peering_data"]


def plot_peerings(data: dict[str, Any]) -> None:
    text_colour_map = {
        "no_data": "black",
        "peer": "darkgreen",
        "non_peer": "firebrick",
    }
    fill_colour_map = {
        "no_data": "white",
        "peer": "#f8e0e0",
        "non_peer": "lightgoldenrodyellow",
    }

    fill_colours: dict[int, list[list[str]]] = {
        t1_asn: [
            [fill_colour_map["no_data"] for _ in T1_ASNS]
            for _ in PeerLocations
        ]
        for t1_asn in T1_ASNS
    }

    text_colours: dict[int, list[list[str]]] = {
        t1_asn: [
            [text_colour_map["no_data"] for _ in T1_ASNS]
            for _ in PeerLocations
        ]
        for t1_asn in T1_ASNS
    }

    col_headings = ["<b>ASN</b>"] + [
        f"<b>{location.value}</b>" for location in PeerLocations
    ]

    # Values are by column
    all_data: dict[int, list[list[str]]] = {
        t1_asn: [[]] + [[] for _ in PeerLocations] for t1_asn in T1_ASNS
    }

    for t1_asn in T1_ASNS:

        for asn_index, local_asn in enumerate(T1_ASNS):
            all_data[t1_asn][0].append(f"<b>{local_asn}</b>")

            if t1_asn == local_asn:
                for i in range(1, len(all_data[t1_asn])):
                    all_data[t1_asn][i].append("-")
                continue

            if (
                str(t1_asn) not in data
                or str(local_asn) not in data[str(t1_asn)]["peers"]
            ):
                for i in range(1, len(all_data[t1_asn])):
                    all_data[t1_asn][i].append("")
                continue

            locations: dict[str, dict[str, dict[str, Any]]] = data[
                str(t1_asn)
            ]["peers"][str(local_asn)]["location_peerings"]
            for loc_index, location in enumerate(PeerLocations):
                if not location.name in locations:
                    all_data[t1_asn][loc_index + 1].append("")
                    continue

                peer_types = set(
                    list(locations[location.name]["peerings_in_loc"].keys())
                )
                all_data[t1_asn][loc_index + 1].append(f"{peer_types}")

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
                    values=all_data[T1_ASNS[0]],
                    line_color="gainsboro",
                    # fill_color=fill_colours[T1_ASNS[0]],
                    align="center",
                    font=dict(color=text_colours[T1_ASNS[0]], size=12),
                ),
            )
        ]
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
                                        values=all_data[t1_asn],
                                        # fill_color=fill_colours[t1_asn],
                                        font=dict(
                                            color=text_colours[t1_asn],
                                            size=12,
                                        ),
                                    )
                                )
                            ],
                        )
                        for t1_asn in T1_ASNS
                    ]
                ),
            )
        ]
    )

    fig.update_layout(
        title_text="Peering Matrix",
        title_x=0.5,
        margin=dict(l=0, r=0, b=0, t=100, pad=0),
    )
    po.plot(
        fig,
        filename=PEERING_OUTPUT,
        auto_open=False,
    )


def parse_cli_args() -> None:
    parser = argparse.ArgumentParser(
        description="Script to plot discovered peering relationships and locations",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
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
    plot_peerings(load_peerings(cli_args.peering_data))


if __name__ == "__main__":
    main()
